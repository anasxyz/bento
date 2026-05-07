use bento_winit::{App, WindowConfig};

fn main() {
    let mut app = App::new();
    app.open_window(WindowConfig::default());
    app.run();
}
