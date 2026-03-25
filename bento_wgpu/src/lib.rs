// bento_wgpu
//
// standalone gpu rendering crate for 2d ui primitives
//
// ## Architecture
//   RenderContext     owns wgpu device + queue, one per app
//   Surface           wgpu surface for one window, one per window
//   SceneGraph        owns all nodes, one per window or shared
//   Renderer          reads SceneGraph, drives gpu, one per RenderContext
//
// ## Primitive types
//   RectNode       rounded rect with border and clip   (RectId)
//   TextNode       text via glyphon/cosmic-text        (TextId)
//   ShadowNode     soft box shadow                     (ShadowId)
//
// ## Adding a new primitive type
//   add the struct + id type to nodes.rs
//   add slab + crud methods to scene.rs
//   add pipelines/newtype.rs
//   pub mod newtype in pipelines/mod.rs
//   add the pipeline field + sync logic to renderer.rs
//
// ## Usage
//   ```rust
//   let ctx  = RenderContext::new().await;
//   let mut surface  = Surface::new(&ctx, &window, width, height, scale);
//   let mut renderer = Renderer::new(&ctx, &surface);
//   let mut scene    = SceneGraph::new();
//
//   // create nodes
//   let bg = scene.add_rect();
//   scene.rect_mut(bg).set_rect(0.0, 0.0, 800.0, 600.0);
//   scene.rect_mut(bg).set_color([0.1, 0.1, 0.1, 1.0]);
//   scene.rect_mut(bg).set_visible(true);
//
//   // render loop
//   renderer.render(&ctx, &mut surface, &mut scene, [0.0, 0.0, 0.0, 1.0]);
//   ```
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

mod allocator;
mod context;
mod nodes;
mod pipelines;
mod renderer;
mod scene;
mod surface;

pub use context::RenderContext;
pub use surface::Surface;
pub use scene::SceneGraph;
pub use renderer::Renderer;

pub use nodes::{
    RectId,   RectNode,
    TextId,   TextNode,
    ShadowId, ShadowNode,
};
