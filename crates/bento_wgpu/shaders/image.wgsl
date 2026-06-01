struct Screen {
  size: vec2f,
  // padding for webgl bc it requires buffer bindings to be 16 byte aligned
  _pad: vec2f,
}
@group(0) @binding(0) var<uniform> screen: Screen;
@group(1) @binding(0) var image_tex: texture_2d<f32>;
@group(1) @binding(2) var image_smp: sampler;

struct Instance {
  @location(0) pos_size: vec4f,
  @location(1) radii: vec4f,
  @location(2) border_color: vec4f,
  @location(3) border_widths: vec4f,
  @location(4) transform: vec4f,
  @location(5) clip: vec4f,
  @location(6) opacity_pad: vec4f,
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) uv: vec2f,
  @location(1) local_pos: vec2f,
  @location(2) half_size: vec2f,
  @location(3) radii: vec4f,
  @location(4) border_color: vec4f,
  @location(5) border_widths: vec4f,
  @location(6) clip: vec4f,
  @location(7) opacity: f32,
  @location(8) scale: f32,
}

var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
  vec2f(0.0, 0.0), vec2f(1.0, 0.0), vec2f(0.0, 1.0),
  vec2f(1.0, 0.0), vec2f(1.0, 1.0), vec2f(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VOut {
  let q = QUAD[vi];
  let half = inst.pos_size.zw * 0.5;
  let local = (q - vec2f(0.5)) * inst.pos_size.zw;

  let a = inst.transform.x;
  let b = inst.transform.y;
  let c = inst.transform.z;
  let d = inst.transform.w;

  let px = a * local.x + c * local.y + inst.pos_size.x + half.x;
  let py = b * local.x + d * local.y + inst.pos_size.y + half.y;

  let ndcx =  (px / screen.size.x) * 2.0 - 1.0;
  let ndcy = -(py / screen.size.y) * 2.0 + 1.0;

  var out: VOut;
  out.pos = vec4f(ndcx, ndcy, 0.0, 1.0);
  out.uv = q;
  out.local_pos = local;
  out.half_size = half;
  out.radii = inst.radii;
  out.border_color = inst.border_color;
  out.border_widths= inst.border_widths;
  out.clip = inst.clip;
  out.opacity = inst.opacity_pad.x;
  out.scale = length(vec2f(inst.transform.x, inst.transform.y));

  return out;
}

fn sdf_rect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
  let q = abs(p) - half_size + radius;
  return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4f {
  // discard if outside clip
  if in.pos.x < in.clip.x || in.pos.y < in.clip.y ||
     in.pos.x > in.clip.x + in.clip.z || in.pos.y > in.clip.y + in.clip.w {
    discard;
  }

  // rounded corners
  let r = select(
    select(in.radii.x, in.radii.y, in.local_pos.x > 0.0),
    select(in.radii.w, in.radii.z, in.local_pos.x > 0.0),
    in.local_pos.y > 0.0
  );
  let aa = 0.5 / in.scale;
  let d = sdf_rect(in.local_pos, in.half_size, r);
  let alpha = clamp(-d + aa, 0.0, 1.0);
  if alpha <= 0.0 { discard; }

  // sample image
  let color = textureSample(image_tex, image_smp, in.uv);

  // border
  let has_border = (in.border_widths.x + in.border_widths.y +
                    in.border_widths.z + in.border_widths.w) > 0.0;
  var out_color: vec4f;
  if !has_border {
    out_color = vec4f(color.rgb * color.a, color.a) * in.opacity * alpha;
  } else {
    let bw = (in.border_widths.x + in.border_widths.y +
                   in.border_widths.z + in.border_widths.w) * 0.25;
    let inner_r = max(r - bw, 0.0);
    let d_inner = sdf_rect(in.local_pos, in.half_size - bw, inner_r);
    let inner_aa = 0.5;
    if d_inner >= inner_aa {
      out_color = vec4f(in.border_color.rgb, in.border_color.a * alpha) * in.opacity;
    } else if d_inner <= -inner_aa {
      out_color = vec4f(color.rgb * color.a, color.a) * in.opacity * alpha;
    } else {
      let t = (d_inner + inner_aa) / (2.0 * inner_aa);
      out_color = vec4f(
        mix(color.rgb * color.a, in.border_color.rgb, t),
        mix(color.a, in.border_color.a, t) * alpha
      ) * in.opacity;
    }
  }

  if out_color.a <= 0.0 { discard; }
  return out_color;
}
