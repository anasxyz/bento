#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let slider = ui.add(Slider::new(100.0, 100.0, 300.0, 20.0));
    ui.listen(slider, |e: &SliderChanged, ui| {
        println!("slider value: {}", e.value);
    });

    let slider2 = ui.add(Slider::new(20.0, 0.0, 300.0, 20.0));
    ui.listen(slider2, |e: &SliderChanged, ui| {
        println!("slider2 value: {}", e.value);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
