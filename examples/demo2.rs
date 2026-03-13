use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = Column::new(&mut ui);
    let btn = Button::new(&mut ui, "Click meeeeeeeeeee");
    ui.append(root, btn);
    ui.set_root(root);

    ui[root].layout_mut().width = Size::Percent(100.0);
    ui[root].layout_mut().height = Size::Percent(100.0);

    ui.connect(btn, Signal::Press, move |ui| {
        println!("clicked");
    });

    ui.connect(btn, Signal::Click, move |ui| {
        btn.set_text(ui, "Clicked!");
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
