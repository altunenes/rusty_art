// reference: https://iquilezles.org/articles/tunnel
//note: VOROnoi is useless on here!! ;-)
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};
const PI: f32 = 3.1415927;

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
@group(2) @binding(2)
var<uniform> params: Params;

fn hash(p: vec2<f32>) -> vec2<f32> {
    let q = vec2<f32>(
        dot(p, vec2<f32>(127.1, 311.7)),
        dot(p, vec2<f32>(269.5, 183.3))
    );
    return -1.0 + 2.0 * fract(sin(q) * 43758.5453123);
}

fn voronoi(uv: vec2<f32>) -> f32 {
    let g = floor(uv);
    let f = fract(uv);
    var res = 8.0;
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            let lattice = vec2<f32>(f32(x), f32(y));
            let point = g + lattice + hash(g + lattice);
            res = min(res, length(point - uv));
        }
    }
    return res;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(1920.0, 1080.0);
    let center = vec2<f32>(0.5, 0.5);

    var p = params.gamma * (FragCoord.xy / resolution) - 1.0;

    let a = atan2(p.y, p.x);
    let b = atan2(p.y, abs(p.x));

    let k = params.lambda; 
    let p2 = p * p;
    let p4 = p2 * p2;
    let p8 = p4 * p4;
    let circular_r = length(p);
    let square_r = pow(p8.x + p8.y, params.alpha / 8.0);
    let r = mix(circular_r, square_r, k);  

    let uvL = vec2<f32>(params.theta / r + params.blue * u_time.time, a / PI);
    let uvR = vec2<f32>(0.3 / r + params.blue * u_time.time, b / PI);

    var col = vec3<f32>(voronoi(uvL * 1.0));
    col = textureSampleGrad(tex, tex_sampler, uvL, dpdx(uvR), dpdy(uvR)).xyz;

    col = col * r;

    return vec4<f32>(col, 1.0);
}