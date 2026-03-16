use bento::*;

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

    let mut following = true;
    let mut current_id = Some(follow_id);

    ui.connect(ui.global(), move |ui, event| {
        if let Event::KeyPress {
            key: Key::Space, ..
        } = event
        {
            if following {
                if let Some(id) = current_id {
                    ui.disconnect(id);
                    current_id = None;
                    following = false;
                    println!("stopped following");
                }
            } else {
                let id = ui.connect(ui.global(), move |ui, event| {
                    if let Event::MouseMove { x, y } = event {
                        ui[cursor].layout.inset[0] = Size::Fixed(y - size / 2.0);
                        ui[cursor].layout.inset[3] = Size::Fixed(x - size / 2.0);
                    }
                });
                current_id = Some(id);
                following = true;
                println!("started following");
            }
        }
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
