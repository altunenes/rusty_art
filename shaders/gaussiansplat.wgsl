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
fn gaussian(p: vec2<f32>, c: vec2<f32>, r: vec2<f32>, a: f32, anim: f32) -> f32 {
    let angle: f32 = 0.5 * u_time.time + 4.0 * c.y;
    let cos_val: f32 = cos(angle);
    let sin_val: f32 = sin(angle);
    let animatedPos = p - (c + anim * 250.0 * vec2<f32>(cos_val, sin_val));
    let an = a * (2.0* PI / 256.0) + anim * 0.01 * c.x;
    let rot = mat2x2<f32>(cos(an), -sin(an), sin(an), cos(an));
    let transformedP = rot * (animatedPos / r);
    let g = exp(params.lambda * dot(transformedP, transformedP));
    let test2= params.sigma;
    return select(-0.1, g, g > test2);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var p = vec2<f32>(FragCoord.x, resolution.y - FragCoord.y) * 2.0 - resolution;
    let uv = params.theta;
    p.y = uv - uv* p.y / resolution.y; 
    p.x = uv - uv * p.x / resolution.y;
    let cycleTime: f32 = u_time.time % 6.0;
    let anim = smoothstep(params.gamma, params.blue, abs(3.0 - cycleTime));
    var col = vec3<f32>(0.0, 0.0, 0.0);
    let dataCount: i32 = 1458;
    for (var i: i32 = 0; i < dataCount; i += 3) {
        let xy = data[i];
        let whag = data[i + 1];
        let rgb = data[i + 2];
        let test = u32(params.alpha);
        let x = (xy >> u32(23)) & u32(511);
        let y = (xy >> u32(test)) & u32(511);
        let w = (whag >> u32(24)) & u32(255);
        let h = (whag >> u32(16)) & u32(255);
        let a = (whag >> u32(8)) & u32(255);
        let g = whag & u32(255);
        let r = (rgb >> u32(16)) & u32(255);
        let b = (rgb >> u32(8)) & u32(255);

        let f = gaussian(p, vec2<f32>(f32(x), f32(y)), vec2<f32>(f32(w), f32(h)), f32(a), anim);
        if (f > params.sigma) {
            col = mix(col, vec3<f32>(f32(r), f32(g), f32(b)) / 255.0, f);
        }
    }
    col = applyGamma(col, 0.5);
    return vec4<f32>(col, 1.0);
}
