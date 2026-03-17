use bento::*;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_bg_color(Some(Color::hex("1e1e2e")));
    ui.set_root(root);

    let btn = ui.add(Rect::new());
    ui.get_mut(btn)
        .unwrap()
        .set_padding([10.0, 24.0, 10.0, 24.0])
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_bg_color(Color::hex("cba6f7"))
        .set_border_radius(Some(8.0))
        .set_border([0.0, 0.0, 4.0, 0.0]);
    ui.append(root, btn);

    let label = ui.add(Label::new("Click me"));
    ui.get_mut(label)
        .unwrap()
        .set_text_color(Color::hex("1e1e2e"))
        .set_font_size(15.0)
        .set_font_weight(600);
    ui.append(btn, label);

    ui.connect(btn, move |ui, event| match event {
        Event::Hover => {
            ui.get_mut(btn).unwrap().set_bg_color(Color::hex("b4befe"));
        }
        Event::HoverEnd => {
            ui.get_mut(btn).unwrap().set_bg_color(Color::hex("cba6f7"));
        }
        Event::Press { .. } => {
            ui.get_mut(btn).unwrap().set_bg_color(Color::hex("89b4fa"));
        }
        Event::Release { .. } => {
            ui.get_mut(btn).unwrap().set_bg_color(Color::hex("cba6f7"));
        }
        Event::Click { .. } => println!("clicked!"),
        _ => {}
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
