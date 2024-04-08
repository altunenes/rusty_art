//One of my oldest shaders, you can find the online versions of both GLSL and WGSL online:
//https://compute.toys/view/713; https://www.shadertoy.com/view/ctK3DV
// Global constants
const PI: f32 = 3.14159265;
const L: f32 = 0.7; //

// TimeUniform definition based on my wgpu rust code
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
    blue:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
fn implicit(x: f32, y: f32) -> f32 {
    let t: f32 = u_time.time / params.lambda;
    let n1: f32 = 6.0 + 3.0 * sin(t);
    let m1: f32 = 4.0 + 3.0 * cos(t);
    let n2: f32 = 5.0 + 2.5 * cos(2.0 * t);
    let m2: f32 = 3.0 + 2.5 * sin(2.0 * t);
    let val1: f32 = cos(n1 * PI * x / L) * cos(m1 * PI * y / L) -
                    cos(m1 * PI * x / L) * cos(n1 * PI * y / L);
    let val2: f32 = cos(n2 * PI * x / L) * cos(m2 * PI * y / L) -
                    cos(m2 * PI * x / L) * cos(n2 * PI * y / L);
    return val1 + val2;
}
fn delf_delx(x: f32, y: f32) -> f32 {
    let dx: f32 = 0.001;
    return (implicit(x + dx, y) - implicit(x - dx, y)) / (2.5 * dx);
}
fn delf_dely(x: f32, y: f32) -> f32 {
    let dy: f32 = 0.001; 
    return (implicit(x, y + dy) - implicit(x, y - dy)) / (2.5 * dy);
}
fn gradient(x: f32, y: f32) -> vec2<f32> {
    return vec2<f32>(delf_delx(x, y), delf_dely(x, y));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let uv: vec2<f32> = (FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);
    let g: vec2<f32> = gradient(uv.x, uv.y);
    let unit: f32 = 25.0 / resolution.y;
    let sharpVal: f32 = smoothstep(-unit, unit, abs(implicit(uv.x, uv.y)) / sqrt(g.x * g.x + g.y * g.y));
    let col: vec3<f32> = 0.5 + 0.5 * cos(u_time.time + vec3<f32>(params.sigma, params.gamma, params.blue) + params.theta * PI * vec3<f32>(sharpVal));
    return vec4<f32>(col, 1.0);
}