// bento_render
//
// A standalone GPU rendering crate for 2D UI primitives.
// No coupling to any UI framework, window system, or element model.
//
// ## Architecture
//
//   RenderContext   — owns wgpu Device + Queue. One per app.
//   Surface         — wgpu surface for one window. One per window.
//   SceneGraph      — owns all nodes (pure CPU). One per window or shared.
//   Renderer        — reads SceneGraph, drives GPU. One per RenderContext.
//
// ## Primitive types
//
//   RectNode    — rounded rect with border and clip  (RectId)
//   TextNode    — text via glyphon/cosmic-text        (TextId)
//   ShadowNode  — soft box shadow                     (ShadowId)
//
// ## Adding a new primitive type
//
//   1. Add the struct + Id type to nodes.rs
//   2. Add Slab + CRUD methods to scene.rs
//   3. Add pipelines/newtype.rs
//   4. pub mod newtype in pipelines/mod.rs
//   5. Add the pipeline field + sync logic to renderer.rs
//
// ## Usage
//
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

mod allocator;
mod context;
mod nodes;
mod pipelines;
mod renderer;
mod scene;
mod surface;

// ── public re-exports ─────────────────────────────────────────────────────────

pub use context::RenderContext;
pub use surface::Surface;
pub use scene::SceneGraph;
pub use renderer::Renderer;

pub use nodes::{
    RectId,   RectNode,
    TextId,   TextNode,
    ShadowId, ShadowNode,
};
