#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui.add(Rect::new(100.0, 100.0, 100.0, 100.0));

    let text = ui.add(Text::new("Hello, world!", 100.0, 100.0, 32.0));
    if let Some(t) = ui.get_mut(text) {
        t.set_font_family("JetBrainsMono Nerd Font");
        t.add_background(0, t.text().len(), [0.2, 0.5, 0.9, 0.3]);
    }

    ui.set_children(rect, [text]);

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
