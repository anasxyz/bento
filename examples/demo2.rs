use bento::*;

fn main() {
    let mut ui = Ui::new();

    let btn = ui.add(Rect::new()); // Handle<Rect>
    let root = ui.add(Column::new()); // Handle<Container>

    ui.append(root, btn);
    ui.set_root(root);

    ui[btn].bg_color = Some(rgb(100, 0, 0)); // returns &mut Rect directly
}
