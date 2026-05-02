// image.wgsl — instanced image with affine transform, rounded corners, and border
//
// Instance layout:
//   location 0: col01          vec4(a, b, c, d)
//   location 1: trans          vec4(tx, ty, pw, ph)
//   location 2: uv             vec4(u0, v0, u1, v1)
//   location 3: tint           vec4
//   location 4: clip           vec4
//   location 5: params         vec4(radius, 0, 0, 0)
//   location 6: border_color   vec4
//   location 7: border_widths  vec4(top, right, bottom, left)

struct ScreenUniform { size: vec2<f32>, _pad: vec2<f32> }
@group(0) @binding(0) var<uniform> screen: ScreenUniform;
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var smp: sampler;

struct VertexInput {
    @location(0) col01:         vec4<f32>,
    @location(1) trans:         vec4<f32>,
    @location(2) uv:            vec4<f32>,
    @location(3) tint:          vec4<f32>,
    @location(4) clip:          vec4<f32>,
    @location(5) params:        vec4<f32>,
    @location(6) border_color:  vec4<f32>,
    @location(7) border_widths: vec4<f32>,
    @builtin(vertex_index)   vi: u32,
    @builtin(instance_index) ii: u32,
}

struct VertexOutput {
    @builtin(position) pos:          vec4<f32>,
    @location(0)       tex_uv:       vec2<f32>,
    @location(1)       tint:         vec4<f32>,
    @location(2)       clip:         vec4<f32>,
    @location(3)       local_pos:    vec2<f32>,
    @location(4)       half_size:    vec2<f32>,
    @location(5)       r:            f32,
    @location(6)       border_color: vec4<f32>,
    @location(7)       border_widths:vec4<f32>,
}

var<private> QUAD_UV: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0), vec2<f32>(1.0, 0.0), vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 0.0), vec2<f32>(1.0, 1.0), vec2<f32>(0.0, 1.0),
);

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let q  = QUAD_UV[in.vi];
    let pw = in.trans.z; let ph = in.trans.w;
    let lx = q.x * pw; let ly = q.y * ph;
    let px = in.col01.x * lx + in.col01.z * ly + in.trans.x;
    let py = in.col01.y * lx + in.col01.w * ly + in.trans.y;
    let ndc_x =  px / screen.size.x * 2.0 - 1.0;
    let ndc_y = 1.0 - py / screen.size.y * 2.0;

    var out: VertexOutput;
    out.pos           = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.tex_uv        = vec2<f32>(mix(in.uv.x, in.uv.z, q.x), mix(in.uv.y, in.uv.w, q.y));
    out.tint          = in.tint;
    out.clip          = in.clip;
    out.local_pos     = vec2<f32>(lx - pw * 0.5, ly - ph * 0.5);
    out.half_size     = vec2<f32>(pw * 0.5, ph * 0.5);
    out.r             = in.params.x;
    out.border_color  = in.border_color;
    out.border_widths = in.border_widths;
    return out;
}

fn sdf_rrect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2(r);
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn aa_coverage(d: f32) -> f32 { return clamp(0.5 - d, 0.0, 1.0); }

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let has_clip = in.clip.z > 0.0 || in.clip.w > 0.0;
    if has_clip && (in.pos.x < in.clip.x || in.pos.y < in.clip.y ||
                    in.pos.x > in.clip.z || in.pos.y > in.clip.w) { discard; }

    let outer = aa_coverage(sdf_rrect(in.local_pos, in.half_size, in.r));
    if outer <= 0.0 { discard; }

    var color = textureSample(tex, smp, in.tex_uv) * in.tint;

    // Border overlay — same logic as rect shader
    let bw_top    = in.border_widths.x;
    let bw_right  = in.border_widths.y;
    let bw_bottom = in.border_widths.z;
    let bw_left   = in.border_widths.w;
    let has_border = (bw_top + bw_right + bw_bottom + bw_left) > 0.0
                  && in.border_color.a > 0.0;
    if has_border {
        let inner_half = vec2<f32>(
            in.half_size.x - (bw_left + bw_right)  * 0.5,
            in.half_size.y - (bw_top  + bw_bottom) * 0.5,
        );
        let offset = vec2<f32>((bw_left - bw_right) * 0.5, (bw_top - bw_bottom) * 0.5);
        let inner_r = max(in.r - max(max(bw_top, bw_bottom), max(bw_left, bw_right)), 0.0);
        let inner = aa_coverage(sdf_rrect(in.local_pos - offset, inner_half, inner_r));
        // outside inner = border region; blend border_color over image
        color = mix(in.border_color, color, inner);
    }

    return vec4<f32>(color.rgb, color.a * outer);
}
