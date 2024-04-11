//my first attempt for image manipulation in wgsl (based on my glsl code: https://www.shadertoy.com/view/cdBcW3)
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
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); // Example fixed resolution
    let imgColor: vec4<f32> = textureSample(tex, tex_sampler, tex_coords);
    let luminance: f32 = dot(imgColor.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let centeredUv: vec2<f32> = (tex_coords * resolution - 0.5 * resolution) / min(resolution.y, resolution.x);
    let r: f32 = length(centeredUv);
    let theta: f32 = atan2(centeredUv.y, centeredUv.x);
    let spiral: f32 = params.lambda * cos(params.blue * (theta + params.theta * r - params.alpha * u_time.time)) + luminance;
    var col: vec3<f32>;
    //you can adjut colors on here (0.0 to 0.1; for each channel)
    col.r = smoothstep(0.0, params.sigma, spiral);
    col.g = smoothstep(0.0, params.sigma, spiral);
    col.b = smoothstep(0.0, params.gamma, spiral);
    col *= 2.0 + 1.0 * cos(u_time.time + tex_coords.xyx * 3.14);
    return vec4<f32>(col, 1.0);
}