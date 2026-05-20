use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;

use bento_shared::Scene;

use crate::input::InputState;
use crate::ui::asyncs::AsyncEventQueue;
use crate::widget::Widget;

pub struct Ui {
    pub scene: Scene,
    pub input: InputState,
    pub asyncs: AsyncEventQueue,
    pub needs_redraw: bool,
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

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    pub fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    pub fn process_input(&mut self) {}

    pub fn add<W: Widget>(&mut self, mut widget: W) -> W {
        widget.build(self);
        self.request_redraw();
        widget
    }

    pub fn remove<W: Widget>(&mut self, mut widget: W) {
        widget.remove(self);
        self.request_redraw();
    }
}
