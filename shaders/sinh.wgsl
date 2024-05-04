const PI: f32 = 3.141592653589793;
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

fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn c_mul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn c_sinh(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(sinh(z.x) * cos(z.y), cosh(z.x) * sin(z.y));
}

fn c_abs(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(abs(sin(z.x)), abs(sin(z.y)));
}

fn c_sinh_pow4(z: vec2<f32>) -> vec2<f32> {
    let sinh_z = c_sinh(z);
    return c_mul(c_mul(sinh_z, sinh_z), c_mul(sinh_z, sinh_z));
}
fn implicit(z: vec2<f32>, c: vec2<f32>, time: f32) -> vec2<f32> {
var MI: i32 = i32(params.iter);
var B: f32 = params.bound;
    var z_local = z; 
    var i: i32 = 0;
    loop {
        if (i >= MI) { break; } 
        z_local = c_abs(c_sinh_pow4(z_local)) + c;
        z_local = z_local + 0.03 * vec2<f32>(cos(1.05 * time / params.lambda), cos(1.05 * time / params.lambda));
        if (dot(z_local, z_local) > B * B) {
            break;
        }
        i = i + 1;
    }
    return vec2<f32>(f32(i), dot(z_local, z_local));
}
fn modf(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var MI: i32 = i32(params.iter);
    var B: f32 = params.bound;
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let AA: i32 = i32(params.aa);
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let time: f32 = u_time.time/params.tt; 
    let gradient0: vec3<f32> = vec3<f32>(params.a, params.b, params.c);
    let gradient1: vec3<f32> = vec3<f32>(0.4, 0.2, 0.0);
    let gradient2: vec3<f32> = vec3<f32>(0.7, 0.35, 0.15);
    for (var m: i32 = 0; m < AA; m = m + 1) {
        for (var n: i32 = 0; n < AA; n = n + 1) {
            let uv: vec2<f32> = ((FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x) * 2.0) * 0.5;
            let c_value: f32 = mix(2.197, 2.99225, 0.01 + 0.01 * sin(0.1 * time / params.lambda));
            let oscillation: f32 = params.theta + params.alpha * (sin(0.1 * time / params.lambda) +params.blue);
            let c: vec2<f32> = vec2<f32>(oscillation, c_value);
            let z_and_i = implicit(uv, c, time);
            let iter_ratio: f32 = z_and_i.x / f32(MI);
            let lenSq: f32 = z_and_i.y;
            let gradientIndex: f32 = modf(iter_ratio * 24.0, 12.0);
            let index1: i32 = i32(gradientIndex);
            let index2: i32 = i32(modf(gradientIndex + 1.0, 3.0));
            let blend: f32 = fract(gradientIndex);
            var col1: vec3<f32>;
            var col2: vec3<f32>;
            if (index1 == 0) { col1 = gradient0; }
            else if (index1 == 1) { col1 = gradient1; }
            else { col1 = gradient2; }
            if (index2 == 0) { col2 = gradient0; }
            else if (index2 == 1) { col2 = gradient1; }
            else { col2 = gradient2; }
            let col4: vec3<f32> = mix(col1, col2, blend);
            let baseColor: vec3<f32> = 0.5 + 0.5 * cos(params.d + time + vec3<f32>(params.e, params.f, params.g) + PI * vec3<f32>(params.sigma * lenSq));
            col = col + sqrt(baseColor * col4);
        }
    }
    col = sqrt(col / f32(AA * AA));
    col = applyGamma(col, params.gamma);
    return vec4<f32>(col, 1.0);
}
