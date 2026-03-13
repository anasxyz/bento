use bento::*;

fn main() {
    let mut ui = Ui::new();

    let btn = ui.add(Rect::new());
    let root = ui.add(Column::new());

    ui.append(root, btn);
    ui.set_root(root);

    ui[btn].bg_color = rgb(100, 0, 0);

    println!("btn bg: {:?}", ui[btn].bg_color);
    println!("root children: {:?}", ui.children(root));
}
