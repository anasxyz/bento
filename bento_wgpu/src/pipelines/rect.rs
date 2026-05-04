use bytemuck::{Pod, Zeroable};
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct RectInstance {
    pub pos_size: [f32; 4],
    pub color: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub transform: [f32; 4],
}

pub struct RectPipeline {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    screen_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    capacity: usize,
    instances: Vec<RectInstance>,
    dirty: Vec<bool>,
    next_slot: u32,
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
            instances: Vec::new(),
            dirty: Vec::new(),
            next_slot: 0,
        }
    }

    pub fn resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32) {
        queue.write_buffer(
            &self.screen_buffer,
            0,
            bytemuck::cast_slice(&[width, height]),
        );
    }

    pub fn alloc_slot(&mut self) -> u32 {
        let slot = self.next_slot;
        self.next_slot += 1;
        // grow instances and dirty vecs
        self.instances.push(RectInstance {
            pos_size: [0.0; 4],
            color: [0.0; 4],
            radii: [0.0; 4],
            border_color: [0.0; 4],
            border_widths: [0.0; 4],
            transform: [1.0, 0.0, 0.0, 1.0],
        });
        self.dirty.push(true);
        slot
    }

    pub fn write_slot(&mut self, slot: u32, instance: RectInstance) {
        let s = slot as usize;
        if bytemuck::bytes_of(&self.instances[s]) != bytemuck::bytes_of(&instance) {
            self.instances[s] = instance;
            self.dirty[s] = true;
            println!("rect slot {} marked dirty", slot);
        }
    }

    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        if self.instances.is_empty() {
            return;
        }

        if self.instances.len() > self.capacity {
            self.capacity = self.instances.len().next_power_of_two();
            self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("rect vertex buffer"),
                size: (self.capacity * std::mem::size_of::<RectInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            queue.write_buffer(
                &self.vertex_buffer,
                0,
                bytemuck::cast_slice(&self.instances),
            );
            for d in &mut self.dirty {
                *d = false;
            }
            return;
        }

        for (i, dirty) in self.dirty.iter_mut().enumerate() {
            if *dirty {
                println!("uploading rect slot {}", i);
                let offset = (i * std::mem::size_of::<RectInstance>()) as u64;
                queue.write_buffer(
                    &self.vertex_buffer,
                    offset,
                    bytemuck::bytes_of(&self.instances[i]),
                );
                *dirty = false;
            }
        }
    }

    pub fn draw_slot<'pass>(&'pass self, pass: &mut wgpu::RenderPass<'pass>, slot: u32) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..6, slot..slot + 1);
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
