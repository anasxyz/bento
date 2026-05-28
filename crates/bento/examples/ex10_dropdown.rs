#![allow(unused)]
#![allow(dead_code)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let dropdown = ui.add(Dropdown::new("Dropdown"));
    ui.set(dropdown, |d| {
        d.x = 200.0;
        d.y = 200.0;
    });
    let option1 = ui.add(Button::new("Option 1"));
    ui.set(option1, |b| {
        b.width = Size::Fill;
        b.height = Size::Fixed(32.0);
    });
    let option2 = ui.add(Button::new("Option 2"));
    ui.set(option2, |b| {
        b.width = Size::Fill;
        b.height = Size::Fixed(32.0);
    });
    let option3 = ui.add(Button::new("Option 3"));
    ui.set(option3, |b| {
        b.width = Size::Fill;
        b.height = Size::Fixed(32.0);
    });

    let options = ui.get_mut(dropdown).unwrap().options;
    ui.append(options, option1);
    ui.append(options, option2);
    ui.append(options, option3);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
