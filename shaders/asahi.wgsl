//The Asahi illusion reveals how brightness perception shifts with yellow's placement and underscores that 
//pupil size indicates both light reception and cerebral interpretation. 
//Note the central area remains consistently white, though it may seem brighter/darker over time.

// Global constants
const PI: f32 = 3.14159265;

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
fn curve(x: f32, a: f32, b: f32) -> f32 {
    let y: f32 = smoothstep(a, b, x) * smoothstep(b, a, x);
    return pow(y, 0.08);
}
fn drawLeaf(uv: vec2<f32>, ls: f32, le: f32, lw: f32, ang: f32, time: f32, isLeft: bool) -> vec3<f32> {
    let cosA: f32 = cos(ang);
    let sinA: f32 = sin(ang);
    let rotuv: vec2<f32> = vec2(cosA * uv.x - sinA * uv.y, sinA * uv.x + cosA * uv.y);
    let angle: f32 = atan2(rotuv.y, rotuv.x);
    let radius: f32 = length(rotuv);
    let angleMod: f32 = angle % (2.0 * PI);
    let normalang: f32 = angleMod / (2.0 * PI);
    let leafRadius: f32 = mix(ls, le, curve(normalang, 0.5 - (lw / 2.0), 0.5 + (lw / 16.0)));
    let withinLeaf: bool = radius >= ls && radius <= leafRadius;
    let firstcol: f32 = smoothstep(ls, le, radius);
    let colorShift: f32 = params.lambda + 0.5 * sin(time * params.theta * PI); 
    var animcol: f32 = firstcol;
    if (isLeft) {
        animcol = mix(firstcol, 1.0 - firstcol, colorShift);
    } else {
        animcol = mix(1.0 - firstcol, firstcol, colorShift);
    }

    var first_color: vec3<f32>;
    var ended_color: vec3<f32>;
    if (isLeft) {
        first_color = vec3<f32>(params.sigma, params.gamma, params.blue);
        ended_color = vec3<f32>(0.0, 0.0, 0.0);
    } else {
        first_color = vec3<f32>(0.0, 0.0, 0.0);
        ended_color = vec3<f32>(params.sigma, params.gamma, params.blue);
    }

    var leafColor: vec3<f32>;
    if (withinLeaf) {
        leafColor = mix(first_color, ended_color, animcol);
    } else {
        leafColor = vec3<f32>(1.0, 1.0, 1.0);
    }

    return leafColor;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = (FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x);
    uv.y += 0.25;

    var col = vec3<f32>(1.0, 1.0, 1.0);

    let n = 32;

    var uvLeft = uv - vec2<f32>(0.25, 0.25);
    var uvRight = uv - vec2<f32>(1.80, 0.25);
    for (var i: i32 = 0; i < n; i = i + 1) {
        let oriant = 50.0 * PI * f32(i) / f32(n);
        let lefone = drawLeaf(uvLeft, 0.2, 0.7, 0.05, oriant, u_time.time  / 5.0, true);
        let rightone = drawLeaf(uvRight, 0.2, 0.7, 0.05, oriant, -u_time.time  / 5.0, false);
        col = min(col, lefone);
        col = min(col, rightone);
    }
    return vec4<f32>(col, 1.0);
}