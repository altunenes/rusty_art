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
const numSamples: i32 = 10;
const numSquares: i32 = 15;

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = FragCoord.xy / resolution;
    let squareWidth: f32 = 1.0 / f32(numSquares);
    var colorSums: array<vec3<f32>, numSquares>;
    for (var k: i32 = 0; k < numSquares; k++) {
        colorSums[k] = vec3<f32>(0.0, 0.0, 0.0);
    }
    for (var j: i32 = 0; j < numSamples; j++) {
        let sampleY: f32 = (f32(j) +0.5) / f32(numSamples);
        for (var i: i32 = 0; i < numSquares; i++) {
            let sampleX: f32 = (f32(i) + params.alpha) / f32(numSquares);
            let sampleUV: vec2<f32> = vec2<f32>(sampleX, sampleY);
            colorSums[i] += textureSample(tex, tex_sampler, sampleUV).rgb;
        }
    }
    for (var l: i32 = 0; l < numSquares; l++) {
        colorSums[l] /= f32(numSamples);
    }
    let squareIndex: i32 = i32(uv.x * f32(numSquares));
    let finalColor: vec4<f32> = mix(vec4<f32>(textureSample(tex, tex_sampler, tex_coords).rgb, 1.0), vec4<f32>(colorSums[squareIndex], 1.0), step(params.sigma, uv.y));
    return finalColor;
}