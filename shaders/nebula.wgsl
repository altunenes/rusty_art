const PI: f32 = 3.141592653589793;
const AA: i32 = 4;

struct TimeUniform {
    time: f32,
};
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
@group(1) @binding(0) var<uniform> u_time: TimeUniform;

fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var MAX_ITER: f32 = params.sigma;
    var BOUND: f32 = params.gamma;
    let zoom: f32 = params.alpha;
    let pan: vec2<f32> = vec2<f32>(params.lambda, params.theta);
    let scramb: f32 = osc(0.8, 0.2, 5.0, u_time.time);

    for (var aaX: i32 = 0; aaX < AA; aaX++) {
        for (var aaY: i32 = 0; aaY < AA; aaY++) {
            var uv: vec2<f32> = (FragCoord.xy + vec2<f32>(f32(aaX), f32(aaY)) / f32(AA) - vec2<f32>(0.5)) * vec2<f32>(4.0) / vec2<f32>(1920.0, 1080.0) - vec2<f32>(2.0);
            uv *= zoom;
            uv += pan;

            var z: vec2<f32> = uv;
            let c: vec2<f32> = vec2<f32>(1.0, scramb) * (0.01 + vec2<f32>(sin(u_time.time / 2.0), cos(0.0001 * u_time.time)));

            var i: f32 = 0.0;
            loop {
                if (i >= MAX_ITER) { break; }

                let sin_z: vec2<f32> = vec2<f32>(sin(z.x) * cosh(z.y), cos(z.x) * sinh(z.y));
                z = vec2<f32>(c.x * sin_z.x - c.y * sin_z.y, c.x * sin_z.y + c.y * sin_z.x);

                if (dot(z, z) > BOUND * BOUND) {
                    break;
                }

                i = i + 1.0;
            }

            let iter_ratio: f32 = f32(i) / f32(MAX_ITER);
            let lenSq: f32 = dot(z, z);
            let exteriorColor: vec3<f32> = 0.1 + 0.5 * sin(1.0 + vec3<f32>(0.0, 0.5, 1.0) + PI * vec3<f32>(2.0 * iter_ratio) + u_time.time / 2.0);

            if (iter_ratio >= 1.0) {
                let col1: vec3<f32> = params.blue + 0.5 * sin(1.0 + vec3<f32>(0.0, 0.5, 1.0) + PI * vec3<f32>(2.0 * lenSq) + u_time.time / 2.0);
                let col2: vec3<f32> = params.blue + 0.5 * sin(2.1 + PI * vec3<f32>(lenSq) + u_time.time / 2.0);
                col += 1.5 * sqrt(col1 * col2);
            } else {
                col += exteriorColor;
            }
        }
    }

    col /= f32(AA * AA);
    return vec4<f32>(col, 1.0);
}
