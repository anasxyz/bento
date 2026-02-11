use crate::{Fonts, InputState, MouseState};
use crate::widgets::WidgetManager;
use crate::layout::LayoutManager;

/// everything the user needs during setup and update
pub struct Ctx {
    pub widgets: WidgetManager,
    pub layout: LayoutManager,
    pub fonts: Fonts,
    pub mouse: MouseState,
    pub input: InputState,
}

impl Ctx {
    pub(crate) fn new(fonts: Fonts) -> Self {
        Self {
            widgets: WidgetManager::new(),
            layout: LayoutManager::new(),
            fonts,
            mouse: MouseState::default(),
            input: InputState::default(),
        }
    }
}
