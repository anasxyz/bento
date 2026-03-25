// RenderContext owns the wgpu device and queue
//
// create one per application and shared across all windows/surfaces

use wgpu;

pub struct RenderContext {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl RenderContext {
    /// create a RenderContext
    /// this is async because wgpu adapter/device request is async
    /// call with pollster::block_on or within an async runtime
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
        }
    }

    /// create a RenderContext compatible with a specific window surface
    /// use this when you need to ensure the adapter supports the windows surface
    /// (i beliebe this is required on some platforms / drivers)
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
        }
    }

    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}
