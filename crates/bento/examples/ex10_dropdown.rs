#![allow(unused)]
#![allow(dead_code)]
use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let label = ui.add(Text::new("Choose an option:"));
    ui.set(label, |t| {
        t.set_x(200.0);
        t.set_y(170.0);
    });

    let dropdown = ui.build(Dropdown::new("Select..."), |ui, dropdown| {
        ui.set(dropdown, |d| {
            d.x = 200.0;
            d.y = 200.0;
        });

        let og = ui.get(dropdown).unwrap().options_group;

        let opt1 = ui.add(Button::new("Option A"));
        ui.set(opt1, |b| {
            b.width = Size::Fill;
            b.z = 100;
        });
        let opt2 = ui.add(Button::new("Option B"));
        ui.set(opt2, |b| {
            b.width = Size::Fill;
            b.z = 100;
        });
        let opt3 = ui.add(Button::new("Option C"));
        ui.set(opt3, |b| {
            b.width = Size::Fill;
            b.z = 100;
        });

        ui.append(og, opt1);
        ui.append(og, opt2);
        ui.append(og, opt3);

        ui.listen(opt1, move |_: &Click, ui: &mut Ui| {
            ui.set(dropdown, |d| d.label = "Option A".to_string());
        });
        ui.listen(opt2, move |_: &Click, ui: &mut Ui| {
            ui.set(dropdown, |d| d.label = "Option B".to_string());
        });
        ui.listen(opt3, move |_: &Click, ui: &mut Ui| {
            ui.set(dropdown, |d| d.label = "Option C".to_string());
        });
    });

    let result = ui.add(Text::new("Nothing selected yet"));
    ui.set(result, |t| {
        t.set_x(200.0);
        t.set_y(260.0);
    });

    let confirm = ui.add(Button::new("Confirm"));
    ui.set(confirm, |b| {
        b.x = 200.0;
        b.y = 300.0;
    });
    ui.listen(confirm, move |_: &Click, ui: &mut Ui| {
        let selected = ui
            .get(dropdown)
            .map(|d| d.label.clone())
            .unwrap_or_default();
        ui.set(result, |t| {
            t.set_content(&format!("You chose: {}", selected))
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
