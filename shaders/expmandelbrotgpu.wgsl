//this is very expensive mandelbrot careful if you have low gpu, go line 61 and reduce the AA (or iter)
const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;


const MAX_ITER: i32 = 855;
const BOUND: f32 = 3.5;
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
};
@group(0) @binding(1)
var<uniform> params: Params;
fn remapTime(currentTime: f32, startInterval: f32, endInterval: f32, newDuration: f32) -> f32 {
    if currentTime < startInterval {
        return currentTime;
    } else if currentTime >= startInterval && currentTime <= endInterval {
        let normalizedTime: f32 = (currentTime - startInterval) / (endInterval - startInterval);
        return startInterval + normalizedTime * newDuration;
    } else {
        return currentTime + newDuration - (endInterval - startInterval);
    }
}

fn implicit(c: vec2<f32>, time: f32) -> vec2<f32> {
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var i: i32 = 0;
    loop {
        if (i >= MAX_ITER) { break; }
        let xnew: f32 = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = xnew;

        let dampenedTime: f32 = time / 18.001; 
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
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let pan: vec2<f32> = vec2<f32>(params.theta, params.alpha);
    let zoomLevel: f32 = osc(params.lambda, params.lambda, 20.0, u_time.time / 18.0);
    let AA: i32 = 4;

    let camSpeed: vec2<f32> = vec2<f32>(0.0002, 0.0002);
    let camPath: vec2<f32> = vec2<f32>(sin(camSpeed.x * u_time.time / 18.0), cos(camSpeed.y * u_time.time / 18.0));

    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 

    for (var m: i32 = 0; m < AA; m = m + 1) {
        for (var n: i32 = 0; n < AA; n = n + 1) {
            let uv: vec2<f32> = ((FragCoord.xy + vec2<f32>(f32(m), f32(n)) / f32(AA) - 0.5 * resolution) / min(resolution.y, resolution.x) * zoomLevel + pan + camPath) * 2.033 - vec2<f32>(2.14278);
            let z_and_i: vec2<f32> = implicit(uv, u_time.time);
            let iter_ratio: f32 = z_and_i.x / f32(MAX_ITER);
            let lenSq: f32 = z_and_i.y;
            let exteriorColor: vec3<f32> = 0.1 + 0.5 * sin(1.0 + vec3<f32>(params.sigma, params.gamma, params.blue) + PI * vec3<f32>(8.0 * iter_ratio) + u_time.time / 2.0);

            if (iter_ratio >= 1.0) {
                let c1: f32 = pow(clamp(2.00 * sqrt(lenSq), 0.0, 1.0), 0.5);
                let col1: vec3<f32> = 0.5 + 0.5 * sin(1.0 + vec3<f32>(params.sigma, params.gamma, params.blue) + PI * vec3<f32>(2.0 * lenSq) + u_time.time / 2.0);
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
