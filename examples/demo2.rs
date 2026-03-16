use bento::*;
use std::cell::Cell;

fn main() {
    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center)
        .set_row_gap(20.0)
        .set_flex_direction(FlexDirection::Col)
        .set_bg_color(Some(Color::hex("181825")));
    ui.set_root(root);

    let hover_rect = ui.add(Rect::new());
    ui.get_mut(hover_rect)
        .unwrap()
        .set_width(Size::Fixed(100.0))
        .set_height(Size::Fixed(100.0))
        .set_bg_color(Color::hex("313244"));
    ui.append(root, hover_rect);

    ui.connect(hover_rect, move |ui, event| match event {
        Event::Hover => {
            println!("rect hovered");
            ui.get_mut(hover_rect)
                .unwrap()
                .set_bg_color(Color::hex("89b4fa"));
        }
        Event::HoverEnd => {
            println!("rect hover end");
            ui.get_mut(hover_rect)
                .unwrap()
                .set_bg_color(Color::hex("313244"));
        }
        Event::Click { x, y } => println!("rect clicked at {x:.1}, {y:.1}"),
        Event::RightClick { x, y } => println!("rect right clicked at {x:.1}, {y:.1}"),
        Event::DoubleClick { x, y } => println!("rect double clicked at {x:.1}, {y:.1}"),
        Event::Press { .. } => println!("rect pressed"),
        Event::Release { .. } => println!("rect released"),
        Event::FocusGained => println!("rect focused"),
        Event::FocusLost => println!("rect focus lost"),
        _ => {}
    });

    let drag_rect = ui.add(Rect::new());
    ui.get_mut(drag_rect)
        .unwrap()
        .set_width(Size::Fixed(40.0))
        .set_height(Size::Fixed(40.0))
        .set_bg_color(Color::hex("a6e3a1"))
        .set_position(Position::Absolute)
        .set_z_index(100);
    ui.append(root, drag_rect);

    let dragging = std::rc::Rc::new(Cell::new(false));
    let dragging2 = dragging.clone();

    ui.connect(drag_rect, move |_ui, event| match event {
        Event::Press { .. } => {
            println!("drag rect pressed");
            dragging.set(true);
        }
        Event::Release { .. } => {
            println!("drag rect released");
            dragging.set(false);
        }
        _ => {}
    });

    ui.connect(ui.global(), move |ui, event| match event {
        Event::MouseMove { x, y } => {
            if dragging2.get() {
                ui.get_mut(drag_rect).unwrap().set_inset([
                    Size::Fixed(y - 20.0),
                    Size::Auto,
                    Size::Auto,
                    Size::Fixed(x - 20.0),
                ]);
            }
        }
        Event::Release { .. } => dragging2.set(false),
        _ => {}
    });

    let key_rect = ui.add(Rect::new());
    ui.get_mut(key_rect)
        .unwrap()
        .set_width(Size::Fixed(100.0))
        .set_height(Size::Fixed(100.0))
        .set_bg_color(Color::hex("cba6f7"));
    ui.append(root, key_rect);

    ui.connect(key_rect, move |ui, event| match event {
        Event::FocusGained => {
            println!("key rect focused — now press keys");
            ui.get_mut(key_rect)
                .unwrap()
                .set_bg_color(Color::hex("f38ba8"));
        }
        Event::FocusLost => {
            println!("key rect focus lost");
            ui.get_mut(key_rect)
                .unwrap()
                .set_bg_color(Color::hex("cba6f7"));
        }
        Event::KeyPress { key, text, .. } => {
            println!("key pressed on focused rect: {:?} text: {:?}", key, text);
        }
        _ => {}
    });

    ui.connect(ui.global(), |_ui, event| match event {
        Event::KeyPress {
            key: Key::Escape, ..
        } => println!("GLOBAL: escape"),
        Event::KeyPress { key: Key::Tab, .. } => println!("GLOBAL: tab"),
        _ => {}
    });

    let conn = ui.connect(ui.global(), |_ui, event| {
        if let Event::Custom(99) = event {
            println!("custom event 99 received");
        }
    });

    ui.connect(ui.global(), move |ui, event| match event {
        Event::KeyPress { key: Key::E, .. } => {
            println!("emitting custom event 99");
            ui.emit(ui.global(), Event::Custom(99));
        }
        Event::KeyPress { key: Key::D, .. } => {
            println!("disconnecting custom event handler");
            ui.disconnect(conn);
        }
        _ => {}
    });

    AppWindow::new(WindowConfig::default()).run(ui, |_ui| {});
}
