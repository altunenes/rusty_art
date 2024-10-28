const PI: f32 = 3.141592653589793;
const TAU: f32 = 6.2831855;
const MAX_TRAIL_LENGTH: i32 = 12;

struct TimeUniform {
    time: f32,
};

@group(1) @binding(0) var<uniform> u_time: TimeUniform;

struct Params {
    lambda: f32,
    theta: f32,
    alpha: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};

@group(0) @binding(1) var<uniform> params: Params;

fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn cliffordAttractor(p: vec2<f32>, a: f32, b: f32, c: f32, d: f32) -> vec2<f32> {
    var x = sin(a * p.y) + c * cos(a * p.x);
    var y = sin(b * p.x) + d * cos(b * p.y);
    x = sin(a * y) + c * cos(a * x);
    y = sin(b * x) + d * cos(b * y);
    x = sin(a * y) + c * cos(a * x);
    y = sin(b * x) + d * cos(b * y);
    
    return vec2<f32>(x, y);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = 16.0 * (FragCoord.xy - 0.5 * resolution) / resolution.y;
    let numCircles = 6;
    let numPoints = 15;
    let circleRadius = oscillate(3.5, 4.5, 5.0, u_time.time);
    let intensity = oscillate(0.02, 0.01, 12.0, u_time.time);
    var color = vec3<f32>(0.0);
    let time_offset = u_time.time * 0.1;
    let scale = params.gamma;
    for(var i = 0; i < numCircles; i++) {
        let angle_step = TAU / f32(numPoints);
        let base_radius = f32(i+1) * circleRadius * 0.1;
        let point_color = 0.5 + 0.5 * sin(vec3<f32>(0.1, TAU/3.0, TAU*2.0/3.0) + f32(i) * 0.87);
        for(var j = 0; j < numPoints; j++) {
            let t = f32(j) * angle_step + time_offset;
            let initialPoint = vec2<f32>(cos(t), sin(t)) * base_radius;
            
            let attractorPoint = cliffordAttractor(
                initialPoint,
                params.lambda,
                params.theta,
                params.alpha,
                params.sigma
            ) * scale;
            
            let dist = length(uv + attractorPoint);
            color += point_color * intensity / dist;
        }
    }
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    color = sqrt(color) * params.blue - 1.0;
    return vec4<f32>(color, 1.0);
}