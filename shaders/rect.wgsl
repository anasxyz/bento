var<private> QUAD: array<vec2f, 6> = array<vec2f, 6>(
    vec2f(0.0, 0.0),
    vec2f(1.0, 0.0),
    vec2f(0.0, 1.0),
    vec2f(1.0, 0.0),
    vec2f(1.0, 1.0),
    vec2f(0.0, 1.0),
);

struct VertexOut {
    @builtin(position) frag_coord  : vec4f,
    @location(0) local_pos         : vec2f,
    @location(1) half_size         : vec2f,
    @location(2) radius            : f32,
    @location(3) aa_width          : f32,
    @location(4) fill_color        : vec4f,
    @location(5) border_color      : vec4f,
    @location(6) clip              : vec4f,
    @location(7) border_widths     : vec4f, // top, right, bottom, left
}

@vertex
fn vs_main(
    @builtin(vertex_index) vi : u32,
    @location(0) pos_size     : vec4f,
    @location(1) params       : vec4f,  // [radius, aa_width, 0, 0]
    @location(2) fill_color   : vec4f,
    @location(3) border_color : vec4f,
    @location(4) clip         : vec4f,
    @location(5) screen_size  : vec4f,
    @location(6) border_widths: vec4f,  // top, right, bottom, left
) -> VertexOut {
    let x  = pos_size.x;  let y  = pos_size.y;
    let w  = pos_size.z;  let h  = pos_size.w;
    let sw = screen_size.x;  let sh = screen_size.y;
    let radius   = params.x;
    let aa_width = params.y;
    let b  = aa_width;
    let qx = x - b;  let qy = y - b;
    let qw = w + b * 2.0;  let qh = h + b * 2.0;
    let c  = QUAD[vi];
    let px = qx + c.x * qw;
    let py = qy + c.y * qh;
    let ndcx =  px / sw * 2.0 - 1.0;
    let ndcy = -(py / sh * 2.0 - 1.0);
    let cx = x + w * 0.5;
    let cy = y + h * 0.5;
    var out: VertexOut;
    out.frag_coord    = vec4f(ndcx, ndcy, 0.0, 1.0);
    out.local_pos     = vec2f(px - cx, py - cy);
    out.half_size     = vec2f(w * 0.5, h * 0.5);
    out.radius        = radius;
    out.aa_width      = aa_width;
    out.fill_color    = fill_color;
    out.border_color  = border_color;
    out.clip          = clip;
    out.border_widths = border_widths;
    return out;
}

fn sdf_rrect(p: vec2f, half_size: vec2f, radius: f32) -> f32 {
    let q = abs(p) - half_size + radius;
    return length(max(q, vec2f(0.0))) + min(max(q.x, q.y), 0.0) - radius;
}

fn aa_coverage(d: f32) -> f32 {
    let fw = fwidth(d);
    return clamp(0.5 - d / max(fw, 0.0001), 0.0, 1.0);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4f {
    // clip
    let cl = in.clip;
    if cl.x != 0.0 || cl.y != 0.0 || cl.z != 0.0 || cl.w != 0.0 {
        if in.frag_coord.x < cl.x || in.frag_coord.y < cl.y ||
           in.frag_coord.x > cl.z || in.frag_coord.y > cl.w {
            discard;
        }
    }

    let d = sdf_rrect(in.local_pos, in.half_size, in.radius);
    let outer = aa_coverage(d);
    if outer <= 0.0 { discard; }

    let bw_top    = in.border_widths.x;
    let bw_right  = in.border_widths.y;
    let bw_bottom = in.border_widths.z;
    let bw_left   = in.border_widths.w;

    let has_border = (bw_top + bw_right + bw_bottom + bw_left) > 0.0
                   && in.border_color.a > 0.0;

    var color: vec4f;
    if has_border {
        // shrink the inner rect by the border widths per side
        let inner_half = vec2f(
            in.half_size.x - (bw_left + bw_right) * 0.5,
            in.half_size.y - (bw_top  + bw_bottom) * 0.5
        );
        // offset center due to asymmetric borders
        let offset = vec2f(
            (bw_left - bw_right) * 0.5,
            (bw_top  - bw_bottom) * 0.5
        );
        let inner_radius = max(in.radius - max(max(bw_top, bw_bottom), max(bw_left, bw_right)), 0.0);
        let d_inner = sdf_rrect(in.local_pos - offset, inner_half, inner_radius);
        let inner = aa_coverage(d_inner);
        color = mix(in.border_color, in.fill_color, inner);
    } else {
        color = in.fill_color;
    }

    return vec4f(color.rgb, color.a * outer);
}
