#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let toggle = ui.add(Button::new("Select..."));
    ui.set(toggle, |b| {
        b.x = 200.0;
        b.y = 200.0;
    });

    let mut options = Group::new();
    options.x = 200.0;
    options.y = 232.0;
    options.layout = Layout::Column { gap: 0.0 };
    options.background = Some([0.2, 0.2, 0.2, 1.0]);
    options.visible = false;
    let options = ui.add(options);

    let opt1 = ui.add(Button::new("Option A"));
    let opt2 = ui.add(Button::new("Option B"));
    let opt3 = ui.add(Button::new("Option C"));
    ui.append(options, opt1);
    ui.append(options, opt2);
    ui.append(options, opt3);

    ui.listen(toggle, move |_: &Click, ui: &mut Ui| {
        ui.set(options, |g| g.visible = !g.visible);
    });

    ui.listen(opt1, move |_: &Click, ui: &mut Ui| {
        ui.set(toggle, |b| b.label_text = "Option A".to_string());
        ui.set(options, |g| g.visible = false);
    });
    ui.listen(opt2, move |_: &Click, ui: &mut Ui| {
        ui.set(toggle, |b| b.label_text = "Option B".to_string());
        ui.set(options, |g| g.visible = false);
    });
    ui.listen(opt3, move |_: &Click, ui: &mut Ui| {
        ui.set(toggle, |b| b.label_text = "Option C".to_string());
        ui.set(options, |g| g.visible = false);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
