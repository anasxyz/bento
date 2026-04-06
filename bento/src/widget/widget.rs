use crate::fonts::Fonts;
use crate::widget::base::HasBase;
use bento_wgpu::{SceneGraph, SceneNodeId};
use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Widget: HasBase + AsAny + Any + 'static {
    /// create scene nodes
    /// called before the widget is inserted into the slot
    fn build(&mut self, scene: &mut SceneGraph);

    /// register internal event connections
    /// called after the widget is inserted into the slot, so handle is valid
    /// default is no op, only override if widget has internal behaviour to register
    fn register(&mut self, _handle: Handle<()>, _ui: &mut crate::Ui) {}

    /// called every frame after layout
    fn sync(&mut self, scene: &mut SceneGraph);

    /// which scene node children should attach to
    fn children_attachment_node(&self) -> Option<SceneNodeId> {
        None
    }

    /// returns true if this widget captures press/release events
    fn is_interactive(&self) -> bool {
        false
    }

    fn measure(&mut self, _fonts: &mut Fonts, _max_width: Option<f32>) -> Option<(f32, f32)> {
        None
    }

    fn has_measure(&self) -> bool {
        false
    }
}

pub type AnyWidget = Box<dyn Widget>;

use crate::widget::Handle;
