use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let root = ui.root();
    ui.set(root, |g: &mut Group| {
        g.layout = Layout::Column {
            gap: 8.0,
            padding: [16.0, 16.0, 16.0, 16.0],
            main_axis: MainAxis::Center,
            cross_axis: CrossAxis::End,
            wrap: true,
        };
        g.width = Size::Fill;
        g.height = Size::Fill;
    });

    let button = ui.add(root, Button::new(&format!("Button")));
    ui.set(button, |b: &mut Button| {
        b.set_border_radius([8.0, 8.0, 8.0, 8.0]);
        b.set_border_color([0.0, 0.0, 0.0, 1.0]);
        b.set_border_width([3.0, 3.0, 0.0, 0.0]);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
