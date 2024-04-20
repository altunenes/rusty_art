const PI: f32 = 3.141592653589793;
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
@group(0) @binding(1)
var<uniform> params: Params;
@group(2) @binding(0)
var<storage, read> data: array<u32, 1458>;
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
fn gaussian(p_in: vec2<f32>, c: vec2<f32>, r_in: vec2<f32>, a_in: f32, anim: f32, currentTime: f32) -> f32 {
    var p = p_in;
    var r = r_in; 
    var a = a_in; 
    let xV1: f32 = osc(params.theta, params.sigma, 5.0, currentTime);
    let xV2: f32 = osc(params.gamma, params.blue, 5.0, currentTime);
    a += anim * 0.05 * cos(currentTime + 10.0 * c.y);
    let an: f32 = a * (2.0 * PI / 256.0);
    let rot: mat2x2<f32> = mat2x2(cos(an), -sin(an), sin(an), cos(an));
    p -= c + vec2<f32>(anim * 255.0 * cos(0.5 * currentTime + 10.0 * c.y), anim * 1.0 * sin(0.5 * currentTime + 10.0 * c.x));
    p = rot * p;
    r.x *= xV2 + xV1 * sin(currentTime + c.x);
    r.y *= xV2 + xV1 * cos(currentTime + c.y);
    p /= r;
    return exp(params.lambda * dot(p, p));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var p = vec2<f32>(FragCoord.x, resolution.y - FragCoord.y) * 2.0 - resolution;
    let uv = 255.0;
    p.y = uv - uv* p.y / resolution.y; 
    p.x = uv - uv * p.x / resolution.y;
    let cycleTime: f32 = u_time.time % 12.0;
    let anim: f32 = smoothstep(1.0, 0.0, abs(6.0 - cycleTime));
    let base: i32 = max(0, 760 - i32(190.0 * cycleTime));
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: i32 = base; i < min(760, base + 760); i++) {
        let xy: u32 = data[3 * i];
        let whag: u32 = data[3 * i + 1];
        let rgb: u32 = data[3 * i + 2];        
        let test = u32(params.alpha);
        let x = (xy >> u32(23)) & u32(511);
        let y = (xy >> u32(test)) & u32(511);
        let w = (whag >> u32(24)) & u32(255);
        let h = (whag >> u32(16)) & u32(255);
        let a = (whag >> u32(8)) & u32(255);
        let g = whag & u32(255);
        let r = (rgb >> u32(16)) & u32(255);
        let b = (rgb >> u32(8)) & u32(255);
        let f: f32 = gaussian(p, vec2<f32>(f32(x), f32(y)), vec2<f32>(f32(w), f32(h)), f32(a), anim, u_time.time);
        if (f > 0.0) {
            col = mix(col, vec3<f32>(f32(r), f32(g), f32(b)) / 255.0, f);
        }
    }
    col = applyGamma(col, 0.5);
    return vec4<f32>(col, 1.0);
}