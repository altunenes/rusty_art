//live shadertoy version: https://www.shadertoy.com/view/dsyczD
const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
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
const iter: i32 = 123;
const zz: f32 = 77.0;
const h: f32 = 0.1;


fn complex_power(z: vec2<f32>, x: f32) -> vec2<f32> {
    let r: f32 = length(z);
    let theta: f32 = atan2(z.y, z.x);
    let r_pow: f32 = pow(r, x);
    let x_theta: f32 = x * theta;
    return vec2<f32>(r_pow * cos(x_theta), r_pow * sin(x_theta));
}

fn f(z: vec2<f32>) -> vec2<f32> {
    return complex_power(z, 1.5) - vec2<f32>(0.2, 0.0);
}

fn implicit(z_input: vec2<f32>) -> vec2<f32> {
    var z: vec2<f32> = z_input;
    var dz: vec2<f32> = vec2<f32>(h, 0.0);
    var i: i32 = 0;
    loop {
        if (i >= iter) { break; }
        dz = 1.5 * pow(length(z), 0.5) * dz;
        z = f(z);
        if (dot(z, z) > zz) {
            break;
        }
        i = i + 1;
    }
    return vec2<f32>(f32(i), dot(z, z) / dot(dz, dz));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let pan: vec2<f32> = vec2<f32>(params.lambda,params.blue);
    let zoom: f32 = 0.24;
    let AA: i32 = 1;
    let resolution: vec2<f32> = vec2<f32>(800.0, 600.0);
    let time: f32 = u_time.time;
    for (var m: i32 = 0; m < AA; m = m + 1) {
        for (var n: i32 = 0; n < AA; n = n + 1) {
            let uv: vec2<f32> = ((vec2<f32>(FragCoord.x, resolution.y - FragCoord.y) - 0.5 * resolution) / min(resolution.y, resolution.x) + pan) * zoom;
            let z_and_i: vec2<f32> = implicit(uv);
            let iter_ratio: f32 = z_and_i.x / f32(iter);
            let sharpness: f32 = z_and_i.y;
            let col1: vec3<f32> = 0.5 + 0.5 * cos(1.0 + time + vec3<f32>(params.theta, params.alpha, params.sigma) + PI * vec3<f32>(params.gamma * sharpness));
            let col2: vec3<f32> = 0.5 + 0.5 * cos(4.1 + time + PI * vec3<f32>(sharpness));
            col = col + mix(col1, col2, iter_ratio);
        }
    }
    col = sqrt(col / f32(AA * AA));
    col = applyGamma(col,0.5);
    return vec4<f32>(col, 1.0);
}