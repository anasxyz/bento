use bento::*;

fn main() {
    let mut app = App::new();
    let avatar = app.load_image_svg("/home/anas/cloud-icon.svg", 200, 200);

    let mut ui = Ui::new();

    let root = ui.add(Container::new());
    ui.get_mut(root)
        .unwrap()
        .set_width(Size::Percent(100.0))
        .set_height(Size::Percent(100.0))
        .set_align_items(AlignItems::Center)
        .set_justify_content(JustifyContent::Center);
    ui.set_root(root);

    let label = ui.add(Label::new("Hello world!"));
    ui.get_mut(label)
        .unwrap()
        .set_size(20.0)
        .set_selectable(true)
        .set_color(Color::rgb(255, 255, 2));
    ui.append(root, label);

    let img = ui.add(Image::new(avatar));
    ui.get_mut(img)
        .unwrap()
        .set_width(Size::Fixed(200.0))
        .set_height(Size::Fixed(200.0));
    ui.append(root, img);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
