// rect.wgsl — instanced rounded rect with affine transform + gradient fills
//
// Instance layout (matches rect.rs Instance struct):
//   location 0: col01          vec4(a, b, c, d)        2x2 rotation+scale
//   location 1: trans          vec4(tx, ty, pw, ph)    translation + physical size
//   location 2: fill_color     vec4
//   location 3: border_color   vec4
//   location 4: clip           vec4  (physical px; all-zero = no clip)
//   location 5: border_widths  vec4  (top, right, bottom, left  physical px)
//   location 6: params         vec4  (radius, aa_width, _, _)
//   location 7: grad_color0    vec4  (start colour; alpha=0 means solid fill)
//   location 8: grad_color1    vec4  (end colour)
//   location 9: grad_params    vec4  (cos_a, sin_a, _, _)  gradient angle unit vector

struct Screen { size: vec2f }
@group(0) @binding(0) var<uniform> screen: Screen;

var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0), vec2f(1.0, 0.0), vec2f(0.0, 1.0),
    vec2f(1.0, 0.0), vec2f(1.0, 1.0), vec2f(0.0, 1.0),
);

struct VertexOut {
    @builtin(position) frag_coord  : vec4f,
    @location(0)       local_pos   : vec2f,
    @location(1)       half_size   : vec2f,
    @location(2)       radius      : f32,
    @location(3)       aa_width    : f32,
    @location(4)       fill_color  : vec4f,
    @location(5)       border_color: vec4f,
    @location(6)       clip        : vec4f,
    @location(7)       border_widths: vec4f,
    @location(8)       grad_color0 : vec4f,
    @location(9)       grad_color1 : vec4f,
    @location(10)      grad_dir    : vec2f,   // unit vector along gradient
    @location(11)      grad_local  : vec2f,   // local_pos for gradient (pre-transform)
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi: u32,
    @location(0) col01:          vec4f,
    @location(1) trans:          vec4f,
    @location(2) fill_color:     vec4f,
    @location(3) border_color:   vec4f,
    @location(4) clip:           vec4f,
    @location(5) border_widths:  vec4f,
    @location(6) params:         vec4f,
    @location(7) grad_color0:    vec4f,
    @location(8) grad_color1:    vec4f,
    @location(9) grad_params:    vec4f,
) -> VertexOut {
    let pw = trans.z; let ph = trans.w;
    let radius   = params.x;
    let aa_width = params.y;
    let b  = aa_width;
    let q  = QUAD[vi];
    let lx = -b + q.x * (pw + b * 2.0);
    let ly = -b + q.y * (ph + b * 2.0);
    let px = col01.x * lx + col01.z * ly + trans.x;
    let py = col01.y * lx + col01.w * ly + trans.y;
    let ndcx =  px / screen.size.x * 2.0 - 1.0;
    let ndcy = -(py / screen.size.y * 2.0 - 1.0);

    var out: VertexOut;
    out.frag_coord     = vec4f(ndcx, ndcy, 0.0, 1.0);
    out.local_pos      = vec2f(lx - pw * 0.5, ly - ph * 0.5);
    out.half_size      = vec2f(pw * 0.5, ph * 0.5);
    out.radius         = radius;
    out.aa_width       = aa_width;
    out.fill_color     = fill_color;
    out.border_color   = border_color;
    out.clip           = clip;
    out.border_widths  = border_widths;
    out.grad_color0    = grad_color0;
    out.grad_color1    = grad_color1;
    out.grad_dir       = grad_params.xy;
    // Store the unit quad corner for gradient interpolation (0..1 range)
    out.grad_local     = q;
    return out;
}

fn sdf_rrect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
    let q = abs(p) - half_size + radius;
    return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn aa_coverage(d: f32) -> f32 { return clamp(0.5 - d, 0.0, 1.0); }

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    let cl = in.clip;
    if cl.x != 0.0 || cl.y != 0.0 || cl.z != 0.0 || cl.w != 0.0 {
        if in.frag_coord.x < cl.x || in.frag_coord.y < cl.y ||
           in.frag_coord.x > cl.z || in.frag_coord.y > cl.w { discard; }
    }

    let d     = sdf_rrect(in.local_pos, in.half_size, in.radius);
    let outer = aa_coverage(d);
    if outer <= 0.0 { discard; }

    // Resolve fill colour — solid or gradient
    var base_fill: vec4f;
    // grad_color0.a > 0 signals gradient is active (solid has a=0 sentinel)
    if in.grad_color0.a > 0.0 {
        // Project the fragment's local position onto the gradient direction.
        // grad_local is in [0,1] quad space; center it to [-0.5, 0.5].
        let centered = in.grad_local - vec2f(0.5);
        let t = clamp(dot(centered, in.grad_dir) + 0.5, 0.0, 1.0);
        base_fill = mix(in.grad_color0, in.grad_color1, t);
    } else {
        base_fill = in.fill_color;
    }

    let bw_top    = in.border_widths.x;
    let bw_right  = in.border_widths.y;
    let bw_bottom = in.border_widths.z;
    let bw_left   = in.border_widths.w;
    let has_border = (bw_top + bw_right + bw_bottom + bw_left) > 0.0
                  && in.border_color.a > 0.0;

    var color: vec4f;
    if has_border {
        let inner_half = vec2f(
            in.half_size.x - (bw_left + bw_right)  * 0.5,
            in.half_size.y - (bw_top  + bw_bottom) * 0.5,
        );
        let offset = vec2f((bw_left - bw_right) * 0.5, (bw_top - bw_bottom) * 0.5);
        let inner_radius = max(in.radius - max(max(bw_top, bw_bottom), max(bw_left, bw_right)), 0.0);
        let d_inner = sdf_rrect(in.local_pos - offset, inner_half, inner_radius);
        let inner   = aa_coverage(d_inner);
        color = mix(in.border_color, base_fill, inner);
    } else {
        color = base_fill;
    }

    return vec4f(color.rgb, color.a * outer);
}
