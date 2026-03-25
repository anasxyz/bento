#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod app;
mod runner;
mod window;
mod color;
mod settings;

pub use crate::app::AppWindow;
pub use crate::color::{Color, hex, hsl, hsla, hwb, hwba, rgb, rgba};
pub use crate::settings::WindowConfig;
