use bento::*;

fn main() {
    let mut ui = Ui::new();

    let btn = Rect::new(&mut ui);
    let label = Label::new(&mut ui, "hello");
    let root = Column::new(&mut ui);

    ui.append(root, btn);
    ui.append(root, label);
    ui.set_root(root);

    ui[btn].bg_color = Color::rgb(100, 0, 0);
    ui[label].text = "world".to_string();

    println!("btn bg: {:?}", ui[btn].bg_color);
    println!("label text: {}", ui[label].text);
    println!("root children: {:?}", ui.children(root));
}
