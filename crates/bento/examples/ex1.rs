use bento::*;

struct State {
    count: i32,
    label: WidgetHandle<Text>,
}

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut group = Group::new();
    group.layout = Layout::Row { gap: 16.0 };
    group.x = 100.0;
    group.y = 100.0;
    let group = ui.add(group);

    let label = ui.add(Text::new("0"));
    let btn_inc = ui.add(Button::new("+"));
    let btn_dec = ui.add(Button::new("-"));

    ui.append(group, btn_dec);
    ui.append(group, label);
    ui.append(group, btn_inc);

    ui.set_state(State { count: 0, label });

    ui.listen(btn_inc, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| {
            s.count += 1;
            ui.set(s.label, |t: &mut Text| t.set_content(format!("{}", s.count).as_str()));
        });
    });

    ui.listen(btn_dec, move |_: &Click, ui: &mut Ui| {
        ui.with_state(|s: &mut State, ui: &mut Ui| {
            s.count -= 1;
            ui.set(s.label, |t: &mut Text| t.set_content(format!("{}", s.count).as_str()));
        });
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
