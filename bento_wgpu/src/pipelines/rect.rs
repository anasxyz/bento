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

        /*
         * bind group layout is the description of what slots exists and
         * what type of data goes in each one
         *
         * binding field is what slot number
         *
         * visibility field is which shader is it relevant to between VERTEX or FRAGMENT
         *
         * outer ty field is type of resource being stored in this slot, in this case its a buffer
         *
         * inner ty field is the type of buffer being stored, in this case a uniform
         * different types of buffers as far as I know are uniform or storage types:
         * - uniform buffer: CPU writes it, shader reads it, smaller and faster
         * - storage buffer: shader both reads and writes to it. the shader can basically modify
         * values and they can be read back by the CPU
         */
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("rect bind group layout"),
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

        /*
        * bind group is where resource to binding slot allocation actually happens
        *
        * bind screen_buffer to slot 0
        */
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rect bind group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: screen_buffer.as_entire_binding(),
            }],
        });
    }
}
