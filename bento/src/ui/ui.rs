use bento_wgpu::SceneGraph;

pub struct Ui {
    pub(crate) scene: SceneGraph,
}

impl Ui {
    pub fn new() -> Self {
        let mut scene = SceneGraph::new();
        
        let shadow = scene.add_shadow();
        scene.shadow_mut(shadow).set_rect(50.0, 50.0, 300.0, 200.0);
        scene.shadow_mut(shadow).set_color([0.0, 0.0, 0.0, 0.4]);
        scene.shadow_mut(shadow).set_blur(8.0);
        scene.shadow_mut(shadow).set_radius(0.0);
        scene.shadow_mut(shadow).set_offset(0.0, 0.0);
        scene.shadow_mut(shadow).set_visible(true);
        scene.add_child(scene.root, shadow);

        let bg = scene.add_rect();
        scene.rect_mut(bg).set_rect(50.0, 50.0, 300.0, 200.0);
        scene.rect_mut(bg).set_color([0.2, 0.3, 0.8, 1.0]);
        scene.rect_mut(bg).set_radius(8.0);
        scene.rect_mut(bg).set_visible(true);
        scene.add_child(scene.root, bg);

        Self { scene: scene }
    }
}
