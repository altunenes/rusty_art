const PI: f32 = 3.1415926535897932384626433832795;
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

fn map(p: vec2<f32>) -> f32 {
    return length(p) - 0.1;
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = (FragCoord.xy * 3.0 - resolution) / min(resolution.x, resolution.y);
    uv -= vec2<f32>(0.3, 0.3);
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let frequency: f32 = params.gamma;
    var t: f32 = 0.6;
    while (t < 400.0) {
        let angle: f32 = u_time.time * 0.05 + t *0.5;
        let rotation: mat2x2<f32> = mat2x2<f32>(
            cos(u_time.time * 0.05), sin(u_time.time * 0.05),
            -sin(u_time.time * 0.05), cos(u_time.time * 0.05)
        );
        uv = (rotation * uv) * params.blue;
        let r: f32 = dot(uv, uv);
        if (r > 5.0) {
            uv *= 5.0 / r;
        }
        uv.x += (params.lambda / t * sin(t * atan2(u_time.time, 1.0) * 2.0 * uv.y + (u_time.time * 0.1)));
        uv.y += (params.theta / t * cos(t * 0.6 * uv.x + (u_time.time * 0.12)));
        col += vec3<f32>(0.2, 0.3, 0.4) / t;
        t *= 1.5;
    }
    let lenSq: f32 = atan2(uv.x, uv.y);
    let col1: vec3<f32> = 0.1 + 0.5 * cos(frequency * (1.0 + u_time.time) + vec3<f32>(0.0, 0.5, 1.0) + PI * vec3<f32>(5.0 * lenSq));
    let col2: vec3<f32> = 0.2 + 0.1 * cos(frequency * (1.1 + u_time.time) + PI * vec3<f32>(lenSq));
    let col3: vec3<f32> = 0.2 + 3.1 * sin(frequency * (1.0 + u_time.time) + vec3<f32>(1.0, 0.5, 0.0) + PI * vec3<f32>(2.0 * sin(lenSq)));
    let col4: vec3<f32> = 0.0 + 0.5 * sin(frequency * (1.0 + u_time.time) + vec3<f32>(0.0, 0.5, 5.0) + PI * vec3<f32>(1.0 * lenSq));
    let col5: vec3<f32> = 0.0 + 0.5 * sin(frequency * (1.1 + u_time.time) + PI * vec3<f32>(lenSq));
    let col6: vec3<f32> = 0.0 + 0.5 * cos(frequency * (1.0 + u_time.time) + vec3<f32>(5.0, 0.5, 0.0) + PI * vec3<f32>(1.0 * sin(lenSq)));
    col += col1 + col2 + col3;
    col /= params.sigma;
    let bg: vec3<f32> = col4 + col5 + col6;
    col = mix(
        col,
        bg,
        1.0 - smoothstep(0.0, abs(sin(u_time.time * 0.05) * params.alpha), map(uv))
    );
    return vec4<f32>(col, 1.0);
}