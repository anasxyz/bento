use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use wgpu;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct ImageInstance {
    pub pos_size: [f32; 4],
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub transform: [f32; 4],
    pub clip: [f32; 4],
    pub opacity: f32,
    pub _pad: [f32; 3],
}

pub struct ImageSpec {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub image_id: u64,
    pub radii: [f32; 4],
    pub border_color: [f32; 4],
    pub border_widths: [f32; 4],
    pub transform: [f32; 4],
    pub clip: [f32; 4],
    pub opacity: f32,
}

pub struct ImagePipeline {
    pipeline: wgpu::RenderPipeline,
    screen_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    screen_bind_group_layout: wgpu::BindGroupLayout,
    screen_bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    capacity: usize,
    textures: HashMap<u64, (wgpu::Texture, wgpu::TextureView, wgpu::BindGroup)>,
    instances: Vec<ImageInstance>,
    // maps slot index to image_id to know which texture to bind per draw
    slots: Vec<u64>,
}
