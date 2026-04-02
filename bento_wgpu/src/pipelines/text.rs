use bytemuck::{Pod, Zeroable};
use cosmic_text::{
    Attrs, Buffer, CacheKey, Family, FontSystem, Metrics, Shaping, Style as CStyle, SwashCache,
    SwashContent, Weight,
};
use etagere::{Allocation, AtlasAllocator, size2};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

pub enum DecorationKind {
    Underline,
    Strikethrough,
}

const ATLAS_SIZE: u32 = 2048;

struct AtlasEntry {
    #[allow(dead_code)]
    alloc: Allocation,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    left: i32,
    top: i32,
    is_color: bool,
}

struct GlyphAtlas {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    packer: AtlasAllocator,
    entries: HashMap<CacheKey, AtlasEntry>,
    dirty: bool,
    was_cleared: bool,
}

impl GlyphAtlas {
    fn new(device: &wgpu::Device) -> Self {
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        Self {
            texture,
            view,
            packer,
            entries: HashMap::new(),
            dirty: false,
            was_cleared: false,
        }
    }

    pub fn clear(&mut self, device: &wgpu::Device) {
        self.packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        self.entries.clear();
        self.texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("glyph atlas"),
            size: wgpu::Extent3d {
                width: ATLAS_SIZE,
                height: ATLAS_SIZE,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        self.view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.dirty = true;
        self.was_cleared = true;
    }

    fn get_or_insert(
        &mut self,
        cache_key: CacheKey,
        font_system: &mut FontSystem,
        swash_cache: &mut SwashCache,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
    ) -> Option<&AtlasEntry> {
        if self.entries.contains_key(&cache_key) {
            return self.entries.get(&cache_key);
        }

        let image = swash_cache.get_image_uncached(font_system, cache_key)?;
        let w = image.placement.width;
        let h = image.placement.height;
        if w == 0 || h == 0 {
            return None;
        }

        let is_color = matches!(image.content, SwashContent::Color);

        let alloc = match self.packer.allocate(size2(w as i32 + 1, h as i32 + 1)) {
            Some(a) => a,
            None => {
                self.clear(device);
                self.packer.allocate(size2(w as i32 + 1, h as i32 + 1))?
            }
        };
        let ax = alloc.rectangle.min.x as u32;
        let ay = alloc.rectangle.min.y as u32;

        let rgba: Vec<u8> = match image.content {
            SwashContent::Color => image.data.to_vec(),
            SwashContent::Mask | SwashContent::SubpixelMask => {
                // fill all channels with the mask value for consistency
                image.data.iter().flat_map(|&a| [a, a, a, a]).collect()
            }
        };

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d { x: ax, y: ay, z: 0 },
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

        self.entries.insert(
            cache_key,
            AtlasEntry {
                alloc,
                x: ax,
                y: ay,
                w,
                h,
                left: image.placement.left,
                top: image.placement.top,
                is_color,
            },
        );
        self.dirty = true;

        self.entries.get(&cache_key)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct GlyphInstance {
    pos: [f32; 2],
    size: [f32; 2],
    uv: [f32; 2],
    uv_sz: [f32; 2],
    color: [f32; 4],
    clip: [f32; 4],
    flags: u32,
    _pad: [u32; 3],
}

struct BufferEntry {
    buffer: Buffer,
    text: String,
    family: String,
    size: f32,
    weight: u16,
    italic: bool,
    width: f32,
}

#[derive(Clone)]
struct SubmitMeta {
    x: f32,
    y: f32,
    color: [f32; 4],
    clip: Option<[f32; 4]>,
}

pub struct TextPipeline {
    swash_cache: SwashCache,
    atlas: GlyphAtlas,
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    screen_buf: wgpu::Buffer,
    sampler: wgpu::Sampler,
    instances: Vec<GlyphInstance>,
    ranges: Vec<(u32, u32)>,
    vertex_buf: wgpu::Buffer,
    vertex_buf_cap: usize,
    entries: Vec<BufferEntry>,
    meta: Vec<SubmitMeta>,
    active: usize,
    screen_w: f32,
    screen_h: f32,
    scale: f32,
}

impl TextPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
        scale: f32,
    ) -> Self {
        let atlas = GlyphAtlas::new(device);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("glyph shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into()),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glyph sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let phys = [screen_w * scale, screen_h * scale];
        let screen_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("glyph screen uniform"),
            contents: bytemuck::cast_slice(&phys),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("glyph bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let bind_group = Self::make_bind_group(
            device,
            &bind_group_layout,
            &screen_buf,
            &atlas.view,
            &sampler,
        );

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("glyph pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let inst_attrs = wgpu::vertex_attr_array![
            0 => Float32x2,
            1 => Float32x2,
            2 => Float32x2,
            3 => Float32x2,
            4 => Float32x4,
            5 => Float32x4,
            6 => Uint32,
        ];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("glyph pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GlyphInstance>() as u64,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &inst_attrs,
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let initial_cap = 256;
        let vertex_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("glyph vb"),
            size: (initial_cap * std::mem::size_of::<GlyphInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            swash_cache: SwashCache::new(),
            atlas,
            pipeline,
            bind_group_layout,
            bind_group,
            screen_buf,
            sampler,
            instances: Vec::new(),
            ranges: Vec::new(),
            vertex_buf,
            vertex_buf_cap: initial_cap,
            entries: Vec::new(),
            meta: Vec::new(),
            active: 0,
            screen_w,
            screen_h,
            scale,
        }
    }

    fn make_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        screen_buf: &wgpu::Buffer,
        atlas_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glyph bind group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(atlas_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(sampler),
                },
            ],
        })
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        screen_w: f32,
        screen_h: f32,
        scale: f32,
    ) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        if self.scale != scale {
            self.scale = scale;
            self.swash_cache = SwashCache::new();
            self.atlas.clear(device);
        } else {
            self.scale = scale;
        }
        let phys = [screen_w * scale, screen_h * scale];
        queue.write_buffer(&self.screen_buf, 0, bytemuck::cast_slice(&phys));
        self.atlas.dirty = true;
    }

    pub fn begin_frame(&mut self) {
        self.active = 0;
        self.instances.clear();
        self.ranges.clear();
    }

    pub fn end_frame(&mut self) {
        self.entries.truncate(self.active);
        self.meta.truncate(self.active);
    }

    pub fn submit(
        &mut self,
        font_system: &mut FontSystem,
        x: f32,
        y: f32,
        content: &str,
        family: &str,
        size: f32,
        weight: u16,
        italic: bool,
        color: [f32; 4],
        width: f32,
        clip: Option<[f32; 4]>,
    ) {
        let line_height = size * 1.4;
        let idx = self.active;
        self.active += 1;

        let attrs = Attrs::new()
            .family(Family::Name(family))
            .weight(Weight(weight))
            .style(if italic {
                CStyle::Italic
            } else {
                CStyle::Normal
            });

        if idx < self.entries.len() {
            let e = &mut self.entries[idx];
            let needs_reshape = e.text != content
                || e.family != family
                || e.size != size
                || e.weight != weight
                || e.italic != italic
                || e.width != width;
            if needs_reshape {
                e.text.clear();
                e.text.push_str(content);
                e.family.clear();
                e.family.push_str(family);
                e.size = size;
                e.weight = weight;
                e.italic = italic;
                e.width = width;
                e.buffer
                    .set_metrics(font_system, Metrics::new(size, line_height));
                e.buffer.set_size(
                    font_system,
                    if width >= f32::MAX { None } else { Some(width) },
                    None,
                );
                e.buffer
                    .set_text(font_system, content, &attrs, Shaping::Advanced, None);
                e.buffer.shape_until_scroll(font_system, false);
            }
        } else {
            let mut buf = Buffer::new(font_system, Metrics::new(size, line_height));
            buf.set_size(
                font_system,
                if width >= f32::MAX { None } else { Some(width) },
                None,
            );
            buf.set_text(font_system, content, &attrs, Shaping::Advanced, None);
            buf.shape_until_scroll(font_system, false);
            self.entries.push(BufferEntry {
                buffer: buf,
                text: content.to_string(),
                family: family.to_string(),
                size,
                weight,
                italic,
                width,
            });
        }

        let m = SubmitMeta { x, y, color, clip };
        if idx < self.meta.len() {
            self.meta[idx] = m;
        } else {
            self.meta.push(m);
        }
    }

    pub fn prepare(
        &mut self,
        font_system: &mut FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        if self.active == 0 {
            return;
        }

        let scale = self.scale;
        let phys_w = self.screen_w * scale;
        let phys_h = self.screen_h * scale;

        for idx in 0..self.active {
            let meta = self.meta[idx].clone();
            let entry = &self.entries[idx];
            let start = self.instances.len() as u32;

            for run in entry.buffer.layout_runs() {
                for glyph in run.glyphs.iter() {
                    let physical = glyph.physical((0.0, 0.0), scale);
                    let cache_key = physical.cache_key;
                    let origin_x = (meta.x * scale).round() as i32;
                    let origin_y = (meta.y * scale).round() as i32;
                    let glyph_phys_x = origin_x + physical.x;
                    let glyph_phys_y = origin_y + physical.y;

                    if let Some([cx, cy, cx2, cy2]) = meta.clip {
                        let pcx = cx * scale;
                        let pcy = cy * scale;
                        let pcx2 = cx2 * scale;
                        let pcy2 = cy2 * scale;
                        let gx = glyph_phys_x as f32;
                        let gy = glyph_phys_y as f32 + (run.line_y * scale).round();
                        if gx >= pcx2 || gy >= pcy2 + entry.size * scale {
                            continue;
                        }
                        if gx + glyph.w * scale <= pcx {
                            continue;
                        }
                        if gy + entry.size * scale <= pcy {
                            continue;
                        }
                    }

                    let Some(ae) = self.atlas.get_or_insert(
                        cache_key,
                        font_system,
                        &mut self.swash_cache,
                        queue,
                        device,
                    ) else {
                        continue;
                    };

                    let px = glyph_phys_x as f32 + ae.left as f32;
                    let py = glyph_phys_y as f32 + (run.line_y * scale).round() - ae.top as f32;

                    if (px + ae.w as f32) < 0.0 || px > phys_w {
                        continue;
                    }
                    if (py + ae.h as f32) < 0.0 || py > phys_h {
                        continue;
                    }

                    let u0 = ae.x as f32 / ATLAS_SIZE as f32;
                    let v0 = ae.y as f32 / ATLAS_SIZE as f32;
                    let uw = ae.w as f32 / ATLAS_SIZE as f32;
                    let vh = ae.h as f32 / ATLAS_SIZE as f32;

                    let clip_phys = match meta.clip {
                        Some([cx, cy, cx2, cy2]) => {
                            [cx * scale, cy * scale, cx2 * scale, cy2 * scale]
                        }
                        None => [0.0f32; 4],
                    };

                    self.instances.push(GlyphInstance {
                        pos: [px, py],
                        size: [ae.w as f32, ae.h as f32],
                        uv: [u0, v0],
                        uv_sz: [uw, vh],
                        color: meta.color,
                        clip: clip_phys,
                        flags: if ae.is_color { 1 } else { 0 },
                        _pad: [0; 3],
                    });
                }
            }
            let count = self.instances.len() as u32 - start;
            self.ranges.push((start, count));
        }

        if self.instances.is_empty() {
            return;
        }

        if self.atlas.was_cleared {
            self.swash_cache = SwashCache::new();
            self.atlas.was_cleared = false;
        }

        if self.atlas.dirty {
            self.bind_group = Self::make_bind_group(
                device,
                &self.bind_group_layout,
                &self.screen_buf,
                &self.atlas.view,
                &self.sampler,
            );
            self.atlas.dirty = false;
        }

        if self.instances.len() > self.vertex_buf_cap {
            self.vertex_buf_cap = self.instances.len().next_power_of_two();
            self.vertex_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("glyph vb"),
                size: (self.vertex_buf_cap * std::mem::size_of::<GlyphInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(&self.vertex_buf, 0, bytemuck::cast_slice(&self.instances));
    }

    pub fn draw_range(&self, pass: &mut wgpu::RenderPass<'_>, start: u32, count: u32) {
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        pass.draw(0..6, start..start + count);
    }

    pub fn render(
        &mut self,
        font_system: &mut FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'_>,
    ) {
        self.prepare(font_system, device, queue);
        let count = self.instances.len() as u32;
        self.draw_range(pass, 0, count);
    }

    pub fn instance_range(&self, idx: usize) -> (u32, u32) {
        self.ranges.get(idx).copied().unwrap_or((0, 0))
    }

    pub fn trim_atlas(&mut self) {}

    pub fn compute_decoration_rects(
        &self,
        idx: usize,
        start: usize,
        end: usize,
        thickness: f32,
        decoration_type: DecorationKind,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
    ) -> Vec<(f32, f32, f32, f32)> {
        if idx >= self.active || start >= end {
            return Vec::new();
        }
        let entry = &self.entries[idx];
        let meta = &self.meta[idx];
        let mut rects = Vec::new();
        let thick = (thickness * scale).round().max(1.0) / scale;

        for run in entry.buffer.layout_runs() {
            let mut x1: Option<f32> = None;
            let mut x2: Option<f32> = None;
            for glyph in run.glyphs {
                if glyph.end <= start || glyph.start >= end {
                    continue;
                }
                let gx1 = offset_x + meta.x + glyph.x;
                let gx2 = gx1 + glyph.w;
                x1 = Some(x1.map_or(gx1, |v: f32| v.min(gx1)));
                x2 = Some(x2.map_or(gx2, |v: f32| v.max(gx2)));
            }
            if let (Some(rx1), Some(rx2)) = (x1, x2) {
                if rx2 <= rx1 {
                    continue;
                }
                let px1 = (rx1 * scale).round() / scale;
                let px2 = (rx2 * scale).round() / scale;
                let line_y = match decoration_type {
                    DecorationKind::Underline => {
                        let baseline = offset_y + meta.y + run.line_y;
                        ((baseline + 1.0 / scale) * scale).round() / scale
                    }
                    DecorationKind::Strikethrough => {
                        let baseline = offset_y + meta.y + run.line_y;
                        let ascent = run.line_y - run.line_top;
                        ((baseline - ascent * 0.4) * scale).round() / scale
                    }
                };
                rects.push((px1, line_y, px2 - px1, thick));
            }
        }
        rects
    }

    pub fn compute_selection_rects(
        &self,
        idx: usize,
        sel_start: usize,
        sel_end: usize,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
    ) -> Vec<(f32, f32, f32, f32)> {
        if idx >= self.active || sel_start >= sel_end {
            return Vec::new();
        }
        let entry = &self.entries[idx];
        let meta = &self.meta[idx];
        let mut rects = Vec::new();
        let line_height = entry.size * 1.4;

        for run in entry.buffer.layout_runs() {
            let run_y = offset_y + meta.y + run.line_top;
            let mut x1: Option<f32> = None;
            let mut x2: Option<f32> = None;
            for glyph in run.glyphs {
                if glyph.end <= sel_start || glyph.start >= sel_end {
                    continue;
                }
                let gx = offset_x + meta.x + glyph.x;
                let gx2 = gx + glyph.w + 1.0 / scale;
                x1 = Some(x1.map_or(gx, |v: f32| v.min(gx)));
                x2 = Some(x2.map_or(gx2, |v: f32| v.max(gx2)));
            }
            if let (Some(rx1), Some(rx2)) = (x1, x2) {
                rects.push((rx1, run_y, rx2 - rx1, line_height));
            }
        }
        rects
    }
}
