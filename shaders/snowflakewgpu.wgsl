//my GLSL version: https://www.shadertoy.com/view/dsGfDG
// Global constants
const PI: f32 = 3.14159265;
const MAX_ITER: i32 = 125;
const BOUND: f32 = 25.0;
const AA: i32 = 3;
// TimeUniform definition based on my wgpu rust code
struct TimeUniform {
    time: f32,
};
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn implicit(c: vec2<f32>, time: f32) -> vec2<f32> {
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var i: i32 = 0;
    loop {
        if (i >= MAX_ITER) { break; }
        let sin_z: vec2<f32> = vec2<f32>(sin(z.x) * cosh(z.y), cos(z.x) * sinh(z.y));
        z = vec2<f32>(c.x * sin_z.x - c.y * sin_z.y, c.x * sin_z.y + c.y * sin_z.x);
        z += 0.2 * vec2<f32>(sin(params.blue * time), cos(params.blue * time));
        if (dot(z, z) > BOUND * BOUND) { break; }
        i = i + 1;
        continue;
    }
    return vec2<f32>(f32(i), dot(z, z));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 600.0); //adjust as needed
    var col: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    let pan: vec2<f32> = vec2<f32>(0.878729, 1.504069);
    let zl: f32 = 0.005;
    for (var m: i32 = 0; m < AA; m++) {
        for (var n: i32 = 0; n < AA; n++) {
            let uv: vec2<f32> = ((FragCoord.xy + vec2<f32>(f32(m), f32(n)) / f32(AA) - 0.5 * resolution) / min(resolution.y, resolution.x) * zl + pan) * 2.033 - vec2<f32>(2.04278);
            let z_and_i: vec2<f32> = implicit(uv, u_time.time);
            let iter_ratio: f32 = z_and_i.x / f32(MAX_ITER);
            let col1: vec3<f32> = 0.5 + 0.5 * cos(u_time.time + vec3<f32>(params.theta, params.alpha, params.sigma) + params.lambda * PI * vec3<f32>(iter_ratio));
            col = col * 0.9; 
            col += col1;
        }
    }
    col /= f32(AA * AA);
    col = applyGamma(col,params.gamma);
    return vec4<f32>(col, 1.0);
}