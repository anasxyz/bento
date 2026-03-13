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

    layout_tree(&mut ui, 800.0, 600.0, &mut Fonts::new());

    println!(
        "btn x:{} y:{} w:{} h:{}",
        ui[btn].layout().x,
        ui[btn].layout().y,
        ui[btn].layout().w,
        ui[btn].layout().h
    );
    println!(
        "label x:{} y:{} w:{} h:{}",
        ui[label].layout().x,
        ui[label].layout().y,
        ui[label].layout().w,
        ui[label].layout().h
    );
}
