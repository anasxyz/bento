// shadow.wgsl
// Instance-based box shadow with affine transform support.
//
// The shadow quad is expanded by (blur * 2) in local space so the gaussian
// fringe always fits. The SDF distance is computed in local space so the
// gaussian falloff remains correct even on rotated shadows.
//
// Instance layout (matches shadow.rs Instance struct):
//   location 0: col01  — vec4(a, b, c, d)
//   location 1: trans  — vec4(tx, ty, pw, ph)
//   location 2: color  — vec4
//   location 3: params — vec4(corner_radius, blur, offset_x, offset_y)
//                        all values already in physical pixels

struct Screen { size: vec2<f32> }
@group(0) @binding(0) var<uniform> screen: Screen;

struct Instance {
    @location(0) col01:  vec4<f32>,
    @location(1) trans:  vec4<f32>,
    @location(2) color:  vec4<f32>,
    @location(3) params: vec4<f32>,
}

struct VertexOut {
    @builtin(position) pos:        vec4<f32>,
    @location(0)       local_pos:  vec2<f32>,  // relative to shadow rect center, local space
    @location(1)       color:      vec4<f32>,
    @location(2)       half_size:  vec2<f32>,
    @location(3)       corner_radius: f32,
    @location(4)       blur:       f32,
}

var<private> CORNERS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 1.0),
);

@vertex
fn vs_main(@builtin(vertex_index) vi: u32, inst: Instance) -> VertexOut {
    let blur     = inst.params.y;
    let offset_x = inst.params.z;
    let offset_y = inst.params.w;
    let pw       = inst.trans.z;
    let ph       = inst.trans.w;

    // Expand quad by blur in local space so fringe fits
    let expand = blur * 2.0;
    let lx0 = -expand + offset_x;
    let ly0 = -expand + offset_y;
    let lw  = pw + expand * 2.0;
    let lh  = ph + expand * 2.0;

    let corner = CORNERS[vi];
    let lx = lx0 + corner.x * lw;
    let ly = ly0 + corner.y * lh;

    // Apply 2x3 affine transform
    let px = inst.col01.x * lx + inst.col01.z * ly + inst.trans.x;
    let py = inst.col01.y * lx + inst.col01.w * ly + inst.trans.y;

    // local_pos for SDF: relative to shadow rect center (in local space,
    // accounting for the shadow offset so the gaussian is centred correctly)
    let cx = pw * 0.5 + offset_x;
    let cy = ph * 0.5 + offset_y;
    let local_pos = vec2<f32>(lx - cx, ly - cy);

    var out: VertexOut;
    out.pos          = vec4<f32>(px / screen.size.x * 2.0 - 1.0,
                                 1.0 - py / screen.size.y * 2.0, 0.0, 1.0);
    out.local_pos    = local_pos;
    out.color        = inst.color;
    out.half_size    = vec2<f32>(pw * 0.5, ph * 0.5);
    out.corner_radius = inst.params.x;
    out.blur         = blur;
    return out;
}

fn rounded_rect_sdf(p: vec2<f32>, half_size: vec2<f32>, radius: f32) -> f32 {
    let q = abs(p) - half_size + vec2<f32>(radius, radius);
    return length(max(q, vec2<f32>(0.0, 0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn gaussian(x: f32, sigma: f32) -> f32 {
    return exp(-0.5 * (x / sigma) * (x / sigma));
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let sdf   = rounded_rect_sdf(in.local_pos, in.half_size, in.corner_radius);
    let sigma = max(in.blur * 0.5, 0.0001);
    let alpha = gaussian(sdf, sigma) * in.color.a;
    return vec4<f32>(in.color.rgb, alpha);
}
