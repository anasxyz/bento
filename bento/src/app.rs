/*
* user facing entry point
*/

use bento_wgpu::RenderContext;
use winit::event_loop::EventLoop;

use crate::runner::Runner;
use crate::settings::WindowConfig;

/// stable handle to an open window
/// returned by App::open_window()
/// used to close the window or identify it in callbacks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowHandle(pub(crate) u32);

pub struct App {
    pending: Vec<(WindowHandle, WindowConfig)>,
    next_handle: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            pending: Vec::new(),
            next_handle: 0,
        }
    }

    /// queue a window to be opened when the event loop starts
    /// returns a stable handle the user can hold onto
    pub fn open_window(&mut self, config: WindowConfig) -> WindowHandle {
        let handle = WindowHandle(self.next_handle);
        self.next_handle += 1;
        self.pending.push((handle, config));
        handle
    }

    /// start the event loop
    /// blocks until all windows are closed
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
