#![allow(dead_code)]
#![allow(unused)]

use bento::*;

fn make_button(ui: &mut Ui, label: &str, x: f32, y: f32, w: f32, h: f32) -> SceneNodeId {
    let mut g = GroupNode::new();
    g.x = x;
    g.y = y;
    g.w = w;
    g.h = h;
    let root = ui.scene_mut().add_group(g);

    let rect = ui.scene_mut().add_rect(RectNode::new(x, y, w, h));
    let text = ui.scene_mut().add_text(TextNode::new(label, x, y, 16.0));

    if let Some(SceneNode::Text(t)) = ui.scene_mut().get_mut(text) {
        t.color = [0.0, 0.0, 0.0, 1.0];
    }

    ui.scene_mut().append(root, rect);
    ui.scene_mut().append(root, text);

    root
}

fn main() {
    let mut app = App::new();
    let mut ui = Ui::new();

    let rect = ui
        .scene_mut()
        .add_rect(RectNode::new(100.0, 100.0, 200.0, 100.0));
    let text = ui
        .scene_mut()
        .add_text(TextNode::new("Hello, world!", 100.0, 220.0, 33.0));

    if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(rect) {
        r.color = [1.0, 0.0, 0.0, 1.0];
    }

    let button = make_button(&mut ui, "Click me!", 100.0, 100.0, 200.0, 50.0);

    ui.asyncs.spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        println!("async sleep");
        move |ui: &mut Ui| {
            if let Some(SceneNode::Rect(r)) = ui.scene_mut().get_mut(button) {
                r.color = [0.0, 0.0, 1.0, 1.0];
            }
        }
    });

    app.open_window(WindowConfig::default(), ui);
    app.run();
}
