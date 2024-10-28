const PI: f32 = 3.14159265;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    let uv: vec2<f32> = (FragCoord.xy * 2.0 - resolution) / min(resolution.x, resolution.y);
    let r: f32 = length(uv);
    let spiral = fract(
        fract(r * params.lambda - u_time.time * 0.2) + 
        atan2(uv.y, uv.x) / (2.0 * PI) + 
        u_time.time * params.theta
    );
    let s: f32 = smoothstep(0.5, 0.8, spiral) * 0.5 + 0.5;
    let color1 = vec3<f32>(1.0, params.sigma, params.gamma);
    let color2 = vec3<f32>(params.gamma, params.sigma, params.blue);
    let finalColor = mix(color1, color2, r) * s * smoothstep(1.0, 0.5, r);
    return vec4<f32>(finalColor, 1.0);
}