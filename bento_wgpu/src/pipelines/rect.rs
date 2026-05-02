// instanced rounded rect pipeline
//
// instance layout changed from pos_size (x,y,w,h) to a 2x3 affine transform
// plus local size. this lets the vertex shader place all four quad corners
// through the transform, supporting rotation and scale inherited from
// TransformNode parents.
//
// the SDF for rounded corners still runs in local space using local_size,
// so rounded corners stay perfectly round even on rotated rects.

use crate::scene::Mat2x3;
use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu;

// Per-instance data sent to the GPU.
//
// transform encodes the full 2x3 affine matrix as two vec4s:
//   col01  = (a, b, c, d)  — the 2x2 rotation+scale part
//   trans  = (tx, ty, pw, ph) — translation + local pixel size
//
// In the shader:
//   screen_pos = col01.xy * local.x + col01.zw * local.y + trans.xy
//
// local_size (pw, ph) is packed into trans.zw to keep the struct at
// the same 112 bytes as before (no GPU buffer size change).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Instance {
    col01: [f32; 4], // a, b, c, d
    trans: [f32; 4], // tx, ty, pw, ph
    fill_color: [f32; 4],
    border_color: [f32; 4],
    clip: [f32; 4],
    border_widths: [f32; 4],
    params: [f32; 4], // radius, aa_width, _, _
    // Gradient: grad_color0.a==0 means solid fill (grad inactive)
    grad_color0: [f32; 4], // start colour (or sentinel [0,0,0,0])
    grad_color1: [f32; 4], // end colour
    grad_params: [f32; 4], // cos(angle), sin(angle), _, _
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct ScreenUniform {
    size: [f32; 2],
    _pad: [f32; 2],
}

const INSTANCE_SIZE: usize = mem::size_of::<Instance>();
const INITIAL_CAPACITY: usize = 512;

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_cap: usize,
    pub instances: Vec<Instance>,
    dirty: Vec<bool>,
    screen_uniform: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    screen_w: f32,
    screen_h: f32,
    pub upload_count: u32,
}

