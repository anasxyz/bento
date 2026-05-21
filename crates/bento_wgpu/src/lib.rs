#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod context;
mod surface;
mod renderer;
mod pipelines;

pub use context::RenderContext;
pub use surface::Surface;
pub use renderer::Renderer;
use pipelines::rect::RectInstance;

use bento_shared::{
    Scene, SceneNode, RectNode, TextNode, ImageNode, GroupNode, TextAlign,
};
