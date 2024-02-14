@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};
const PI: f32 = 3.14159;
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
const AVERAGE_COLOR: vec4<f32> = vec4<f32>(0.5, 0.5, 0.5, 1.0);
const mtx: mat2x2<f32> = mat2x2<f32>(1.1, 0.6, -0.6, 0.8);
fn noise(x: vec2<f32>) -> f32 {
    var p: vec2<f32> = floor(x);
    var f: vec2<f32> = fract(x);
    f = f * f * (7.0 - 6.0 * f);
    let a: f32 = textureSampleLevel(tex, tex_sampler, (p + vec2<f32>(0.5, 0.5)) / 256.0, 0.0).x;
    let b: f32 = textureSampleLevel(tex, tex_sampler, (p + vec2<f32>(1.5, 0.5)) / 256.0, 0.0).x;
    let c: f32 = textureSampleLevel(tex, tex_sampler, (p + vec2<f32>(0.5, 1.5)) / 256.0, 0.0).x;
    let d: f32 = textureSampleLevel(tex, tex_sampler, (p + vec2<f32>(1.5, 1.5)) / 256.0, 0.0).x;
    return mix(mix(a, b, f.x), mix(c, d, f.x), f.y);
}
fn fbm(p_initial: vec2<f32>) -> f32 {
    var p: vec2<f32> = p_initial; 
    var f: f32 = 0.0;
    f += 2.500000 * noise(p); p = mtx * p * 6.07;
    f += 1.250000 * noise(p); p = mtx * p * 6.03;
    f += 0.625000 * noise(p); p = mtx * p * 6.01;
    f += 0.312500 * noise(p); p = mtx * p * 6.04;
    f += 0.156250 * noise(p); p = mtx * p * 6.01;
    f += 0.078125 * noise(p);
    return f / 0.96875;
}
struct PatternResult {
    q: vec2<f32>,
    r: vec2<f32>,
    g: vec2<f32>,
    value: f32,
};
fn pattern(p: vec2<f32>, t: f32) -> PatternResult {
    let q: vec2<f32> = vec2<f32>(fbm(p), fbm(p + vec2<f32>(3.0, 1.3)));
    let r: vec2<f32> = vec2<f32>(fbm(p + q + vec2<f32>(t, 0.0) + vec2<f32>(1.7, 9.2)), fbm(p + q + vec2<f32>(t, 0.0) + vec2<f32>(1.3, 1.8)));
    let g: vec2<f32> = vec2<f32>(fbm(p + 2.0 * r + vec2<f32>(t * 20.0, 0.0) + vec2<f32>(2.0, 6.0)), fbm(p + 2.0 * r + vec2<f32>(t, 0.0) + vec2<f32>(5.0, 3.0)));
    let value: f32 = fbm(p + 1.5 * g + vec2<f32>(-t * 7.0, 0.0));
    return PatternResult(q, r, g, value);
}
fn getColorFromImage(pos: vec2<f32>) -> vec4<f32> {
    let wrappedPos: vec2<f32> = fract(pos);
    let color: vec4<f32> = textureSample(tex, tex_sampler, wrappedPos);
    let edgeFade: f32 = smoothstep(1.0, 0.01, min(min(wrappedPos.x, wrappedPos.y), min(1.0 - wrappedPos.x, 1.0 - wrappedPos.y)));
    return color * edgeFade;
}
fn logistic(x: f32) -> f32 {
    return 1.0 / (1.0 + exp(-x));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
     let resolution: vec2<f32> = vec2<f32>(800.0, 600.0);
    var uv: vec2<f32> = FragCoord.xy / resolution;
    let patternResult: PatternResult = pattern(FragCoord.xy * vec2<f32>(0.004), u_time.time * 0.007);
    var fluidVelocity: vec2<f32> = vec2<f32>(patternResult.value, patternResult.value);
    fluidVelocity *= logistic(1.0 * u_time.time - 5.0);
    var imagePos: vec2<f32> = uv + fluidVelocity * 6.8;
    let col: vec4<f32> = getColorFromImage(imagePos);
    let col_vel: vec4<f32> = vec4<f32>(fluidVelocity, 0.4, 0.1) * 0.1 + 0.1;
    let mean_color: vec4<f32> = mix(col, AVERAGE_COLOR, 0.01);
    let finalColor: vec4<f32> = mix(mean_color, col_vel, 0.3);
    return finalColor;
}