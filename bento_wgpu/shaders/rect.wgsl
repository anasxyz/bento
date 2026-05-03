struct Screen {
  size: vec2f,
}
@group(0) @binding(0) var<uniform> screen: Screen;

struct Instance {
  @location(0) pos_size: vec4f,
  @location(1) color: vec4f,
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
}

var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0), vec2f(1.0, 0.0), vec2f(0.0, 1.0),
    vec2f(1.0, 0.0), vec2f(1.0, 1.0), vec2f(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VOut {
  let q = QUAD[vi];
  let px = inst.pos_size.x + q.x * inst.pos_size.z;
  let py = inst.pos_size.y + q.y * inst.pos_size.w;
  let ndcx = (px / screen.size.x) * 2.0 - 1.0;
  let ndcy = -(py / screen.size.y) * 2.0 + 1.0;
  return VOut(vec4f(ndcx, ndcy, 0.0, 1.0), inst.color);
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4f {
  return in.color;
}
