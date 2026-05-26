use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut panel = Group::new();
    panel.layout = Layout::Column { gap: 8.0 };
    panel.x = 200.0;
    panel.y = 200.0;
    panel.draggable = true;
    panel.scrollable = true;
    panel.width = Size::Fixed(300.0);
    panel.height = Size::Fixed(300.0);
    panel.background = Some([0.15, 0.15, 0.15, 1.0]);
    let panel = ui.add(panel);

    let label = ui.add(Text::new("Drag me"));
    let btn1 = ui.add(Button::new("Button A"));
    let btn2 = ui.add(Button::new("Button B"));
    ui.listen(btn1, |ev: &MouseDown, ui: &mut Ui| {
        println!("btn1 down");
    });
    ui.append(panel, label);
    ui.append(panel, btn1);
    ui.append(panel, btn2);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
