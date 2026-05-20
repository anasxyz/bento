use bento_shared::SceneNodeId;

use crate::{ui::Ui, widget::WidgetHandle};
use std::any::Any;

pub trait Widget {
    fn root(&self) -> Option<SceneNodeId>;
    fn name(&self) -> &str;
    fn build(&mut self, ui: &mut Ui, handle: WidgetHandle<()>);
    fn update(&mut self, ui: &mut Ui);
    fn remove(&mut self, ui: &mut Ui);
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool);
    fn focusable(&self) -> bool { false }
}

pub trait AnyWidget: Widget + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<W: Widget + Any> AnyWidget for W {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
