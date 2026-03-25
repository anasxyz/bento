use bento_wgpu::RenderContext;
use winit::event_loop::EventLoop;

use crate::runner::Runner;
use crate::settings::WindowConfig;

pub struct AppWindow {
    config: WindowConfig,
}

impl AppWindow {
    pub fn new(config: WindowConfig) -> Self {
        Self { config }
    }

    pub fn run(self) {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        let ctx = rt.block_on(RenderContext::new());

        let event_loop = EventLoop::new().unwrap();
        event_loop
            .run_app(&mut Runner::new(ctx, self.config))
            .unwrap();
    }
}
