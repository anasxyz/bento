struct ScreenUniform {
    size: vec2<f32>,
    _pad: vec2<f32>,
}

@group(0) @binding(0) var<uniform> screen: ScreenUniform;
@group(1) @binding(0) var tex:     texture_2d<f32>;
@group(1) @binding(1) var smp:     sampler;

struct VertexInput {
    @location(0) pos_size: vec4<f32>,
    @location(1) uv:       vec4<f32>,
    @location(2) tint:     vec4<f32>,
    @location(3) clip:     vec4<f32>,
    @location(4) params:   vec4<f32>,
    @builtin(vertex_index)   vi: u32,
    @builtin(instance_index) ii: u32,
}

struct VertexOutput {
    @builtin(position) pos:  vec4<f32>,
    @location(0)       uv:   vec2<f32>,
    @location(1)       tint: vec4<f32>,
    @location(2)       clip: vec4<f32>,
    @location(3)       rect: vec4<f32>,
    @location(4)       r:    f32,
}

fn quad_uv(vi: u32) -> vec2<f32> {
    switch vi {
        case 0u: { return vec2(0.0, 0.0); }
        case 1u: { return vec2(1.0, 0.0); }
        case 2u: { return vec2(0.0, 1.0); }
        case 3u: { return vec2(1.0, 0.0); }
        case 4u: { return vec2(1.0, 1.0); }
        default: { return vec2(0.0, 1.0); }
    }
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    let q     = quad_uv(in.vi);
    let px    = in.pos_size.x + q.x * in.pos_size.z;
    let py    = in.pos_size.y + q.y * in.pos_size.w;
    let ndc_x = (px / screen.size.x) * 2.0 - 1.0;
    let ndc_y = 1.0 - (py / screen.size.y) * 2.0;

    var out: VertexOutput;
    out.pos  = vec4(ndc_x, ndc_y, 0.0, 1.0);
    out.uv   = vec2(mix(in.uv.x, in.uv.z, q.x), mix(in.uv.y, in.uv.w, q.y));
    out.tint = in.tint;
    out.clip = in.clip;
    out.rect = vec4(
        in.pos_size.x,
        in.pos_size.y,
        in.pos_size.x + in.pos_size.z,
        in.pos_size.y + in.pos_size.w,
    );
    out.r = in.params.x;
    return out;
}

fn sdf_rrect(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + vec2(r);
    return length(max(q, vec2(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let frag = in.pos.xy;

    let has_clip = in.clip.z > 0.0 || in.clip.w > 0.0;
    if has_clip && (frag.x < in.clip.x || frag.y < in.clip.y ||
                    frag.x > in.clip.z || frag.y > in.clip.w) {
        discard;
    }

    var color = textureSample(tex, smp, in.uv) * in.tint;

    if in.r > 0.5 {
        let centre = (in.rect.xy + in.rect.zw) * 0.5;
        let half   = (in.rect.zw - in.rect.xy) * 0.5;
        let dist   = sdf_rrect(frag - centre, half, in.r);
        color.a   *= clamp(-dist, 0.0, 1.0);
    }

    return color;
}
