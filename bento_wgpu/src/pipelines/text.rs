use cosmic_text::{CacheKey, FontSystem, SwashCache};
use etagere::{Allocation, AtlasAllocator, size2};
use std::collections::HashMap;
use wgpu;

const ATLAS_SIZE: u32 = 2048;

pub struct AtlasEntry {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub left: i32,
    pub top: i32,
    pub is_color: bool,
    allocation: Allocation,
}

pub struct GlyphAtlas {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub entries: HashMap<CacheKey, AtlasEntry>,
    packer: AtlasAllocator,
    pub swash: SwashCache,
}

impl GlyphAtlas {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        let swash = SwashCache::new();
        Self {
            texture,
            view,
            entries: HashMap::new(),
            packer,
            swash,
        }
    }

    pub fn clear(&mut self, device: &wgpu::Device) {
        self.packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        self.entries.clear();
        self.swash = SwashCache::new();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.texture = texture;
    }

    pub fn get_or_insert(
        &mut self,
        key: CacheKey,
        font_system: &mut FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Option<&AtlasEntry> {
        if self.entries.contains_key(&key) {
            return self.entries.get(&key);
        }

        // rasterise the glyph on CPU using swash
        let image = self.swash.get_image_uncached(font_system, key)?;

        let w = image.placement.width;
        let h = image.placement.height;
        if w == 0 || h == 0 {
            return None;
        }

        // allocate space in the atlas
        let alloc = match self.packer.allocate(size2(w as i32 + 1, h as i32 + 1)) {
            Some(a) => a,
            None => {
                self.clear(device);
                self.packer.allocate(size2(w as i32 + 1, h as i32 + 1))?
            }
        };

        let x = alloc.rectangle.min.x as u32;
        let y = alloc.rectangle.min.y as u32;

        // convert to rgba
        use cosmic_text::SwashContent;
        let rgba: Vec<u8> = match image.content {
            SwashContent::Mask => image.data.iter().flat_map(|&a| [a, a, a, a]).collect(),
            SwashContent::Color | SwashContent::SubpixelMask => image.data.to_vec(),
        };

        // upload to atlas texture
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x, y, z: 0 },
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: None,
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let is_color = matches!(image.content, SwashContent::Color);

        self.entries.insert(
            key,
            AtlasEntry {
                x,
                y,
                w,
                h,
                left: image.placement.left,
                top: image.placement.top,
                is_color,
                allocation: alloc,
            },
        );

        self.entries.get(&key)
    }
}
