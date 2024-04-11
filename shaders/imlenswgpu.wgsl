@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};
const PI: f32 = 3.14159;

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
fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let scramb: f32 = osc(params.lambda, params.theta, params.alpha, u_time.time); 
    let scramb2: f32 = osc(params.sigma, params.gamma, params.alpha, u_time.time); 
    let effectRadius: f32 = params.blue;
    let effectAngle: f32 = scramb2 * PI;
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let center: vec2<f32> = vec2<f32>(0.5, 0.5) + vec2<f32>(cos(u_time.time), sin(u_time.time)) * scramb;
    var uv: vec2<f32> = (FragCoord.xy / resolution) - center;
    let len: f32 = length(uv * vec2<f32>(resolution.x / resolution.y, 1.0));
    let angle: f32 = atan2(uv.y, uv.x) + effectAngle * smoothstep(effectRadius, 0.0, len);
    let radius: f32 = length(uv);
    let modifiedUV: vec2<f32> = vec2<f32>(radius * cos(angle), radius * sin(angle)) + center;
    let fragColor: vec4<f32> = textureSample(tex, tex_sampler, modifiedUV);
    return vec4<f32>(fragColor.rgb, 1.0); 
}