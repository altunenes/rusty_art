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

fn TAU() -> f32 {
    return 6.2831855;
}

fn implicit(x: f32, y: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    return b1 * sin(a1 * TAU() * x) * sin(a2 * TAU() * y) + b2 * sin(a2 * TAU() * x) * sin(a1 * TAU() * y);
}

fn delf_delx(x: f32, y: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    let dx: f32 = 0.001;
    return (implicit(x + dx, y, a1, a2, b1, b2) - implicit(x - dx, y, a1, a2, b1, b2)) / (2.0 * dx);
}

fn delf_dely(x: f32, y: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    let dy: f32 = 0.001;
    return (implicit(x, y + dy, a1, a2, b1, b2) - implicit(x, y - dy, a1, a2, b1, b2)) / (2.0 * dy);
}

fn gradient(x: f32, y: f32, a1: f32, a2: f32, b1: f32, b2: f32) -> vec2<f32> {
    return vec2<f32>(delf_delx(x, y, a1, a2, b1, b2), delf_dely(x, y, a1, a2, b1, b2));
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let uv: vec2<f32> = (FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);

    let t: f32 = u_time.time / params.lambda;
    let a1: f32 = params.sigma + sin(t);
    let a2: f32 = params.gamma + cos(t);
    let b1: f32 = tanh(2.0 * t);
    let b2: f32 = tanh(2.0 * t);

    let g: vec2<f32> = gradient(uv.x, uv.y, a1, a2, b1, b2);
    let unit: f32 = params.theta / resolution.y;
    let sharpVal: f32 = smoothstep(-unit, unit, abs(implicit(uv.x, uv.y, a1, a2, b1, b2)) / sqrt(g.x * g.x + g.y * g.y));
    let col: vec3<f32> = 0.5 + 0.5 * cos(u_time.time + vec3<f32>(0.6, 0.8, 1.0) + params.blue * TAU() * vec3<f32>(sharpVal));

    return vec4<f32>(col, 1.0);
}
