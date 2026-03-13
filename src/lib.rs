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
    pub mod element;
    pub mod values;
    pub mod layout;
    pub mod rect;
    pub mod label;
    pub mod container;
    pub mod handle;
}

mod app;
mod color;
mod draw;
mod fonts;
mod layout;
mod settings;
mod window;
mod mouse;
mod ui;

pub use crate::{
    app::AppWindow,
    color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba},
    settings::WindowConfig,
    element::{element::Element, layout::Layout, rect::Rect, label::Label, container::*},
    ui::Ui,
};
