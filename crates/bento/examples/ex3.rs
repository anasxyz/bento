use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    for row in 0..5 {
        for col in 0..5 {
            let x = 20.0 + col as f32 * 130.0;
            let y = 20.0 + row as f32 * 60.0;
            let label = format!("Btn {},{}", row, col);
            let btn = ui.add(Button::new(&label, x, y, 120.0, 50.0));
            ui.listen(btn, move |_e: &Click, _ui| {
                println!("clicked {},{}", row, col);
            });
            ui.listen(btn, move |_e: &HoverEnter, _ui| {
                println!("hover enter {},{}", row, col);
            });
        }
    }

    for i in 0..10 {
        let t = ui.add(Text::new(
            &format!("Label {}", i),
            20.0 + i as f32 * 65.0,
            340.0,
            14.0,
        ));
        ui.listen(t, move |_e: &Click, _ui| {
            println!("text {} clicked", i);
        });
    }

    let counter_text = ui.add(Text::new("Count: 0", 20.0, 400.0, 20.0));
    let mut count = 0;
    let counter_btn = ui.add(Button::new("Increment", 20.0, 430.0, 120.0, 40.0));
    ui.listen(counter_btn, move |_e: &Click, ui| {
        count += 1;
        if let Some(t) = ui.get_mut(counter_text) {
            t.set_text(&format!("Count: {}", count));
        }
    });

    let mut is_on = false;
    let toggle = ui.add(Button::new("OFF", 160.0, 430.0, 120.0, 40.0));
    ui.listen(toggle, move |_e: &Click, ui| {
        is_on = !is_on;
        if let Some(b) = ui.get_mut(toggle) {
            b.set_label(if is_on { "ON" } else { "OFF" });
            b.set_color(if is_on {
                [0.0, 0.6, 0.2, 1.0]
            } else {
                [0.2, 0.2, 0.2, 1.0]
            });
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
