// src/lib.rs

mod shapes;
mod text;
mod app;
mod scene;

pub use shapes::ShapeRenderer;
pub use text::TextRenderer;
pub use app::{App, Canvas};
pub use scene::{Scene, DrawCommand};
