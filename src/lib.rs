#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod render {
    pub mod draw_ctx;
    pub mod gpu;
    pub mod shadow_renderer;
    pub mod shape_renderer;
    pub mod text_renderer;
}

mod element {
    pub mod button;
    pub mod container;
    pub mod element;
    pub mod handle;
    pub mod label;
    pub mod layout;
    pub mod rect;
    pub mod values;
}

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

pub use crate::{
    app::AppWindow,
    color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba},
    element::button::Button,
    element::container::{Column, Container, Row},
    element::element::Element,
    element::handle::Handle,
    element::label::Label,
    element::layout::Layout,
    element::rect::Rect,
    element::values::Size,
    fonts::Fonts,
    layout::layout_tree,
    settings::WindowConfig,
    ui::Ui,
    element::values::*,
    keyboard::{Key, Modifiers},
};
