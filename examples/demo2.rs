use bento::*;
use std::cell::Cell;
use std::rc::Rc;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui[root].layout.width = Size::Percent(100.0);
    ui[root].layout.height = Size::Percent(100.0);
    ui[root].layout.align_items = AlignItems::Center;
    ui[root].layout.justify_content = JustifyContent::Center;
    ui[root].bg_color = Some(Color::hex("181825"));
    ui.set_root(root);

    let size = 20.0;
    let cursor = ui.add(Rect::new());
    ui[cursor].layout.width = Size::Fixed(size);
    ui[cursor].layout.height = Size::Fixed(size);
    ui[cursor].bg_color = Color::RED;
    ui[cursor].layout.position = Position::Absolute;
    ui.append(root, cursor);

    let follow_id = ui.connect(ui.global(), move |ui, event| {
        if let Event::MouseMove { x, y } = event {
            ui[cursor].layout.inset[0] = Size::Fixed(y - size / 2.0);
            ui[cursor].layout.inset[3] = Size::Fixed(x - size / 2.0);
        }
    });

    let following = Rc::new(Cell::new(true));
    let current_id = Rc::new(Cell::new(Some(follow_id)));

    let following2 = following.clone();
    let current_id2 = current_id.clone();

    ui.connect(ui.global(), move |ui, event| {
        if let Event::KeyPress {
            key: Key::Space, ..
        } = event
        {
            if following2.get() {
                if let Some(id) = current_id2.get() {
                    ui.disconnect(id);
                    current_id2.set(None);
                    following2.set(false);
                    println!("stopped following");
                }
            } else {
                let id = ui.connect(ui.global(), move |ui, event| {
                    if let Event::MouseMove { x, y } = event {
                        ui[cursor].layout.inset[0] = Size::Fixed(y - size / 2.0);
                        ui[cursor].layout.inset[3] = Size::Fixed(x - size / 2.0);
                    }
                });
                current_id2.set(Some(id));
                following2.set(true);
                println!("started following");
            }
        }
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
