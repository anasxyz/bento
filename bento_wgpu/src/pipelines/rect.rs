use bytemuck::{Pod, Zeroable};
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct RectInstance {
    pub pos_size: [f32; 4],
    pub color: [f32; 4],
}

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    screen_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    capacity: usize,
}

impl RectPipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        screen_w: f32,
        screen_h: f32,
    ) -> Self {
        // create rect shader module
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("rect shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("rect.wgsl").into()),
        });

        /*
         * create buffer holding screen size floats, 4 bytes each
         * shader needs to read it to know screen size to convert pixel
         * coords to NDC
         *
         * UNIFORM = shader can read this buffer
         * COPY_DST = CPU can write to this buffer
         */
        let screen_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect screen uniform"),
            size: 8,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        /*
         * convert two screen size floats into raw bytes &[u8]
         * then upload from CPU memory to GPU buffer created above
         *
         * 0 is byte offset to start writing at
         */
        queue.write_buffer(
            &screen_buffer,
            0,
            bytemuck::cast_slice(&[screen_w, screen_h]),
        );
    }
}
