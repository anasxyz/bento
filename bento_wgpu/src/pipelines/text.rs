use bytemuck::{Pod, Zeroable};
use cosmic_text::{
    Attrs, Buffer, CacheKey, Family, FontSystem, Metrics, Shaping, Style as CStyle, SwashCache,
    SwashContent, Weight,
};
use etagere::{Allocation, AtlasAllocator, size2};
use std::collections::HashMap;
use wgpu::util::DeviceExt;

use crate::scene::{Mat2x3, mat_mul};

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
    generation: u64,
}

impl GlyphAtlas {
    fn new(device: &wgpu::Device) -> Self {
        let texture = Self::make_texture(device);
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        Self {
            texture,
            view,
            packer,
            entries: HashMap::new(),
            dirty: false,
            was_cleared: false,
            generation: 0,
        }
    }

    fn make_texture(device: &wgpu::Device) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
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
        })
    }

    pub fn clear(&mut self, device: &wgpu::Device) {
        self.packer = AtlasAllocator::new(size2(ATLAS_SIZE as i32, ATLAS_SIZE as i32));
        self.entries.clear();
        self.texture = Self::make_texture(device);
        self.view = self
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        self.dirty = true;
        self.was_cleared = true;
        self.generation += 1;
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

// Per-glyph instance sent to the GPU.
//
// col01 + trans encode the 2x3 affine transform for this glyph quad:
//   screen_pos = col01.xy * local.x + col01.zw * local.y + trans.xy
//
// trans.zw = physical glyph size (w, h), used to scale the unit quad.
// The transform already encodes: text_node_transform * T(glyph_local_offset).
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct GlyphInstance {
    col01: [f32; 4], // a, b, c, d
    trans: [f32; 4], // tx, ty, glyph_w, glyph_h  (physical pixels)
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
    size: f32,      // logical size
    phys_size: f32, // size * scale at time of shaping
    weight: u16,
    italic: bool,
    width: f32,
}

// Everything needed to re-check the cache and re-emit instances
#[derive(Clone)]
struct SubmitMeta {
    // The full physical-space transform for the text origin.
    // Stored as [f32; 6] so we can compare it cheaply.
    transform: [f32; 6],
    color: [f32; 4],
    clip: Option<[f32; 4]>,
}

impl SubmitMeta {
    fn matches(&self, other: &SubmitMeta) -> bool {
        self.transform == other.transform && self.color == other.color && self.clip == other.clip
    }
}

struct CachedInstances {
    instances: Vec<GlyphInstance>,
    meta: SubmitMeta,
    atlas_generation: u64,
}

impl CachedInstances {
    fn is_valid(&self, meta: &SubmitMeta, atlas_generation: u64) -> bool {
        self.atlas_generation == atlas_generation && self.meta.matches(meta)
    }
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
    cache: Vec<CachedInstances>,
    instances_dirty: bool,
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
            // Linear filtering is required for rotated/scaled text — Nearest causes
            // jagged aliased edges at non-axis-aligned angles. For axis-aligned text
            // the result is identical since sampling lands exactly on texel centres.
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
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

        // Instance attribute layout matching GlyphInstance struct:
        //   offset  0: col01  Float32x4
        //   offset 16: trans  Float32x4
        //   offset 32: uv     Float32x2
        //   offset 40: uv_sz  Float32x2
        //   offset 48: color  Float32x4
        //   offset 64: clip   Float32x4
        //   offset 80: flags  Uint32
        let inst_attrs = &[
            wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: 16,
                shader_location: 1,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: 32,
                shader_location: 2,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: 40,
                shader_location: 3,
                format: wgpu::VertexFormat::Float32x2,
            },
            wgpu::VertexAttribute {
                offset: 48,
                shader_location: 4,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: 64,
                shader_location: 5,
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: 80,
                shader_location: 6,
                format: wgpu::VertexFormat::Uint32,
            },
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
                    attributes: inst_attrs,
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
            cache: Vec::new(),
            instances_dirty: false,
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
            self.cache.clear();
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
        self.instances_dirty = false;
    }

    pub fn end_frame(&mut self) {
        self.entries.truncate(self.active);
        self.meta.truncate(self.active);
        self.cache.truncate(self.active);
    }

    // Submit a text run for rendering.
    //
    // `transform` is the full accumulated 2x3 affine matrix for this text node
    // in LOGICAL coordinates. The pipeline scales it to physical pixels internally.
    // For plain axis-aligned text this is just [1,0,0,1, x, y].
    pub fn submit(
        &mut self,
        font_system: &mut FontSystem,
        transform: Mat2x3,
        content: &str,
        family: &str,
        size: f32,
        weight: u16,
        italic: bool,
        color: [f32; 4],
        width: f32,
        clip: Option<[f32; 4]>,
    ) {
        // Shape at physical pixels so glyph.physical((0,0), 1.0) gives the
        // correct cache key and the layout positions are already in physical px.
        let scale = self.scale;
        let phys_size = size * scale;
        let line_height = phys_size * 1.4;
        let phys_width = if width >= f32::MAX {
            None
        } else {
            Some(width * scale)
        };

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

        let needs_reshape;
        if idx < self.entries.len() {
            let e = &mut self.entries[idx];
            // Also reshape when scale changed (monitor DPI switch)
            needs_reshape = e.text != content
                || e.family != family
                || e.size != size
                || e.phys_size != phys_size
                || e.weight != weight
                || e.italic != italic
                || e.width != width;
            if needs_reshape {
                e.text.clear();
                e.text.push_str(content);
                e.family.clear();
                e.family.push_str(family);
                e.size = size;
                e.phys_size = phys_size;
                e.weight = weight;
                e.italic = italic;
                e.width = width;
                e.buffer
                    .set_metrics(font_system, Metrics::new(phys_size, line_height));
                e.buffer.set_size(font_system, phys_width, None);
                e.buffer
                    .set_text(font_system, content, &attrs, Shaping::Advanced, None);
                e.buffer.shape_until_scroll(font_system, false);
            }
        } else {
            needs_reshape = true;
            let mut buf = Buffer::new(font_system, Metrics::new(phys_size, line_height));
            buf.set_size(font_system, phys_width, None);
            buf.set_text(font_system, content, &attrs, Shaping::Advanced, None);
            buf.shape_until_scroll(font_system, false);
            self.entries.push(BufferEntry {
                buffer: buf,
                text: content.to_string(),
                family: family.to_string(),
                size,
                phys_size,
                weight,
                italic,
                width,
            });
        }

        let m = SubmitMeta {
            transform,
            color,
            clip,
        };
        if idx < self.meta.len() {
            self.meta[idx] = m;
        } else {
            self.meta.push(m);
        }

        if needs_reshape && idx < self.cache.len() {
            self.cache[idx].atlas_generation = u64::MAX;
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
        let atlas_gen = self.atlas.generation;

        for idx in 0..self.active {
            let meta = self.meta[idx].clone();

            if idx >= self.cache.len() {
                self.cache.push(CachedInstances {
                    instances: Vec::new(),
                    meta: SubmitMeta {
                        transform: [f32::NAN; 6],
                        color: [f32::NAN; 4],
                        clip: None,
                    },
                    atlas_generation: u64::MAX,
                });
            }

            let cache_valid = self.cache[idx].is_valid(&meta, atlas_gen);

            if !cache_valid {
                let entry = &self.entries[idx];
                let mut new_instances: Vec<GlyphInstance> = Vec::new();

                // Build the physical-space transform for glyph placement.
                //
                // The logical transform's 2x2 part (rotation + user scale) is
                // dimensionless — it just rotates/scales directions and should
                // be applied unchanged to the glyph's physical local offsets.
                //
                // Only the translation needs converting from logical → physical px.
                //
                // Why: glyph local offsets (lx, ly) are in physical pixels because
                // the buffer was shaped at (size * dpi_scale). Applying the 2x2 to
                // physical offsets gives physical screen coordinates. We then add
                // the physical translation to get the final screen position.
                let t = meta.transform;
                let phys_transform: Mat2x3 = [
                    t[0],
                    t[1], // 2x2 rotation+user-scale: unchanged
                    t[2],
                    t[3],
                    t[4] * scale, // translation: logical px → physical px
                    t[5] * scale,
                ];

                let clip_phys = match meta.clip {
                    Some([cx, cy, cx2, cy2]) => [cx * scale, cy * scale, cx2 * scale, cy2 * scale],
                    None => [0.0f32; 4],
                };

                for run in entry.buffer.layout_runs() {
                    for glyph in run.glyphs.iter() {
                        // Buffer was shaped at physical size, so glyph.physical with
                        // scale=1.0 gives the correct cache key and pixel positions
                        // directly — no additional scaling needed.
                        let physical = glyph.physical((0.0, 0.0), 1.0);
                        let cache_key = physical.cache_key;

                        // Glyph's offset within the text block — already in physical px
                        // because the buffer was shaped at physical size.
                        let glyph_local_x = physical.x as f32;
                        let glyph_local_y = run.line_y.round() + physical.y as f32;

                        // Early clip test — both glyph positions and clip are in physical px
                        if let Some([cx, cy, cx2, cy2]) = meta.clip {
                            let pcx = cx * scale;
                            let pcy = cy * scale;
                            let pcx2 = cx2 * scale;
                            let pcy2 = cy2 * scale;
                            let sx = phys_transform[0] * glyph_local_x
                                + phys_transform[2] * glyph_local_y
                                + phys_transform[4];
                            let sy = phys_transform[1] * glyph_local_x
                                + phys_transform[3] * glyph_local_y
                                + phys_transform[5];
                            if sx >= pcx2 || sy >= pcy2 + entry.phys_size {
                                continue;
                            }
                            if sx + glyph.w <= pcx {
                                continue;
                            }
                            if sy + entry.phys_size <= pcy {
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

                        // Final glyph top-left in local text space (physical px):
                        //   glyph_local + bearing offset from atlas
                        let lx = glyph_local_x + ae.left as f32;
                        let ly = glyph_local_y - ae.top as f32;

                        // Compose: phys_transform * T(lx, ly)
                        // T(lx, ly) = [1,0,0,1,lx,ly]
                        let glyph_transform = mat_mul(phys_transform, [1.0, 0.0, 0.0, 1.0, lx, ly]);

                        // Quick screen-space bounds check
                        let (sx, sy) = (glyph_transform[4], glyph_transform[5]);
                        if (sx + ae.w as f32) < 0.0 || sx > phys_w {
                            continue;
                        }
                        if (sy + ae.h as f32) < 0.0 || sy > phys_h {
                            continue;
                        }

                        let u0 = ae.x as f32 / ATLAS_SIZE as f32;
                        let v0 = ae.y as f32 / ATLAS_SIZE as f32;
                        let uw = ae.w as f32 / ATLAS_SIZE as f32;
                        let vh = ae.h as f32 / ATLAS_SIZE as f32;

                        new_instances.push(GlyphInstance {
                            col01: [
                                glyph_transform[0],
                                glyph_transform[1],
                                glyph_transform[2],
                                glyph_transform[3],
                            ],
                            trans: [
                                glyph_transform[4],
                                glyph_transform[5],
                                ae.w as f32,
                                ae.h as f32,
                            ],
                            uv: [u0, v0],
                            uv_sz: [uw, vh],
                            color: meta.color,
                            clip: clip_phys,
                            flags: if ae.is_color { 1 } else { 0 },
                            _pad: [0; 3],
                        });
                    }
                }

                self.cache[idx] = CachedInstances {
                    instances: new_instances,
                    meta,
                    atlas_generation: atlas_gen,
                };
                self.instances_dirty = true;
            }

            let start = self.instances.len() as u32;
            self.instances.extend_from_slice(&self.cache[idx].instances);
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
            queue.write_buffer(&self.vertex_buf, 0, bytemuck::cast_slice(&self.instances));
        } else if self.instances_dirty {
            queue.write_buffer(&self.vertex_buf, 0, bytemuck::cast_slice(&self.instances));
        }
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

    pub fn instance_range(&self, idx: usize) -> (u32, u32) {
        self.ranges.get(idx).copied().unwrap_or((0, 0))
    }

    pub fn trim_atlas(&mut self) {}

    // Returns (rect_x, rect_y, rect_w, rect_h, transform) in LOGICAL coordinates.
    // Buffer is shaped at physical size so all glyph positions are physical —
    // we divide by scale to return logical values for write_slot.
    pub fn compute_decoration_rects(
        &self,
        idx: usize,
        start: usize,
        end: usize,
        thickness: f32,
        decoration_type: DecorationKind,
        scale: f32,
    ) -> Vec<(f32, f32, f32, f32, Mat2x3)> {
        if idx >= self.active || start >= end {
            return Vec::new();
        }
        let entry = &self.entries[idx];
        let meta = &self.meta[idx];
        let mut rects = Vec::new();
        // thickness is logical; round to nearest physical pixel then back to logical
        let thick = (thickness * scale).round().max(1.0) / scale;

        for run in entry.buffer.layout_runs() {
            let mut x1: Option<f32> = None;
            let mut x2: Option<f32> = None;
            for glyph in run.glyphs {
                if glyph.end <= start || glyph.start >= end {
                    continue;
                }
                // glyph.x is physical — convert to logical
                let gx1 = glyph.x / scale;
                let gx2 = gx1 + glyph.w / scale;
                x1 = Some(x1.map_or(gx1, |v: f32| v.min(gx1)));
                x2 = Some(x2.map_or(gx2, |v: f32| v.max(gx2)));
            }
            if let (Some(rx1), Some(rx2)) = (x1, x2) {
                if rx2 <= rx1 {
                    continue;
                }
                // Snap to physical pixel grid then back to logical
                let px1 = (rx1 * scale).round() / scale;
                let px2 = (rx2 * scale).round() / scale;
                let line_y = match decoration_type {
                    DecorationKind::Underline => {
                        // run.line_y is physical baseline
                        let baseline = run.line_y / scale;
                        ((baseline + 1.0 / scale) * scale).round() / scale
                    }
                    DecorationKind::Strikethrough => {
                        let baseline = run.line_y / scale;
                        let ascent = (run.line_y - run.line_top) / scale;
                        ((baseline - ascent * 0.4) * scale).round() / scale
                    }
                };
                rects.push((px1, line_y, px2 - px1, thick, meta.transform));
            }
        }
        rects
    }

    pub fn compute_selection_rects(
        &self,
        idx: usize,
        sel_start: usize,
        sel_end: usize,
        scale: f32,
    ) -> Vec<(f32, f32, f32, f32, Mat2x3)> {
        if idx >= self.active || sel_start >= sel_end {
            return Vec::new();
        }
        let entry = &self.entries[idx];
        let meta = &self.meta[idx];
        let mut rects = Vec::new();
        // line_height in logical pixels
        let line_height = entry.size * 1.4;

        for run in entry.buffer.layout_runs() {
            // run.line_top is physical — convert to logical
            let run_y = run.line_top / scale;
            let mut x1: Option<f32> = None;
            let mut x2: Option<f32> = None;
            for glyph in run.glyphs {
                if glyph.end <= sel_start || glyph.start >= sel_end {
                    continue;
                }
                let gx = glyph.x / scale;
                let gx2 = gx + glyph.w / scale + 1.0 / scale;
                x1 = Some(x1.map_or(gx, |v: f32| v.min(gx)));
                x2 = Some(x2.map_or(gx2, |v: f32| v.max(gx2)));
            }
            if let (Some(rx1), Some(rx2)) = (x1, x2) {
                rects.push((rx1, run_y, rx2 - rx1, line_height, meta.transform));
            }
        }
        rects
    }
}
