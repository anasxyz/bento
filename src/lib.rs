#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render;
mod element;
mod app;
mod color;
mod draw;
mod events;
mod fonts;
mod layout;
mod mouse;
mod keyboard;
mod settings;
mod ui;
mod window;

pub use crate::app::AppWindow;
pub use crate::color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba};
pub use crate::element::{
    element::Element,
    button::Button,
    container::{Column, Container, Row},
    text_input::TextInput,
    handle::Handle,
    label::Label,
    layout::Layout,
    rect::Rect,
    values::*,
};

pub use crate::{
    fonts::Fonts,
    layout::layout_tree,
    settings::WindowConfig,
    ui::Ui,
    keyboard::{Key, Modifiers},
};
