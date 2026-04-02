struct GlyphInstance {
    @location(0) pos:   vec2<f32>,
    @location(1) size:  vec2<f32>,
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

@group(0) @binding(0) var<uniform> screen: vec2<f32>;
@group(0) @binding(1) var atlas_tex: texture_2d<f32>;
@group(0) @binding(2) var atlas_sampler: sampler;

@vertex
fn vs_main(inst: GlyphInstance, @builtin(vertex_index) vi: u32) -> VertexOut {
    let cx = array<f32, 6>(0.0, 1.0, 0.0, 0.0, 1.0, 1.0);
    let cy = array<f32, 6>(0.0, 0.0, 1.0, 1.0, 0.0, 1.0);
    let fx = cx[vi];
    let fy = cy[vi];
    let px = inst.pos.x + inst.size.x * fx;
    let py = inst.pos.y + inst.size.y * fy;
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
        return sample;
    } else {
        let alpha = sample.r * in.color.a;
        return vec4<f32>(in.color.rgb * alpha, alpha);
    }
}
