use crate::widget::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{GroupNode, Node, Scene, SceneNodeId},
};

pub struct Group {
    pub offset_x: f32,
    pub offset_y: f32,
    pub opacity: Option<f32>,
    pub clip: Option<[f32; 4]>,
    children: Vec<Box<dyn Widget>>,
    id: Option<SceneNodeId>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            offset_x: 0.0,
            offset_y: 0.0,
            opacity: None,
            clip: None,
            children: Vec::new(),
            id: None,
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, widget: W) -> &mut Self {
        self.children.push(Box::new(widget));
        self
    }
}

impl Widget for Group {
    fn build(&mut self, scene: &mut Scene) {
        let mut node = GroupNode::new();
        node.offset_x = self.offset_x;
        node.offset_y = self.offset_y;
        node.opacity = self.opacity;
        node.clip = self.clip;
        let group_id = scene.add_group(node);
        self.id = Some(group_id);

        for child in &mut self.children {
            let before: Vec<SceneNodeId> = scene.root.clone();
            child.build(scene);
            let new_root_ids: Vec<SceneNodeId> = scene
                .root
                .iter()
                .copied()
                .filter(|id| !before.contains(id))
                .collect();
            for child_id in new_root_ids {
                scene.add_to_group(group_id, child_id);
            }
        }
    }

    fn update(&mut self, scene: &mut Scene, measurer: &mut dyn TextMeasurer) {
        if let Some(id) = self.id {
            if let Some(Node::Group(g)) = scene.get_mut(id) {
                g.offset_x = self.offset_x;
                g.offset_y = self.offset_y;
                g.opacity = self.opacity;
                g.clip = self.clip;
            }
        }
        for child in &mut self.children {
            child.update(scene, measurer);
        }
    }
}
