use bento_wgpu::RenderContext;
use winit::event_loop::EventLoop;

use crate::runner::Runner;
use crate::settings::WindowConfig;
use crate::ui::Ui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowHandle(pub(crate) u32);

pub struct App {
    pending: Vec<(WindowHandle, WindowConfig, Ui)>,
    next_handle: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            next_handle: 0,
        }
    }

    pub fn open_window(&mut self, config: WindowConfig, ui: Ui) -> WindowHandle {
        let handle = WindowHandle(self.next_handle);
        self.next_handle += 1;
        self.pending.push((handle, config, ui));
        handle
    }

    pub fn run(self) {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        let ctx = rt.block_on(RenderContext::new());
        let event_loop = EventLoop::new().unwrap();
        event_loop
            .run_app(&mut Runner::new(ctx, self.pending))
            .unwrap();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
