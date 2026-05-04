struct Screen {
  size: vec2f,
}
@group(0) @binding(0) var<uniform> screen: Screen;
@group(0) @binding(1) var atlas_tex: texture_2d<f32>;
@group(0) @binding(2) var atlas_smp: sampler;

struct Instance {
  @location(0) position: vec2f,
  @location(1) size: vec2f,
  @location(2) uv: vec2f,
  @location(3) uv_size: vec2f,
  @location(4) color: vec4f,
  @location(5) is_color: u32,
}

struct VOut {
  @builtin(position) pos: vec4f,
  @location(0) tex_uv: vec2f,
  @location(1) color: vec4f,
  @location(2) is_color: u32,
}

var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
  vec2f(0.0, 0.0), vec2f(1.0, 0.0), vec2f(0.0, 1.0),
  vec2f(1.0, 0.0), vec2f(1.0, 1.0), vec2f(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VOut {
  let q = QUAD[vi];
  let px = inst.position.x + q.x * inst.size.x;
  let py = inst.position.y + q.y * inst.size.y;
  let ndcx = (px / screen.size.x) * 2.0 - 1.0;
  let ndcy = -(py / screen.size.y) * 2.0 + 1.0;

  var out: VOut;
  out.pos = vec4f(ndcx, ndcy, 0.0, 1.0);
  out.tex_uv = inst.uv + q * inst.uv_size;
  out.color = inst.color;
  out.is_color = inst.is_color;
  return out;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4f {
  let sample = textureSample(atlas_tex, atlas_smp, in.tex_uv);

  if in.is_color == 1u {
      return vec4f(sample.rgb * sample.a, sample.a);
  } else {
      let alpha = sample.r * in.color.a;
      return vec4f(in.color.rgb * alpha, alpha);
  }
}
