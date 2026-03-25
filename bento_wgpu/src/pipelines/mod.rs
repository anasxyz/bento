// pipelines/mod.rs
//
// Each submodule is a completely self-contained GPU pipeline.
// They share nothing with each other except the wgpu device/queue/format
// passed in at construction time.
//
// Adding a new primitive:
//   1. Create pipelines/newtype.rs following the same pattern
//   2. pub mod newtype; here
//   3. Add it to Renderer in renderer.rs

pub mod rect;
pub mod text;
pub mod shadow;
