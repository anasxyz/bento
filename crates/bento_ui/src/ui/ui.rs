use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::Scene;

use crate::input::InputState;
use crate::ui::asyncs::AsyncEventQueue;

pub struct Ui {
    scene: Scene,

    input: InputState,
    pub asyncs: AsyncEventQueue,

    needs_redraw: bool,
}

impl Ui {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            input: InputState::new(),
            asyncs: AsyncEventQueue::new(),
            needs_redraw: false,
        }
    }
}
