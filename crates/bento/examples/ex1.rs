use bento::*;
use std::time::Duration;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let root = ui.root();
    ui.set(root, |g: &mut Group| {
        g.layout = Layout::Row {
            gap: 8.0,
            padding: [16.0, 16.0, 16.0, 16.0],
            main_axis: MainAxis::Start,
            cross_axis: CrossAxis::Start,
            wrap: true,
        };
        g.width = Size::Fill;
        g.height = Size::Fill;
    });

    let mut middle_btn = None;

    for i in 0..9 {
        let btn = ui.add(root, Button::new(&format!("Button {}", i)));
        if i == 4 {
            middle_btn = Some(btn);
        }
    }

    let middle_btn = middle_btn.unwrap();

    ui.asyncs.spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        move |ui: &mut Ui| {
            ui.set(middle_btn, |b: &mut Button| b.set_text("Changeddddddddddddddddddddddddddddd!"));
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
