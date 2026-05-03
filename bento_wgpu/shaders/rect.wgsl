struct Screen {
  size: vec2f,
}
@group(0) @binding(0) var<uniform> screen: Screen;

struct Instance {
  @location(0) pos_size: vec4f,
  @location(1) color: vec4f,
  @location(2) radii: vec4f,
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
  @location(1) local_pos: vec2f,
  @location(2) half_size: vec2f,
  @location(3) radii: vec4f,
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
  
  let half = inst.pos_size.zw * 0.5;
  let center = inst.pos_size.xy + half;

  var out: VOut;
  out.pos = vec4f(ndcx, ndcy, 0.0, 1.0);
  out.color = inst.color;
  out.local_pos = vec2f(px, py) - center;
  out.half_size = half;
  out.radii = inst.radii;

  return out;
}

fn sdf_rect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
  let q = abs(p) - half_size + radius;
  return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4f {
  let r = select(
    select(in.radii.x, in.radii.y, in.local_pos.x > 0.0),
    select(in.radii.w, in.radii.z, in.local_pos.x > 0.0),
    in.local_pos.y > 0.0
  );

  let d = sdf_rect(in.local_pos, in.half_size, r);
  let alpha = clamp(-d + 0.5, 0.0, 1.0);
  
  return vec4f(in.color.rgb, in.color.a * alpha);
}
