//this is very expensive mandelbrot careful if you have low gpu, go line 61 and reduce the AA (or iter)
const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
    aa:f32,
    iter:f32,
    bound:f32,
    tt:f32,
    a:f32,
    b:f32,
    c:f32,
    d:f32,
    e:f32,
    f:f32,
    g:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
fn implicit(c: vec2<f32>, time: f32) -> vec2<f32> {
    var MAX_ITER: i32 = i32(params.iter);
    var BOUND: f32 = params.bound;
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var i: i32 = 0;
    loop {
        if (i >= MAX_ITER) { break; }
        let xnew: f32 = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = xnew;

        let dampenedTime: f32 = time / params.tt; 
        z += 0.1 * vec2<f32>(sin(0.001 * dampenedTime), cos(0.001 * dampenedTime));

        if (dot(z, z) > BOUND / 1.2) {
            break;
        }
        i = i + 1;
    }
    return vec2<f32>(f32(i), dot(z, z));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var MAX_ITER: i32 = i32(params.iter);
    var BOUND: f32 = params.bound;
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let pan: vec2<f32> = vec2<f32>(params.theta, params.alpha);
    let zoomLevel: f32 = osc(params.lambda, params.lambda, 20.0, u_time.time / params.tt);
    let AA: i32 = i32(params.aa);
    let camSpeed: vec2<f32> = vec2<f32>(0.0002, 0.0002);
    let camPath: vec2<f32> = vec2<f32>(sin(camSpeed.x * u_time.time / params.tt), cos(camSpeed.y * u_time.time / params.tt));
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    for (var m: i32 = 0; m < AA; m = m + 1) {
        for (var n: i32 = 0; n < AA; n = n + 1) {
            let uv: vec2<f32> = ((FragCoord.xy + vec2<f32>(f32(m), f32(n)) / f32(AA) - 0.5 * resolution) / min(resolution.y, resolution.x) * zoomLevel + pan + camPath) * 2.033 - vec2<f32>(2.14278);
            let z_and_i: vec2<f32> = implicit(uv, u_time.time);
            let iter_ratio: f32 = z_and_i.x / f32(MAX_ITER);
            let lenSq: f32 = z_and_i.y;
            let exteriorColor: vec3<f32> = params.a + params.b * sin(params.c + vec3<f32>(params.sigma, params.gamma, params.blue) + params.g*PI * vec3<f32>(params.d* iter_ratio) + u_time.time / params.e);
            if (iter_ratio >= 1.0) {
                let c1: f32 = pow(clamp(2.00 * sqrt(lenSq), 0.0, 1.0), 0.5);
                let col1: vec3<f32> = 0.5 + 0.5 * sin(1.0 + vec3<f32>(params.sigma, params.gamma, params.blue) + PI * vec3<f32>(2.0 * lenSq) + u_time.time / params.f);
                let col2: vec3<f32> = 0.5 + 0.5 * sin(2.1 + PI * vec3<f32>(lenSq) + u_time.time / 2.0);
                col += 1.5 * sqrt(c1 * col1 * col2);
            } else {
                col += exteriorColor;
            }
        }
    }

    col /= f32(AA * AA);
    return vec4<f32>(col, 1.0);
}
