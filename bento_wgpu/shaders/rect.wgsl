struct Screen {
  size: vec2f,
}
@group(0) @binding(0) var<uniform> screen: Screen;

struct Instance {
  @location(0) pos_size: vec4f,
  @location(1) color: vec4f,
  @location(2) radii: vec4f,
  @location(3) border_color: vec4f,
  @location(4) border_widths: vec4f,
  @location(5) transform_ab: vec4f, // a, b, c, d
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
  @location(1) local_pos: vec2f,
  @location(2) half_size: vec2f,
  @location(3) radii: vec4f,
  @location(4) border_color: vec4f,
  @location(5) border_widths: vec4f,
  @location(6) scale: f32,
}

var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0), vec2f(1.0, 0.0), vec2f(0.0, 1.0),
    vec2f(1.0, 0.0), vec2f(1.0, 1.0), vec2f(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VOut {
  let q = QUAD[vi];

  // local corner position relative to the rect center
  let half = inst.pos_size.zw * 0.5;
  let local = (q - vec2f(0.5)) * inst.pos_size.zw;

  // apply transform matrix
  let a = inst.transform_ab.x;
  let b = inst.transform_ab.y;
  let c = inst.transform_ab.z;
  let d = inst.transform_ab.w;

  // rotate/scale around center
  // translate to top left position
  let px = a * local.x + c * local.y + inst.pos_size.x + half.x;
  let py = b * local.x + d * local.y + inst.pos_size.y + half.y;

  let ndcx = (px / screen.size.x) * 2.0 - 1.0;
  let ndcy = -(py / screen.size.y) * 2.0 + 1.0;

  var out: VOut;
  out.pos = vec4f(ndcx, ndcy, 0.0, 1.0);
  out.color = inst.color;
  out.local_pos = local;
  out.half_size = half;
  out.radii = inst.radii;
  out.border_color = inst.border_color;
  out.border_widths = inst.border_widths;
  out.scale = length(vec2f(inst.transform_ab.x, inst.transform_ab.y));

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

  let aa = 0.5 / in.scale;

  let d = sdf_rect(in.local_pos, in.half_size, r);
  let alpha = clamp(-d + aa, 0.0, 1.0);
  if alpha <= 0.0 { discard; }

  let inner_half = vec2f(
    in.half_size.x - (in.border_widths.y + in.border_widths.w) * 0.5,
    in.half_size.y - (in.border_widths.x + in.border_widths.z) * 0.5,
  );
  let inner_offset = vec2f(
    (in.border_widths.w - in.border_widths.y) * 0.5,
    (in.border_widths.x - in.border_widths.z) * 0.5,
  );

  let inner_r = max(r - max(max(in.border_widths.x, in.border_widths.z),
                            max(in.border_widths.y, in.border_widths.w)), 0.0);
  let d_inner = sdf_rect(in.local_pos - inner_offset, inner_half, inner_r);
  let in_inner = clamp(-d_inner + aa, 0.0, 1.0);

  let color = mix(in.border_color, in.color, in_inner);

  return vec4f(color.rgb, color.a * alpha);
}
