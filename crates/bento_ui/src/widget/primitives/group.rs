use crate::widget::Widget;
use crate::widget::{Base, HasBase};
use bento_macros::Widget;
use bento_shared::{
    TextMeasurer,
    scene::{GroupNode, Node, Scene, SceneNodeId},
};

#[derive(Widget)]
pub struct Group {
    pub base: Base,
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
            base: Base::new(),
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

        scene.push_parent(group_id);
        for child in &mut self.children {
            child.build(scene);
        }
        scene.pop_parent();
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
