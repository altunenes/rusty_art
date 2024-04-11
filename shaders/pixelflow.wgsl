struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

@group(0) @binding(0)
var tex: texture_2d<f32>;

@group(0) @binding(1)
var tex_sampler: sampler;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(2) @binding(2)
var<uniform> params: Params;

fn hash2(p: vec2<f32>) -> vec2<f32> {
    let transformed_p = vec2<f32>(dot(p, vec2<f32>(1.4, 748.6)), dot(p, vec2<f32>(1.3, 659.3))); 
    return fract(sin(transformed_p) * 3.5453);
}

fn noise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(mix(dot(hash2(i + vec2<f32>(0.0, 0.0)), f - vec2<f32>(0.0, 0.0)),
                   dot(hash2(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0)), u.x),
               mix(dot(hash2(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0)),
                   dot(hash2(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn fbm(p_initial: vec2<f32>) -> f32 {
    var value: f32 = 0.0;
    var amplitude: f32 = params.gamma;
    var p: vec2<f32> = p_initial;
    for (var i: i32 = 0; i < 5; i = i + 1) {
        value += amplitude * noise(p); 
        p *= params.blue;
        amplitude *= 0.3;
    }
    return value;
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 600.0); 
    var uv: vec2<f32> = (FragCoord.xy * 3.0 - resolution) / min(resolution.x, resolution.y);
    let t: f32 = u_time.time * params.theta;
    let noiseScale: f32 = params.lambda;
    var noiseVec: vec2<f32> = vec2<f32>(0.0, 0.0);
    noiseVec.x = fbm(uv + vec2<f32>(t, params.alpha)) * noiseScale;
    noiseVec.y = fbm(uv + vec2<f32>(params.sigma, t)) * noiseScale;

    let imagePos: vec2<f32> = fract(uv + noiseVec); 
    let col: vec4<f32> = textureSample(tex, tex_sampler, imagePos);

    return col; 
}