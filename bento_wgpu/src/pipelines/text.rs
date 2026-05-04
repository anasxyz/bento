use bytemuck::{Pod, Zeroable};
use cosmic_text::{CacheKey, FontSystem, SwashCache};
use etagere::{Allocation, AtlasAllocator, size2};
use std::collections::HashMap;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GlyphInstance {
    pub position: [f32; 2], // glyph position relative to text origin
    pub origin: [f32; 2],   // text origin in physical pixels
    pub size: [f32; 2],
    pub uv: [f32; 2],
    pub uv_size: [f32; 2],
    pub color: [f32; 4],
    pub transform: [f32; 4],
    pub is_color: u32,
    pub _pad: [u32; 3],
}

struct TextSlot {
    text: String,
    x: f32,
    y: f32,
    size: f32,
    color: [f32; 4],
    rotate: f32,
    scale_x: f32,
    scale_y: f32,
    cached_instances: Vec<GlyphInstance>,
}

impl TextSlot {
    fn matches(
        &self,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        color: [f32; 4],
        rotate: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> bool {
        self.text == text
            && self.x == x
            && self.y == y
            && self.size == size
            && self.color == color
            && self.rotate == rotate
            && self.scale_x == scale_x
            && self.scale_y == scale_y
    }
}

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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
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
            SwashContent::Color => image.data.to_vec(),
            SwashContent::Mask | SwashContent::SubpixelMask => {
                image.data.iter().flat_map(|&a| [a, a, a, a]).collect()
            }
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

pub struct TextPipeline {
    pub atlas: GlyphAtlas,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    screen_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    capacity: usize,
    count: u32,
    ranges: Vec<(u32, u32)>,
    slots: Vec<TextSlot>,
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

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glyph sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text screen uniform"),
            size: 8,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &screen_buffer,
            0,
            bytemuck::cast_slice(&[screen_w * scale, screen_h * scale]),
        );

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text bgl"),
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text bind group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: screen_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&atlas.view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/text.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GlyphInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                }, // position
                wgpu::VertexAttribute {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }, // origin
                wgpu::VertexAttribute {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                }, // size
                wgpu::VertexAttribute {
                    offset: 24,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x2,
                }, // uv
                wgpu::VertexAttribute {
                    offset: 32,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x2,
                }, // uv_size
                wgpu::VertexAttribute {
                    offset: 40,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                }, // color
                wgpu::VertexAttribute {
                    offset: 56,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                }, // transform
                wgpu::VertexAttribute {
                    offset: 72,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Uint32,
                }, // is_color
            ],
        };

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vertex_layout],
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
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let capacity = 1024;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("glyph vertex buffer"),
            size: (capacity * std::mem::size_of::<GlyphInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            atlas,
            pipeline,
            vertex_buffer,
            screen_buffer,
            bind_group,
            bind_group_layout,
            sampler,
            capacity,
            count: 0,
            ranges: Vec::new(),
            slots: Vec::new(),
            scale: 1.0,
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32, scale: f32) {
        self.scale = scale;
        queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::cast_slice(&[width * scale, height * scale]),
        );
        self.slots.clear();
    }

    pub fn prepare(
        &mut self,
        texts: &[(&str, f32, f32, f32, [f32; 4], f32, f32, f32)],
        font_system: &mut cosmic_text::FontSystem,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        use cosmic_text::{Attrs, Buffer, Metrics, Shaping};

        while self.slots.len() < texts.len() {
            self.slots.push(TextSlot {
                text: String::new(),
                x: f32::NAN,
                y: f32::NAN,
                size: f32::NAN,
                color: [f32::NAN; 4],
                rotate: f32::NAN,
                scale_x: f32::NAN,
                scale_y: f32::NAN,
                cached_instances: Vec::new(),
            });
        }

        let mut instances: Vec<GlyphInstance> = Vec::new();
        let mut any_changed = false;
        self.ranges.clear();

        for (i, &(text, x, y, size, color, rotate, scale_x, scale_y)) in texts.iter().enumerate() {
            let slot = &mut self.slots[i];

            if !slot.matches(text, x, y, size, color, rotate, scale_x, scale_y) {
                any_changed = true;
                slot.cached_instances.clear();

                let mut buffer = Buffer::new(font_system, Metrics::new(size, size * 1.4));
                buffer.set_size(font_system, None, None);
                buffer.set_text(font_system, text, &Attrs::new(), Shaping::Advanced, None);
                buffer.shape_until_scroll(font_system, false);

                let cos_r = rotate.cos();
                let sin_r = rotate.sin();
                let transform = [
                    cos_r * scale_x,
                    sin_r * scale_x,
                    -sin_r * scale_y,
                    cos_r * scale_y,
                ];

                for run in buffer.layout_runs() {
                    for glyph in run.glyphs {
                        let physical =
                            glyph.physical((0.0, 0.0), self.scale * scale_x.max(scale_y));

                        let Some(entry) = self.atlas.get_or_insert(
                            physical.cache_key,
                            font_system,
                            device,
                            queue,
                        ) else {
                            continue;
                        };

                        let raster_scale = self.scale * scale_x.max(scale_y);
                        let origin_x = (x * self.scale).round();
                        let origin_y = (y * self.scale).round();
                        let gx = (physical.x as f32 + entry.left as f32) / scale_x.max(scale_y);
                        let gy = ((run.line_y * raster_scale).round() + physical.y as f32
                            - entry.top as f32)
                            / scale_x.max(scale_y);

                        let u0 = entry.x as f32 / ATLAS_SIZE as f32;
                        let v0 = entry.y as f32 / ATLAS_SIZE as f32;
                        let uw = entry.w as f32 / ATLAS_SIZE as f32;
                        let vh = entry.h as f32 / ATLAS_SIZE as f32;

                        slot.cached_instances.push(GlyphInstance {
                            position: [gx, gy],
                            origin: [origin_x, origin_y],
                            size: [
                                entry.w as f32 / scale_x.max(scale_y),
                                entry.h as f32 / scale_x.max(scale_y),
                            ],
                            uv: [u0, v0],
                            uv_size: [uw, vh],
                            color,
                            transform,
                            is_color: entry.is_color as u32,
                            _pad: [0; 3],
                        });
                    }
                }

                slot.text = text.to_string();
                slot.x = x;
                slot.y = y;
                slot.size = size;
                slot.color = color;
                slot.rotate = rotate;
                slot.scale_x = scale_x;
                slot.scale_y = scale_y;
            }

            let start = instances.len() as u32;
            instances.extend_from_slice(&slot.cached_instances);
            self.ranges
                .push((start, slot.cached_instances.len() as u32));
        }

        self.count = instances.len() as u32;
        if instances.is_empty() {
            return;
        }
        if !any_changed {
            return;
        }

        if instances.len() > self.capacity {
            self.capacity = instances.len().next_power_of_two();
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("glyph vertex buffer"),
                size: (self.capacity * std::mem::size_of::<GlyphInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&instances));
        self.count = instances.len() as u32;
    }

    pub fn draw_range<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>, index: usize) {
        let Some(&(start, count)) = self.ranges.get(index) else {
            return;
        };
        if count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, start..start + count);
    }

    pub fn draw<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>) {
        if self.count == 0 {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, 0..self.count);
    }
}
