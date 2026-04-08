use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::mem;
use wgpu;

use crate::nodes::ImageKey;

struct CachedImage {
    #[allow(dead_code)]
    texture: wgpu::Texture,
    #[allow(dead_code)]
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

pub struct ImageCache {
    entries: HashMap<ImageKey, CachedImage>,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl ImageCache {
    pub fn new(device: &wgpu::Device) -> Self {
        let bind_group_layout = Self::make_bgl(device);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("image sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        Self {
            entries: HashMap::new(),
            bind_group_layout,
            sampler,
        }
    }

    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        key: ImageKey,
        rgba: &[u8],
        width: u32,
        height: u32,
    ) {
        if self.entries.contains_key(&key) {
            return;
        }
        assert_eq!(
            rgba.len() as u32,
            width * height * 4,
            "rgba length mismatch"
        );

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("image texture"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(width * 4),
                rows_per_image: Some(height),
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.entries.insert(
            key,
            CachedImage {
                texture,
                view,
                bind_group,
            },
        );
    }

    pub fn free(&mut self, key: ImageKey) {
        self.entries.remove(&key);
    }

    pub fn bind_group_of(&self, key: ImageKey) -> Option<&wgpu::BindGroup> {
        self.entries.get(&key).map(|e| &e.bind_group)
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    fn make_bgl(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Instance {
    pos_size: [f32; 4],
    uv: [f32; 4],
    tint: [f32; 4],
    clip: [f32; 4],
    params: [f32; 4],
}

const INSTANCE_SIZE: usize = mem::size_of::<Instance>();
const INITIAL_CAPACITY: usize = 64;

const INSTANCE_ATTRS: &[wgpu::VertexAttribute] = &[
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
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 48,
        shader_location: 3,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 64,
        shader_location: 4,
        format: wgpu::VertexFormat::Float32x4,
    },
];

pub struct ImageCall {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub uv: [f32; 4],
    pub tint: [f32; 4],
    pub radius: f32,
    pub image_key: ImageKey,
    pub clip: Option<[f32; 4]>,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct ScreenUniform {
    size: [f32; 2],
    _pad: [f32; 2],
}

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    screen_uniform: wgpu::Buffer,
    screen_bg: wgpu::BindGroup,
    instance_buffer: wgpu::Buffer,
    instance_cap: usize,
    staged: Vec<Instance>,
    screen_w: f32,
    screen_h: f32,
}

impl ImagePipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
        cache: &ImageCache,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bento_wgpu::image shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/image.wgsl").into()),
        });

        let screen_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("image screen uniform"),
            size: mem::size_of::<ScreenUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let screen_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("image screen bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let screen_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("image screen bg"),
            layout: &screen_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_uniform.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("image pipeline layout"),
            bind_group_layouts: &[&screen_bgl, cache.bind_group_layout()],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bento_wgpu::image pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: INSTANCE_SIZE as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: INSTANCE_ATTRS,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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

        let instance_buffer = Self::make_buffer(device, INITIAL_CAPACITY);

        let mut s = Self {
            pipeline,
            screen_uniform,
            screen_bg,
            instance_buffer,
            instance_cap: INITIAL_CAPACITY,
            staged: Vec::new(),
            screen_w,
            screen_h,
        };
        s.write_screen(queue);
        s
    }

    pub fn begin_frame(&mut self) {
        self.staged.clear();
    }

    pub fn prepare_layer(&mut self, calls: &[ImageCall], scale: f32) -> Vec<(u32, u32, ImageKey)> {
        if calls.is_empty() {
            return vec![];
        }

        let base = self.staged.len() as u32;

        for c in calls {
            let s = scale;
            let px = (c.x * s).round();
            let py = (c.y * s).round();
            let pw = ((c.x + c.w) * s).round() - px;
            let ph = ((c.y + c.h) * s).round() - py;
            let r = (c.radius * s).round().min(pw * 0.5).min(ph * 0.5);
            let clip_arr = match c.clip {
                Some([cx, cy, cx2, cy2]) => [
                    (cx * s).round(),
                    (cy * s).round(),
                    (cx2 * s).round(),
                    (cy2 * s).round(),
                ],
                None => [0.0; 4],
            };
            self.staged.push(Instance {
                pos_size: [px, py, pw, ph],
                uv: c.uv,
                tint: c.tint,
                clip: clip_arr,
                params: [r, 0.0, 0.0, 0.0],
            });
        }

        let mut ranges = Vec::new();
        let mut i = 0usize;
        while i < calls.len() {
            let key = calls[i].image_key;
            let start = base + i as u32;
            while i < calls.len() && calls[i].image_key == key {
                i += 1;
            }
            ranges.push((start, i as u32 - (start - base), key));
        }
        ranges
    }

    pub fn upload_staged(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.staged.is_empty() {
            return;
        }
        self.write_screen(queue);
        if self.staged.len() > self.instance_cap {
            let new_cap = self.staged.len().next_power_of_two();
            self.instance_buffer = Self::make_buffer(device, new_cap);
            self.instance_cap = new_cap;
        }
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.staged));
    }

    pub fn draw_layer<'pass>(
        &'pass self,
        pass: &mut wgpu::RenderPass<'pass>,
        cache: &'pass ImageCache,
        ranges: &[(u32, u32, ImageKey)],
    ) {
        if ranges.is_empty() || self.staged.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.screen_bg, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        for &(start, count, key) in ranges {
            if count == 0 {
                continue;
            }
            let Some(bg) = cache.bind_group_of(key) else {
                continue;
            };
            pass.set_bind_group(1, bg, &[]);
            pass.draw(0..6, start..start + count);
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, screen_w: f32, screen_h: f32) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        self.write_screen(queue);
    }

    fn write_screen(&self, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.screen_uniform,
            0,
            bytemuck::bytes_of(&ScreenUniform {
                size: [self.screen_w, self.screen_h],
                _pad: [0.0; 2],
            }),
        );
    }

    fn make_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bento_wgpu::image instances"),
            size: (capacity * INSTANCE_SIZE).max(INSTANCE_SIZE) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}
