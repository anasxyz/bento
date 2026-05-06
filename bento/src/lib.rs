#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

// app stuff
mod app;
mod window;

// main
mod ui;
mod input;
mod fonts;
mod widget;

// other
mod color;

pub use app::App;
pub use window::WindowConfig;
pub use ui::Ui;
pub use color::Color;
