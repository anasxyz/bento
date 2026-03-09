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

mod app;
mod color;
mod draw;
mod element;
mod fonts;
mod layout;
mod settings;
mod window;

pub use crate::{
    app::App,
    color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba},
    element::*,
    settings::WindowSettings,
};
