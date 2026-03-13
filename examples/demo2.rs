use bento::*;

fn main() {
    let mut ui = Ui::new();

    let btn = Rect::new(&mut ui);
    let label = Label::new(&mut ui, "hello world");
    let root = Column::new(&mut ui);

    ui.append(root, btn);
    ui.append(root, label);
    ui.set_root(root);

    ui[btn].layout_mut().width = Size::Fixed(100.0);
    ui[btn].layout_mut().height = Size::Fixed(40.0);
    ui[btn].layout_mut().margin = [10.0, 10.0, 10.0, 10.0];
    ui[btn].bg_color = rgb(100, 0, 200);
    ui[label].text_color = Color::WHITE;
    ui[label].font_size = 18.0;

    AppWindow::new(WindowConfig::default()).run(ui, |ui| {

    });
}
