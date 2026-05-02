// blur.wgsl — backdrop blur
//
// Two passes: horizontal then vertical Gaussian blur over a copy of the
// framebuffer. Each BlurNode gets a rounded-rect masked quad.
//
// Pass 0 (horizontal): samples the captured framebuffer texture horizontally
// Pass 1 (vertical):   samples the result of pass 0 vertically
//
// The final composited result is drawn clipped to the rounded rect.
//
// Instance layout:
//   location 0: pos_size   vec4(x, y, w, h)  physical pixels
//   location 1: params     vec4(radius, sigma, pass, _)
//   location 2: clip       vec4
//   location 3: tint       vec4

struct Screen { size: vec2<f32>, _pad: vec2<f32> }
@group(0) @binding(0) var<uniform>  screen:  Screen;
@group(0) @binding(1) var src_tex:  texture_2d<f32>;
@group(0) @binding(2) var src_smp:  sampler;

struct Instance {
    @location(0) pos_size: vec4<f32>,
    @location(1) params:   vec4<f32>,
    @location(2) clip:     vec4<f32>,
    @location(3) tint:     vec4<f32>,
}

struct VOut {
    @builtin(position) pos:      vec4<f32>,
    @location(0)       uv:       vec2<f32>,
    @location(1)       local_pos:vec2<f32>,
    @location(2)       half_size:vec2<f32>,
    @location(3)       radius:   f32,
    @location(4)       sigma:    f32,
    @location(5)       clip:     vec4<f32>,
    @location(6)       tint:     vec4<f32>,
    @location(7)       pass_dir: vec2<f32>,
}

var<private> QUAD: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0,0.0), vec2<f32>(1.0,0.0), vec2<f32>(0.0,1.0),
    vec2<f32>(1.0,0.0), vec2<f32>(1.0,1.0), vec2<f32>(0.0,1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VOut {
    let q  = QUAD[vi];
    let px = inst.pos_size.x + q.x * inst.pos_size.z;
    let py = inst.pos_size.y + q.y * inst.pos_size.w;
    let cx = inst.pos_size.x + inst.pos_size.z * 0.5;
    let cy = inst.pos_size.y + inst.pos_size.w * 0.5;
    let is_vert = inst.params.z > 0.5;

    var out: VOut;
    out.pos       = vec4<f32>(px/screen.size.x*2.0-1.0, 1.0-py/screen.size.y*2.0, 0.0, 1.0);
    out.uv        = vec2<f32>(px / screen.size.x, py / screen.size.y);
    out.local_pos = vec2<f32>(px - cx, py - cy);
    out.half_size = inst.pos_size.zw * 0.5;
    out.radius    = inst.params.x;
    out.sigma     = inst.params.y;
    out.clip      = inst.clip;
    out.tint      = inst.tint;
    out.pass_dir  = select(vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0), is_vert);
    return out;
}

fn sdf_rrect(p: vec2<f32>, half: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - half + r;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

// 9-tap Gaussian weights (sigma ~= blur/2)
fn gaussian_blur(uv: vec2<f32>, dir: vec2<f32>, sigma: f32) -> vec4<f32> {
    let step = dir / screen.size;
    var col = vec4<f32>(0.0);
    var total_w = 0.0;
    for (var i = -4; i <= 4; i++) {
        let offset = f32(i);
        let w = exp(-0.5 * (offset/sigma) * (offset/sigma));
        col     += textureSample(src_tex, src_smp, uv + step * offset) * w;
        total_w += w;
    }
    return col / total_w;
}

@fragment
fn fs_main(in: VOut) -> @location(0) vec4<f32> {
    let cl = in.clip;
    if cl.z > 0.0 || cl.w > 0.0 {
        if in.pos.x < cl.x || in.pos.y < cl.y || in.pos.x > cl.z || in.pos.y > cl.w { discard; }
    }

    let d     = sdf_rrect(in.local_pos, in.half_size, in.radius);
    let alpha = clamp(0.5 - d, 0.0, 1.0);
    if alpha <= 0.0 { discard; }

    let blurred = gaussian_blur(in.uv, in.pass_dir, max(in.sigma, 0.5));
    let result  = blurred * in.tint;
    return vec4<f32>(result.rgb, result.a * alpha);
}
