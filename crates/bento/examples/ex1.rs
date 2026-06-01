use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let root = ui.root();
    ui.set(root, |g: &mut Group| {
        g.layout = Layout::Row {
            gap: 8.0,
            padding: [16.0, 16.0, 16.0, 16.0],
            main_axis: MainAxis::Start,
            cross_axis: CrossAxis::Start,
            wrap: true,
        };
        g.width = Size::Fill;
        g.height = Size::Fill;
    });

    for i in 0..12 {
        ui.add(root, Button::new(&format!("Button {}", i)));
    }

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
