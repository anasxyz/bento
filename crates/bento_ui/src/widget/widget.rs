use std::any::Any;

use bento_shared::Scene;
use bento_shared::TextMeasurer;

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

    fn set_handle(&mut self, id: u32, generation: u32) {}

    /// Allows widgets to build their SceneNode(s)
    fn build(&mut self, ui: &mut Ui);

    /// Allows widgets to update their SceneNode(s)
    fn update(&mut self, ui: &mut Ui, measurer: &mut dyn TextMeasurer);

    /// Allows widgets to remove their SceneNode(s)
    fn remove(&mut self, ui: &mut Ui);

    /// Allows widgets to update their dirty flag
    fn is_dirty(&self) -> bool;
    fn set_dirty(&mut self, dirty: bool);

    /// Allows widgets to update their focus "settings"
    fn focusable(&self) -> bool {
        false
    }
    fn is_focused(&self) -> bool {
        false
    }
    fn set_focused(&mut self, focused: bool) {}

    /// Allows widgets to update their hover "settings"
    fn hoverable(&self) -> bool {
        false
    }
    fn is_hovered(&self) -> bool {
        false
    }
    fn set_hovered(&mut self, hovered: bool) {}

    /// Allows widgets to specify their bounds
    fn bounds(&self) -> (f32, f32, f32, f32) {
        (0.0, 0.0, 0.0, 0.0)
    }
}
