use bento::*;

fn main() {
    let mut ui = Ui::new();

    let btn = Rect::new(&mut ui);
    let root = Column::new(&mut ui);
    ui.append(root, btn);
    ui.set_root(root);

    ui[btn].layout_mut().width = Size::Fixed(100.0);
    ui[btn].layout_mut().height = Size::Fixed(40.0);
    ui[btn].bg_color = rgb(100, 0, 200);

    ui[btn].on_click(|ui| {
        println!("clicked");
    });

    AppWindow::new(WindowConfig::default()).run(ui, |ui| {});
}
