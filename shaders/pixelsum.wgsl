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
    alpha: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};

@group(2) @binding(2)
var<uniform> params: Params;

const PI: f32 = 3.14;

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    let uv: vec2<f32> = (FragCoord.xy * 3.0 - resolution) / min(resolution.x, resolution.y);

    let avgColor: vec3<f32> = textureSample(tex, tex_sampler, uv).rgb;

    let numSquares: f32 = 15.0;
    let squareWidth: f32 = resolution.x / numSquares;
    let normalizedSquareIndex: f32 = floor(FragCoord.x / squareWidth) / numSquares;

    let sampleUV: vec2<f32> = vec2<f32>(normalizedSquareIndex, 0.5);
    let squareColor: vec3<f32> = textureSample(tex, tex_sampler, sampleUV).rgb;

    let finalColor: vec4<f32> = mix(vec4<f32>(avgColor, 1.0), vec4<f32>(squareColor, 1.0), step(0.95, uv.y));
    return finalColor;
}
