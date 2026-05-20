use bento_shared::{GroupNode, RectNode, Scene};

fn main() {
    let mut scene = Scene::new();

    // outer group at 100, 100
    let mut outer = GroupNode::new();
    outer.x = 100.0;
    outer.y = 100.0;
    outer.w = 300.0;
    outer.h = 300.0;
    let outer_group = scene.add_group(outer);

    // inner group at 50, 50 inside outer, with scroll offset of 20
    let mut inner = GroupNode::new();
    inner.x = 50.0;
    inner.y = 50.0;
    inner.w = 100.0;
    inner.h = 50.0;
    inner.offset_y = -20.0;
    inner.clip = Some([150.0, 150.0, 100.0, 50.0]);
    let inner_group = scene.add_group(inner);
    scene.append(outer_group, inner_group);

    // rect inside inner group at 0, 0
    let rect = scene.add_rect(RectNode::new(0.0, 0.0, 100.0, 50.0));
    scene.append(inner_group, rect);

    // hitbox of inner group — should be 150, 150 (100+50), 100x50, clipped
    let (x, y, w, h) = scene.hitbox(inner_group);
    println!("inner group hitbox: ({}, {}, {}x{})", x, y, w, h);
    // expected: (150, 150, 100, 50)

    // hitbox of rect — screen pos = 100+50+0, 100+50-20+0 = 150, 130, clipped to 150,150,100,50
    let (x, y, w, h) = scene.hitbox(rect);
    println!("rect hitbox: ({}, {}, {}x{})", x, y, w, h);
    // expected: clipped to (150, 150, 100, 30)
}
