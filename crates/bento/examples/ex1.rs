use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(0.0, 0.0, 400.0, 100.0));
    ui.get_mut(rect).unwrap().border_widths = [3.0; 4];
    ui.get_mut(rect).unwrap().border_color = [0.0, 0.0, 0.0, 1.0];
    ui.get_mut(rect).unwrap().color = [0.0, 0.2, 0.0, 1.0];
    ui.get_mut(rect).unwrap().radii = [15.0; 4];

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
