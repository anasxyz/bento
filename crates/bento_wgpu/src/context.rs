use wgpu;

pub struct RenderContext {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl RenderContext {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                #[cfg(target_arch = "wasm32")]
                // fallback on WASM
                force_fallback_adapter: true,
                #[cfg(not(target_arch = "wasm32"))]
                // native
                force_fallback_adapter: false,
            })
            .await
            .expect("bento_wgpu: no suitable GPU adapter found");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("bento_wgpu: failed to create GPU device");

        Self {
            instance,
            adapter,
            device,
            queue,
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
            .request_device(&wgpu::DeviceDescriptor {
                #[cfg(target_arch = "wasm32")]
                // adapter.limits() to query adapter limits
                // or wgpu::Limits::default()
                required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                #[cfg(not(target_arch = "wasm32"))]
                required_limits: wgpu::Limits::default(),
                ..wgpu::DeviceDescriptor::default()
            })
            .await
            .expect("bento_wgpu: failed to create GPU device");

        Self {
            instance,
            adapter,
            device,
            queue,
        }
    }
}
