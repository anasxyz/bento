use std::any::Any;

use bento_shared::Scene;

use crate::Ui;

// TODO: make it automatically implemented for all widgets by deriving `Widget`
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Allows widgets to be used in the Ui
pub trait Widget: AsAny {
    /// Allows widgets to define their name
    fn name(&self) -> &str;

    /// Allows widgets to build their SceneNode(s)
    fn build(&mut self, scene: &mut Scene);

    /// Allows widgets to update their SceneNode(s)
    fn update(&mut self, scene: &mut Scene);

    /// Allows widgets to remove their SceneNode(s)
    fn remove(&mut self, scene: &mut Scene);

    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool);
}
