use glyphon::FontSystem;
use wgpu;

pub struct RenderContext {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub font_system: FontSystem,
}

impl RenderContext {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("bento_wgpu: no suitable gpu adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("bento_wgpu: failed to create gpu device");

        Self {
            instance,
            adapter,
            device,
            queue,
            font_system: FontSystem::new(),
        }
    }

    pub async fn new_for_surface(
        instance: wgpu::Instance,
        compatible_surface: &wgpu::Surface<'_>,
    ) -> Self {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(compatible_surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("bento_wgpu: no adapter compatible with the given surface");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("bento_wgpu: failed to create gpu device");

        Self {
            instance,
            adapter,
            device,
            queue,
            font_system: FontSystem::new(),
        }
    }
}
