use bento_shared::scene::{Scene, RectNode};

fn main() {
    let mut scene = Scene::new();
    let rect = RectNode::new(0.0, 0.0, 100.0, 100.0);
    scene.add_rect(rect);
    println!("node bounds: {:?}", scene.screen_bounds(rect.id, 0.0, 0.0, 100.0, 100.0));
}
