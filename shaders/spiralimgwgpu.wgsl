@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

const PI: f32 = 3.14159265359;
const SLOPE: f32 = 35.0;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(2) @binding(2)
var<uniform> params: Params;

fn smooth_triangle(x: f32) -> f32 {
    let f = fract(x - 0.5);
    let df = fwidth(x) * 2.0;
    return mix(abs(f - 0.5) * 2.0, 0.5, smoothstep(0.0, 1.0, df));
}

struct SpiralResult {
    pattern: f32,
    coord: vec2<f32>,
}

fn spiral(pos: vec2<f32>, slope: f32, resolution: vec2<f32>) -> SpiralResult {
    let l = length(pos);
    let ang = atan2(pos.y, pos.x) + 5.0 * u_time.time;
    
    let r = smooth_triangle(ang / (2.0*PI) + l / slope);
    
    let phase = ang / 6.28318531;
    let segment = floor(l / slope + fract(phase));
    let blend = fract(phase);
    
    var coord = normalize(pos) * (segment - blend + 0.5) * slope;
    coord = (coord + 0.5 * resolution) / resolution;
    coord = clamp(coord, vec2<f32>(0.0), vec2<f32>(1.0));
    
    return SpiralResult(r, coord);
}

fn sample_texture_smooth(uv: vec2<f32>, blur_amount: f32) -> vec3<f32> {
    let pixel_size = 1.0 / vec2<f32>(1920.0, 1080.0);
    var color = vec3<f32>(0.0);
    let samples = 5;
    
    for(var i = -samples; i <= samples; i++) {
        for(var j = -samples; j <= samples; j++) {
            let offset = vec2<f32>(f32(i), f32(j)) * pixel_size * blur_amount;
            color += textureSample(tex, tex_sampler, uv + offset).xyz;
        }
    }
    
    return color / f32((2 * samples + 1) * (2 * samples + 1));
}

fn luminance(uv: vec2<f32>) -> vec3<f32> {
    let c = sample_texture_smooth(uv, 2.0);
    
    let d = clamp(dot(c.xyz, vec3<f32>(-0.25, 0.5, -0.25)), 0.0, 1.0);
    var color = mix(c, vec3<f32>(1.5), params.theta * d * 0.7); 
    
    let luma = dot(color, vec3<f32>(0.299, 0.587, 0.114));
    color = mix(color, vec3<f32>(luma), 0.7); 
    
    return clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn gradyy(fc: vec2<f32>, eps: f32, resolution: vec2<f32>) -> vec2<f32> {
    let e = vec2<f32>(eps, 0.0);
    
    let col = sample_texture_smooth(fc / resolution, 1.5);
    
    let grad_x = dot(
        sample_texture_smooth((fc + e.xy) / resolution, 1.5) - 
        sample_texture_smooth((fc - e.xy) / resolution, 1.5),
        vec3<f32>(0.299, 0.587, 0.114)
    );
    
    let grad_y = dot(
        sample_texture_smooth((fc + e.yx) / resolution, 1.5) - 
        sample_texture_smooth((fc - e.yx) / resolution, 1.5),
        vec3<f32>(0.299, 0.587, 0.114)
    );

    return vec2<f32>(grad_x, grad_y) / (3.0 * eps);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(1920.0, 1080.0);
    let centered_coord = FragCoord.xy - 0.5 * resolution;
    
    let spiral_result = spiral(centered_coord, params.lambda, resolution);
    let pattern = vec3<f32>(spiral_result.pattern);
    let uv = spiral_result.coord;
    
    let col = luminance(uv);
    
    let b = params.alpha * (1.0 - col.x) + 0.35;
    var c = clamp(pattern.x - 1.0 + b, 0.0, 1.0);
    c = b - (b - c) * (b - c) / b / b;
    c = smoothstep(0.0, 1.0, c);
    let base_color = vec3<f32>(1.0 - c);
    let light = normalize(vec3<f32>(0.5, 0.5, 2.0));
    let grad = gradyy(FragCoord.xy, 1.8, resolution);
    let n = normalize(vec3<f32>(grad, 1.2));
    let spec = dot(reflect(vec3<f32>(0.0, 0.0, -1.0), n), light);
    let diff = clamp(dot(light, n), 0.0, 1.0);
    let fegg = smoothstep(0.5, 1.0, base_color.x);
    let spec_final = pow(clamp(spec, 0.0, 1.0), mix(1.0, 150.0, 1.0 - fegg)) * mix(1.0, 150.0, 1.0 - fegg) / 120.0;
    let final_color = mix(
        vec3<f32>(params.sigma, params.gamma, params.blue),
        vec3<f32>(1.0, 0.97, 0.9) * params.theta,
        smoothstep(0.0, 1.0, fegg)
    );
    let vignette = cos(1.7 * length((FragCoord.xy - 0.5 * resolution) / resolution.x));
    let vignette_final = smoothstep(0.0, 1.0, vignette);
    
    return vec4<f32>((final_color * diff + 0.7 * spec_final) * vignette_final, 1.0);
}