// text.wgsl
//
// Each GlyphInstance now carries a full 2x3 affine transform so glyphs can
// be rotated and scaled exactly like rects and images.
//
// Instance layout:
//   location 0: col01  — vec4(a, b, c, d)   2x2 rotation+scale (physical px)
//   location 1: trans  — vec4(tx, ty, gw, gh) translation + glyph physical size
//   location 2: uv     — vec2(u0, v0)
//   location 3: uv_sz  — vec2(uw, vh)
//   location 4: color  — vec4
//   location 5: clip   — vec4  (physical px, all-zero = no clip)
//   location 6: flags  — u32   (1 = color emoji)

struct GlyphInstance {
    @location(0) col01: vec4<f32>,   // a, b, c, d
    @location(1) trans: vec4<f32>,   // tx, ty, gw, gh
    @location(2) uv:    vec2<f32>,
    @location(3) uv_sz: vec2<f32>,
    @location(4) color: vec4<f32>,
    @location(5) clip:  vec4<f32>,
    @location(6) flags: u32,
}

struct VertexOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) tex_uv:   vec2<f32>,
    @location(1) color:    vec4<f32>,
    @location(2) frag_pos: vec2<f32>,
    @location(3) clip:     vec4<f32>,
    @location(4) flags:    u32,
}

@group(0) @binding(0) var<uniform> screen:       vec2<f32>;
@group(0) @binding(1) var atlas_tex:    texture_2d<f32>;
@group(0) @binding(2) var atlas_sampler: sampler;

@vertex
fn vs_main(inst: GlyphInstance, @builtin(vertex_index) vi: u32) -> VertexOut {
    // Unit quad corners in [0,1]
    let cx = array<f32, 6>(0.0, 1.0, 0.0, 0.0, 1.0, 1.0);
    let cy = array<f32, 6>(0.0, 0.0, 1.0, 1.0, 0.0, 1.0);
    let fx = cx[vi];
    let fy = cy[vi];

    let gw = inst.trans.z;
    let gh = inst.trans.w;

    // Local corner in glyph space (physical pixels, origin = glyph top-left)
    let lx = gw * fx;
    let ly = gh * fy;

    // Apply 2x3 affine transform:  screen = M * local + translation
    let px = inst.col01.x * lx + inst.col01.z * ly + inst.trans.x;
    let py = inst.col01.y * lx + inst.col01.w * ly + inst.trans.y;

    let ndcx =  px / screen.x * 2.0 - 1.0;
    let ndcy = -py / screen.y * 2.0 + 1.0;

    var out: VertexOut;
    out.clip_pos = vec4<f32>(ndcx, ndcy, 0.0, 1.0);
    out.tex_uv   = inst.uv + inst.uv_sz * vec2<f32>(fx, fy);
    out.color    = inst.color;
    out.frag_pos = vec2<f32>(px, py);
    out.clip     = inst.clip;
    out.flags    = inst.flags;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let has_clip = any(in.clip != vec4<f32>(0.0, 0.0, 0.0, 0.0));
    if has_clip {
        if in.frag_pos.x < in.clip.x || in.frag_pos.y < in.clip.y ||
           in.frag_pos.x > in.clip.z || in.frag_pos.y > in.clip.w {
            discard;
        }
    }

    let sample = textureSample(atlas_tex, atlas_sampler, in.tex_uv);

    if in.flags == 1u {
        // colour emoji — premultiply
        return vec4<f32>(sample.rgb * sample.a, sample.a);
    } else {
        // mask glyph — tint with per-instance colour
        let alpha = sample.r * in.color.a;
        return vec4<f32>(in.color.rgb * alpha, alpha);
    }
}
