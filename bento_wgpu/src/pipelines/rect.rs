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
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/rect.wgsl").into()),
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

        /*
         * tells the GPU how to read RectInstance out of vertex buffer
         *
         * array_stride is how far for the GPU to jump to the next instance which
         * is just the size of one instance, in this case 32 bytes, meaning that the next instance
         * will start at 32 bytes because theyre all 32 bytes
         *
         * step_mode tells the GPU when to step to the next entry in the buffer
         * - Vertex: step to the next entry after every vertex
         * - Instance: step to the next entry after every instance
         */
        let vertex_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<RectInstance>() as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
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
            ],
        };

        // tells pipeline which bind group layouts its using
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("rect pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("rect pipeline"),
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
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
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

        let capacity = 64;
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("rect vertex buffer"),
            size: (capacity * std::mem::size_of::<RectInstance>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            pipeline,
            vertex_buffer,
            screen_buffer,
            bind_group,
            capacity,
        }
    }

    pub fn draw<'pass>(
        &'pass self,
        rects: &[RectInstance],
        queue: &wgpu::Queue,
        pass: &mut wgpu::RenderPass<'pass>,
    ) {
        if rects.is_empty() {
            return;
        }

        // upload rect data to the GPU
        queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(rects));

        // set up the pipeline and buffers
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // draw
        pass.draw(0..6, 0..rects.len() as u32);
    }
}
