use std::any::Any;

use bento_shared::Scene;
use bento_shared::SceneNodeId;
use bento_shared::TextMeasurer;

use crate::ui::Ui;

// TODO: make it automatically implemented for all widgets by deriving `Widget`
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Allows widgets to be used in the Ui
pub trait Widget: AsAny {}
