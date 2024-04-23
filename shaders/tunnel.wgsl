const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0) var<uniform> u_time: TimeUniform;
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
fn osc(mV: f32, MV: f32, i: f32, t: f32) -> f32 {
    return mV + (MV - mV) * 0.5 * (sin(2.0 * PI * t / i) + 1.0);
}

fn r2d(p: vec2<f32>, a: f32) -> vec2<f32> {
    let s: f32 = sin(a);
    let c: f32 = cos(a);
    let rm: mat2x2<f32> = mat2x2<f32>(c, -s, s, c);
    return rm * p;
}

fn cC(uv: vec2<f32>, time: f32, resolution: vec2<f32>) -> vec4<f32> {
    var p: vec3<f32> = vec3<f32>(uv, 1.0);
    let a: f32 = 0.15 * PI * time; 
    let rp: vec2<f32> = r2d(p.xy, a);
    let mX: f32 = (sin(u_time.time * 0.5) + 1.0) * params.theta;
    let mY: f32 = (cos(u_time.time * 0.5) + 1.0) * params.theta;
    var hc: vec3<f32> = vec3<f32>((rp.xy * 1.5 + 0.5) * resolution.xy / resolution.y, mX);
    for (var i: i32 = 0; i < 45; i++) {
        let xV: f32 = osc(1.2, 1.3, 10.0, time);
        let yV: f32 = osc(params.alpha, params.sigma, 8.0, u_time.time);
        let zV: f32 = osc(params.gamma, params.blue, 8.0, u_time.time);
        let temp: vec3<f32> = vec3<f32>(xV, yV, zV) * (abs((abs(hc) / abs(dot(hc, hc))) - vec3<f32>(1.0, 1.0, mY)));
        hc.x = temp.x;
        hc.z = temp.y;
        hc.y = temp.z;
    }
    return vec4<f32>(hc, 1.0);
}

fn bl(uv: vec2<f32>, time: f32, resolution: vec2<f32>) -> vec4<f32> {
    let bs: f32 = 0.1 / 255.0; 
    var col: vec4<f32> = vec4<f32>(0.0);
    for (var x: f32 = -1.1; x <= 0.5; x += 1.0) {
        for (var y: f32 = -1.1; y <= 2.5; y += 1.0) {
            col += cC(uv + vec2<f32>(x, y) * bs, time, resolution);
        }
    }
    return col / 9.0; 
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = params.lambda * (1.5 * FragCoord.xy - resolution.xy) / resolution.y * 1.1;
    let fc: vec4<f32> = bl(uv, u_time.time, resolution);
    return fc;
}
