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
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
  @location(1) local_pos: vec2f,
  @location(2) half_size: vec2f,
  @location(3) radii: vec4f,
  @location(4) border_color: vec4f,
  @location(5) border_widths: vec4f,
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
  out.border_color = inst.border_color;
  out.border_widths = inst.border_widths;

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
  let in_inner = clamp(-d_inner + 0.5, 0.0, 1.0);

  let color = mix(in.border_color, in.color, in_inner);

  return vec4f(color.rgb, color.a * alpha);
}
