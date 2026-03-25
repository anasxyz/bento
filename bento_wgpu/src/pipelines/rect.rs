// pipelines/rect.rs
//
// Instanced rounded-rect pipeline.
//
// Each rect node gets a stable slot in the GPU instance buffer.
// Only dirty slots are uploaded each frame — a changed rect costs
// one write_buffer call of 112 bytes, everything else is skipped.
//
// Slots are assigned by the Renderer via the SlotAllocator and written
// into RectNode::slot. This pipeline just owns the GPU buffer and draws.

use bytemuck::{Pod, Zeroable};
use std::mem;
use wgpu;

// ── GPU instance layout ───────────────────────────────────────────────────────
// Must match rect.wgsl vertex input layout exactly.

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Instance {
    pos_size: [f32; 4], // x, y, w, h  (physical pixels)
    params: [f32; 4],   // radius, aa_width, 0, 0
    fill_color: [f32; 4],
    border_color: [f32; 4],
    clip: [f32; 4],          // x, y, x2, y2 (physical pixels); [0,0,0,0] = no clip
    screen_size: [f32; 4],   // w, h, 0, 0
    border_widths: [f32; 4], // top, right, bottom, left (physical pixels)
}

const INSTANCE_SIZE: usize = mem::size_of::<Instance>();
const INITIAL_CAPACITY: usize = 512;

// ── RectPipeline ──────────────────────────────────────────────────────────────

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    instance_buffer: wgpu::Buffer,
    instance_cap: usize,
    instances: Vec<Instance>, // CPU mirror of GPU buffer, indexed by slot
    dirty: Vec<bool>,
    screen_w: f32, // physical pixels
    screen_h: f32,
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

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("bento_wgpu::rect pipeline"),
            layout: None,
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
            screen_w,
            screen_h,
        }
    }

    // ── called by Renderer ────────────────────────────────────────────────────

    /// Ensure the CPU-side slot vec is large enough and mark it dirty.
    /// Called when the Renderer assigns a new slot to a node.
    pub fn ensure_slot(&mut self, slot: usize) {
        while self.instances.len() <= slot {
            self.instances.push(Instance {
                pos_size: [0.0; 4],
                params: [0.0; 4],
                fill_color: [0.0; 4],
                border_color: [0.0; 4],
                clip: [0.0; 4],
                screen_size: [self.screen_w, self.screen_h, 0.0, 0.0],
                border_widths: [0.0; 4],
            });
            self.dirty.push(true);
        }
    }

    /// Write a rect node into its slot. Compares bytes — only marks dirty if
    /// the instance data actually changed.
    pub fn write_slot(
        &mut self,
        slot: usize,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: [f32; 4],
        radius: f32,
        border_color: [f32; 4],
        border_widths: [f32; 4],
        clip: Option<[f32; 4]>,
        scale: f32,
    ) {
        let s = scale;
        let radius = (radius * s).min(w * s * 0.5).min(h * s * 0.5);
        let clip_arr = match clip {
            Some([cx, cy, cx2, cy2]) => [cx * s, cy * s, cx2 * s, cy2 * s],
            None => [0.0; 4],
        };
        let new_inst = Instance {
            pos_size: [x * s, y * s, w * s, h * s],
            params: [radius, 1.0, 0.0, 0.0],
            fill_color: color,
            border_color,
            clip: clip_arr,
            screen_size: [self.screen_w, self.screen_h, 0.0, 0.0],
            border_widths: [
                border_widths[0] * s,
                border_widths[1] * s,
                border_widths[2] * s,
                border_widths[3] * s,
            ],
        };
        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&new_inst) {
            self.instances[slot] = new_inst;
            self.dirty[slot] = true;
        }
    }

    /// Zero out a slot so it renders as invisible. Called when a node is
    /// hidden or removed.
    pub fn clear_slot(&mut self, slot: usize) {
        if slot >= self.instances.len() {
            return;
        }
        let zero = Instance {
            pos_size: [0.0; 4],
            params: [0.0; 4],
            fill_color: [0.0; 4],
            border_color: [0.0; 4],
            clip: [0.0; 4],
            screen_size: [self.screen_w, self.screen_h, 0.0, 0.0],
            border_widths: [0.0; 4],
        };
        if bytemuck::bytes_of(&self.instances[slot]) != bytemuck::bytes_of(&zero) {
            self.instances[slot] = zero;
            self.dirty[slot] = true;
        }
    }

    /// Update physical screen dimensions — marks all slots dirty so
    /// screen_size uniforms embedded in each instance get re-uploaded.
    pub fn resize(&mut self, screen_w: f32, screen_h: f32) {
        self.screen_w = screen_w;
        self.screen_h = screen_h;
        for d in &mut self.dirty {
            *d = true;
        }
    }

    /// Mark all slots dirty — called when the GPU buffer is reallocated
    /// or after a full invalidation.
    pub fn invalidate(&mut self) {
        for d in &mut self.dirty {
            *d = true;
        }
    }

    /// Upload dirty slots to the GPU and issue the draw call.
    /// Coalesces contiguous dirty ranges into single write_buffer calls.
    pub fn render<'pass>(
        &'pass mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if self.instances.is_empty() {
            return;
        }

        // Grow GPU buffer if needed — full upload after realloc
        if self.instances.len() > self.instance_cap {
            let new_cap = self.instances.len().next_power_of_two();
            self.instance_buffer = Self::make_buffer(device, new_cap);
            self.instance_cap = new_cap;
            queue.write_buffer(
                &self.instance_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
            for d in &mut self.dirty {
                *d = false;
            }
        } else {
            // Partial upload — coalesce contiguous dirty slots
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
                if i < self.instances.len() {
                    self.dirty[i] = false;
                }
            }
        }

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));
        pass.draw(0..6, 0..self.instances.len() as u32);
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

// ── vertex attribute layout ───────────────────────────────────────────────────

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
];
