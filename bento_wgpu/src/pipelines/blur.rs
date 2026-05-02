// Backdrop blur pipeline.
//
// Renders a frosted-glass blur over the framebuffer content behind each BlurNode.
// Uses a two-pass separable Gaussian blur (horizontal then vertical) sampled from
// a copy of the framebuffer taken before the blur pass begins.
//
// The blur region is masked to a rounded rect using an SDF in the shader.

use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct BlurInstance {
    pos_size: [f32; 4], // x, y, w, h  (physical pixels)
    params: [f32; 4],   // radius, sigma, pass (0=h, 1=v), _
    clip: [f32; 4],
    tint: [f32; 4],
}

const INSTANCE_SIZE: usize = mem::size_of::<BlurInstance>();

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct ScreenUniform {
    size: [f32; 2],
    _pad: [f32; 2],
}

pub struct BlurCall {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub radius: f32,
    pub sigma: f32,
    pub tint: [f32; 4],
    pub clip: Option<[f32; 4]>,
}

pub struct BlurPipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    screen_buffer: wgpu::Buffer,
    // The framebuffer copy texture used as blur source
    pub src_texture: Option<wgpu::Texture>,
    pub src_view: Option<wgpu::TextureView>,
    src_bind_group: Option<wgpu::BindGroup>,
    sampler: wgpu::Sampler,
    instance_buffer: wgpu::Buffer,
    instance_cap: usize,
    staged: Vec<BlurInstance>,
    screen_w: f32,
    screen_h: f32,
}

impl BlurPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bento_wgpu::blur shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/blur.wgsl").into()),
        });

        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("blur screen uniform"),
            size: mem::size_of::<ScreenUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &screen_buffer,
            0,
            bytemuck::bytes_of(&ScreenUniform {
                size: [screen_w, screen_h],
                _pad: [0.0; 2],
            }),
        );

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("blur sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("blur bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
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

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("blur pipeline layout"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        });

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
                format: wgpu::VertexFormat::Float32x4,
            },
            wgpu::VertexAttribute {
                offset: 48,
                shader_location: 3,
                format: wgpu::VertexFormat::Float32x4,
            },
        ];

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bento_wgpu::blur pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: INSTANCE_SIZE as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: inst_attrs,
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
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

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("blur instances"),
            size: (64 * INSTANCE_SIZE) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            bind_group_layout: bgl,
            screen_buffer,
            src_texture: None,
            src_view: None,
            src_bind_group: None,
            sampler,
            instance_buffer,
            instance_cap: 64,
            staged: Vec::new(),
            screen_w,
            screen_h,
        }
    }

    // Called when the framebuffer size changes — recreates the copy texture
    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        screen_w: f32,
        screen_h: f32,
        format: wgpu::TextureFormat,
    ) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::bytes_of(&ScreenUniform {
                size: [screen_w, screen_h],
                _pad: [0.0; 2],
            }),
        );
        self.recreate_src_texture(device, format);
    }

    fn recreate_src_texture(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        let w = self.screen_w as u32;
        let h = self.screen_h as u32;
        if w == 0 || h == 0 {
            return;
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("blur src texture"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("blur src bind group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.screen_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
        });
        self.src_texture = Some(texture);
        self.src_view = Some(view);
        self.src_bind_group = Some(bind_group);
    }

    pub fn ensure_src_texture(&mut self, device: &wgpu::Device, format: wgpu::TextureFormat) {
        if self.src_texture.is_none() {
            self.recreate_src_texture(device, format);
        }
    }

    pub fn begin_frame(&mut self) {
        self.staged.clear();
    }

    pub fn prepare(
        &mut self,
        calls: &[BlurCall],
        scale: f32,
        queue: &wgpu::Queue,
        device: &wgpu::Device,
    ) {
        if calls.is_empty() {
            return;
        }
        let s = scale;
        for c in calls {
            let clip_arr = match c.clip {
                Some([cx, cy, cx2, cy2]) => [cx * s, cy * s, cx2 * s, cy2 * s],
                None => [0.0; 4],
            };
            // Horizontal pass
            self.staged.push(BlurInstance {
                pos_size: [c.x * s, c.y * s, c.w * s, c.h * s],
                params: [c.radius * s, c.sigma * s, 0.0, 0.0],
                clip: clip_arr,
                tint: c.tint,
            });
            // Vertical pass (same instance, pass=1)
            self.staged.push(BlurInstance {
                pos_size: [c.x * s, c.y * s, c.w * s, c.h * s],
                params: [c.radius * s, c.sigma * s, 1.0, 0.0],
                clip: clip_arr,
                tint: c.tint,
            });
        }
        if self.staged.len() > self.instance_cap {
            self.instance_cap = self.staged.len().next_power_of_two();
            self.instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("blur instances"),
                size: (self.instance_cap * INSTANCE_SIZE) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.staged));
    }

    pub fn draw<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>, count: u32) {
        if count == 0 || self.staged.is_empty() {
            return;
        }
        let Some(bg) = &self.src_bind_group else {
            return;
        };
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, bg, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        // Draw both passes per node (2 instances per BlurCall)
        pass.draw(0..6, 0..count * 2);
    }
}
