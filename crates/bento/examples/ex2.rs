#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let mut g = GroupNode::new();
    g.offset_x = 100.0;
    g.offset_y = 100.0;
    g.x = 100.0;
    g.y = 100.0;
    g.w = 300.0;
    g.h = 40.0;
    let root = ui.scene_mut().add_group(g);

    let mut bg = RectNode::new(0.0, 0.0, 300.0, 40.0);
    bg.color = [0.15, 0.15, 0.15, 1.0];
    bg.border_color = [0.4, 0.4, 0.4, 1.0];
    bg.border_widths = [1.0; 4];
    bg.radii = [4.0; 4];
    let bg_id = ui.scene_mut().add_rect(bg);
    ui.scene_mut().append(root, bg_id);

    let text = ui
        .scene_mut()
        .add_text(TextNode::new("Type here...", 8.0, 12.0, 14.0));
    ui.scene_mut().append(root, text);

    ui.listen(root, move |e: &Click, ui| {
        ui.set_focused(root);
    });

    ui.listen(root, move |e: &KeyPress, ui| {
        if ui.focused() != Some(root) {
            return;
        }
        println!("key: {:?}", e.key);
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
