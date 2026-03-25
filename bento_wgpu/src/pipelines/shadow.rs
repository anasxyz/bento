// pipelines/shadow.rs
//
// Instanced box-shadow pipeline.
// Same slot model as the rect pipeline — persistent slots, dirty-only uploads.
// Shadows are drawn before rects (lower z) so they appear beneath UI chrome.

use std::mem;
use bytemuck::{Pod, Zeroable};
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Instance {
    rect:   [f32; 4],  // x, y, w, h (physical pixels)
    color:  [f32; 4],
    params: [f32; 4],  // corner_radius, blur, offset_x, offset_y (physical pixels)
}

const INSTANCE_SIZE: usize = mem::size_of::<Instance>();
const INITIAL_CAPACITY: usize = 64;

pub struct ShadowPipeline {
    pipeline:        wgpu::RenderPipeline,
    screen_buffer:   wgpu::Buffer,
    screen_bg:       wgpu::BindGroup,
    instance_buffer: wgpu::Buffer,
    instance_cap:    usize,
    instances:       Vec<Instance>,
    dirty:           Vec<bool>,
    screen_w:        f32,  // physical pixels
    screen_h:        f32,
}

impl ShadowPipeline {
    pub fn new(
        device:   &wgpu::Device,
        queue:    &wgpu::Queue,
        format:   wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bento_render::shadow shader"),
            source: wgpu::ShaderSource::Wgsl(SHADOW_SHADER.into()),
        });

        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bento_render::shadow screen uniform"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &screen_buffer, 0,
            bytemuck::bytes_of(&[screen_w, screen_h, 0.0f32, 0.0f32]),
        );

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bento_render::shadow bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let screen_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bento_render::shadow bg"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("bento_render::shadow layout"),
            bind_group_layouts: &[&bgl],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bento_render::shadow pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: INSTANCE_SIZE as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute { offset: 0,  shader_location: 0, format: wgpu::VertexFormat::Float32x4 },
                        wgpu::VertexAttribute { offset: 16, shader_location: 1, format: wgpu::VertexFormat::Float32x4 },
                        wgpu::VertexAttribute { offset: 32, shader_location: 2, format: wgpu::VertexFormat::Float32x4 },
                    ],
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
                cull_mode: None,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        let instance_buffer = Self::make_buffer(device, INITIAL_CAPACITY);

        Self {
            pipeline, screen_buffer, screen_bg,
            instance_buffer, instance_cap: INITIAL_CAPACITY,
            instances: Vec::new(), dirty: Vec::new(),
            screen_w, screen_h,
        }
    }

    pub fn ensure_slot(&mut self, slot: usize) {
        while self.instances.len() <= slot {
            self.instances.push(Instance { rect: [0.0;4], color: [0.0;4], params: [0.0;4] });
            self.dirty.push(true);
        }
    }

    pub fn write_slot(
        &mut self,
        slot: usize,
        x: f32, y: f32, w: f32, h: f32,
        color: [f32; 4],
        blur: f32, radius: f32,
        offset_x: f32, offset_y: f32,
        scale: f32,
    ) {
        let s = scale;
        let new_inst = Instance {
            rect:   [x*s, y*s, w*s, h*s],
            color,
            params: [radius*s, blur*s, offset_x*s, offset_y*s],
        };
        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&new_inst) {
            self.instances[slot] = new_inst;
            self.dirty[slot] = true;
        }
    }

    pub fn clear_slot(&mut self, slot: usize) {
        if slot >= self.instances.len() { return; }
        let zero = Instance { rect: [0.0;4], color: [0.0;4], params: [0.0;4] };
        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&zero) {
            self.instances[slot] = zero;
            self.dirty[slot] = true;
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, screen_w: f32, screen_h: f32) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        queue.write_buffer(
            &self.screen_buffer, 0,
            bytemuck::bytes_of(&[screen_w, screen_h, 0.0f32, 0.0f32]),
        );
        for d in &mut self.dirty { *d = true; }
    }

    pub fn invalidate(&mut self) {
        for d in &mut self.dirty { *d = true; }
    }

    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue:  &wgpu::Queue,
        pass:   &mut wgpu::RenderPass<'pass>,
    ) {
        if self.instances.is_empty() { return; }

        if self.instances.len() > self.instance_cap {
            let new_cap = self.instances.len().next_power_of_two();
            self.instance_buffer = Self::make_buffer(device, new_cap);
            self.instance_cap = new_cap;
            queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&self.instances));
            for d in &mut self.dirty { *d = false; }
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
                        range_start = None;
                    }
                    _ => {}
                }
                if i < self.instances.len() { self.dirty[i] = false; }
            }
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.screen_bg, &[]);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        pass.draw(0..6, 0..self.instances.len() as u32);
    }

    fn make_buffer(device: &wgpu::Device, capacity: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bento_render::shadow instances"),
            size: (capacity * INSTANCE_SIZE) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }
}

const SHADOW_SHADER: &str = r#"
struct Screen { size: vec2<f32>, _pad: vec2<f32> }
@group(0) @binding(0) var<uniform> screen: Screen;

struct Instance {
    @location(0) rect:   vec4<f32>,
    @location(1) color:  vec4<f32>,
    @location(2) params: vec4<f32>,  // radius, blur, offset_x, offset_y
}

struct VertOut {
    @builtin(position) pos:    vec4<f32>,
    @location(0)       uv:     vec2<f32>,
    @location(1)       size:   vec2<f32>,
    @location(2)       color:  vec4<f32>,
    @location(3)       params: vec4<f32>,
}

var<private> VERTS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2(0.0,0.0), vec2(1.0,0.0), vec2(1.0,1.0),
    vec2(0.0,0.0), vec2(1.0,1.0), vec2(0.0,1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VertOut {
    let blur   = inst.params.y;
    let expand = blur * 2.0;
    let ox     = inst.params.z;
    let oy     = inst.params.w;
    // expand the quad by blur on all sides
    let x = inst.rect.x + ox - expand;
    let y = inst.rect.y + oy - expand;
    let w = inst.rect.z + expand * 2.0;
    let h = inst.rect.w + expand * 2.0;
    let uv = VERTS[vi];
    let px = x + uv.x * w;
    let py = y + uv.y * h;
    let cx = (px / screen.size.x) * 2.0 - 1.0;
    let cy = 1.0 - (py / screen.size.y) * 2.0;
    var out: VertOut;
    out.pos    = vec4(cx, cy, 0.0, 1.0);
    out.uv     = uv;
    out.size   = vec2(w, h);
    out.color  = inst.color;
    out.params = inst.params;
    return out;
}

fn sd_rounded_box(p: vec2<f32>, size: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - size + vec2(r);
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    let radius = in.params.x;
    let blur   = in.params.y;
    // local coords centred on the shadow quad
    let p      = (in.uv - vec2(0.5)) * in.size;
    let half   = in.size * 0.5 - vec2(blur * 2.0);
    let dist   = sd_rounded_box(p, half, radius);
    let alpha  = 1.0 - smoothstep(-blur, blur, dist);
    return vec4(in.color.rgb, in.color.a * alpha);
}
"#;