impl RectPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bento_wgpu::rect shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into()),
        });

        let screen_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bento_wgpu::rect screen uniform"),
            size: mem::size_of::<ScreenUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bento_wgpu::rect bind group layout"),
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bento_wgpu::rect bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_uniform.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bento_wgpu::rect pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bento_wgpu::rect pipeline"),
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

        Self {
            pipeline,
            instance_buffer,
            instance_cap: INITIAL_CAPACITY,
            instances: Vec::new(),
            dirty: Vec::new(),
            screen_uniform,
            bind_group,
            screen_w,
            screen_h,
            upload_count: 0,
        }
    }

    pub fn ensure_slot(&mut self, slot: usize) {
        while self.instances.len() <= slot {
            self.instances.push(Instance {
                col01: [0.0; 4],
                trans: [0.0; 4],
                fill_color: [0.0; 4],
                border_color: [0.0; 4],
                clip: [0.0; 4],
                border_widths: [0.0; 4],
                params: [0.0; 4],
                grad_color0: [0.0; 4],
                grad_color1: [0.0; 4],
                grad_params: [0.0; 4],
            });
            self.dirty.push(true);
        }
    }

    pub fn write_slot(
        &mut self,
        slot: usize,
        transform: Mat2x3,
        local_w: f32,
        local_h: f32,
        color: [f32; 4],
        radius: f32,
        border_color: [f32; 4],
        border_widths: [f32; 4],
        clip: Option<[f32; 4]>,
        scale: f32,
        // Gradient: None = solid fill
        gradient: Option<([f32; 4], [f32; 4], f32)>, // (color0, color1, angle_rad)
    ) {
        let s = scale;
        let a = transform[0] * s;
        let b = transform[1] * s;
        let c = transform[2] * s;
        let d = transform[3] * s;
        let tx = transform[4] * s;
        let ty = transform[5] * s;
        let pw = local_w * s;
        let ph = local_h * s;
        let r = (radius * s).min(pw * 0.5).min(ph * 0.5);
        let aa = if pw > 2.0 { 1.0 } else { 0.0 };

        let clip_arr = match clip {
            Some([cx, cy, cx2, cy2]) => [cx * s, cy * s, cx2 * s, cy2 * s],
            None => [0.0; 4],
        };
        let bw = [
            (border_widths[0] * s).round(),
            (border_widths[1] * s).round(),
            (border_widths[2] * s).round(),
            (border_widths[3] * s).round(),
        ];

        // Gradient encoding: use grad_color0.a as active flag.
        // Solid fill: grad_color0 = [0,0,0,0] (alpha=0 = inactive).
        let (gc0, gc1, gp) = match gradient {
            Some((c0, c1, angle)) => {
                let (sin_a, cos_a) = angle.sin_cos();
                (c0, c1, [cos_a, sin_a, 0.0, 0.0])
            }
            None => ([0.0f32; 4], [0.0f32; 4], [0.0f32; 4]),
        };

        let new_inst = Instance {
            col01: [a, b, c, d],
            trans: [tx, ty, pw, ph],
            fill_color: color,
            border_color,
            clip: clip_arr,
            border_widths: bw,
            params: [r, aa, 0.0, 0.0],
            grad_color0: gc0,
            grad_color1: gc1,
            grad_params: gp,
        };

        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&new_inst) {
            self.instances[slot] = new_inst;
            self.dirty[slot] = true;
        }
    }

    pub fn clear_slot(&mut self, slot: usize) {
        if slot >= self.instances.len() {
            return;
        }
        let zero = Instance {
            col01: [0.0; 4],
            trans: [0.0; 4],
            fill_color: [0.0; 4],
            border_color: [0.0; 4],
            clip: [0.0; 4],
            border_widths: [0.0; 4],
            params: [0.0; 4],
            grad_color0: [0.0; 4],
            grad_color1: [0.0; 4],
            grad_params: [0.0; 4],
        };
        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&zero) {
            self.instances[slot] = zero;
            self.dirty[slot] = true;
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, screen_w: f32, screen_h: f32) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        queue.write_buffer(
            &self.screen_uniform,
            0,
            bytemuck::bytes_of(&ScreenUniform {
                size: [screen_w, screen_h],
                _pad: [0.0; 2],
            }),
        );
    }

    pub fn invalidate(&mut self) {
        for d in &mut self.dirty {
            *d = true;
        }
    }

    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.instances.is_empty() {
            return;
        }
        self.upload_count = 0;

        if self.instances.len() > self.instance_cap {
            let new_cap = self.instances.len().next_power_of_two();
            self.instance_buffer = Self::make_buffer(device, new_cap);
            self.instance_cap = new_cap;
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
            self.upload_count = self.instances.len() as u32;
            for d in &mut self.dirty {
                *d = false;
            }
        } else {
            let mut range_start: Option<usize> = None;
            for i in 0..=self.instances.len() {
                let is_dirty = i < self.instances.len() && self.dirty[i];
                match (is_dirty, range_start) {
                    (true, None) => range_start = Some(i),
                    (false, Some(start)) => {
                        queue.write_buffer(
                            &self.instance_buffer,
                            (start * INSTANCE_SIZE) as u64,
                            bytemuck::cast_slice(&self.instances[start..i]),
                        );
                        self.upload_count += (i - start) as u32;
                        range_start = None;
                    }
                    _ => {}
                }
                if i < self.instances.len() {
                    self.dirty[i] = false;
                }
            }
        }
    }

    pub fn draw_slots<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>, slots: &[usize]) {
        if slots.is_empty() || self.instances.is_empty() {
            return;
        }
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        let mut sorted = slots.to_vec();
        sorted.sort_unstable();

        let mut i = 0;
        while i < sorted.len() {
            let start = sorted[i];
            let mut end = start + 1;
            while i + 1 < sorted.len() && sorted[i + 1] == end {
                end += 1;
                i += 1;
            }
            if start < self.instances.len() {
                let clamped_end = end.min(self.instances.len());
                pass.draw(0..6, start as u32..clamped_end as u32);
            }
            i += 1;
        }
    }

    fn make_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bento_wgpu::rect instances"),
            size: (capacity * INSTANCE_SIZE) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}

// Vertex attribute layout:
//    0 → col01         16 → trans          32 → fill_color
//   48 → border_color  64 → clip           80 → border_widths
//   96 → params       112 → grad_color0   128 → grad_color1   144 → grad_params
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
    wgpu::VertexAttribute {
        offset: 80,
        shader_location: 5,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 96,
        shader_location: 6,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 112,
        shader_location: 7,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 128,
        shader_location: 8,
        format: wgpu::VertexFormat::Float32x4,
    },
    wgpu::VertexAttribute {
        offset: 144,
        shader_location: 9,
        format: wgpu::VertexFormat::Float32x4,
    },
];
