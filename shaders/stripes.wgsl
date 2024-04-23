@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
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
const PI: f32 = 3.1415926535897932384626433832795;
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let p: vec2<f32> = tex_coords+params.alpha;
    let sampledColor: vec4<f32> = textureSample(tex, tex_sampler, p);
    let l: f32 = fract((params.lambda * params.theta * (p.x + p.y) +0.1 * (params.blue*u_time.time)* sampledColor.g) * params.gamma);
    let color: vec3<f32> = vec3<f32>(smoothstep(0.1, params.sigma, abs(l - 0.5)));
    return vec4<f32>(color, 1.0);
}
