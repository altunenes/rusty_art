const PI: f32 = 3.141592653589793;
const TAU: f32 = 6.2831855;

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
fn hsl2rgb(h: f32, s: f32, l: f32) -> vec3<f32> {
    let rgb = clamp(abs(((h * 6.0 + vec3<f32>(0.0, 4.0, 2.0)) % 6.0) - 3.0) - 1.0, vec3<f32>(0.0), vec3<f32>(1.0));
    return l + s * (rgb - 0.5) * (1.0 - abs(2.0 * l - 1.0));
}

fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = 32.0 * (2.0 * FragCoord.xy - resolution) / resolution.y;
    
    let numCircles = 6;
    let numPoints = 48;
    let xVal1 = oscillate(1.0, 1.0, 15.0, u_time.time);
    let xVal2 = oscillate(12.0, 12.0, 5.0, u_time.time);
    let xVal3 = oscillate(1.0, 2.0, 12.0, u_time.time);
    let xVal12 = oscillate(2.0, 4.0, 12.0, u_time.time);
    let xVal4444 = oscillate(4.5, 4.5, 5.0, u_time.time);
    let ddsad = oscillate(0.05, 0.07, 12.0, u_time.time);
    let frequency = xVal1;
    let amplitude = xVal2;
    let phase = u_time.time * params.alpha;
    let circleRadius = xVal4444 / f32(numCircles);
    var color = vec3<f32>(0.0);
    
    for(var i = 0; i < numCircles; i++) {
        for(var j = 0; j < numPoints; j++) {
            let radius = f32(i) * circleRadius;
            var circlePoint: vec2<f32>;
            circlePoint.x = sin(phase + TAU * f32(-j) / f32(-numPoints)) * radius;
            circlePoint.y = cos(phase + TAU * f32(-j) / f32(-numPoints)) * radius;
            circlePoint.x += sin(params.lambda * PI * frequency * f32(j) / f32(numPoints) + phase) * amplitude;
            circlePoint.y += cos(params.theta * xVal3 * PI * frequency * f32(-j) / f32(numPoints) + phase) * amplitude;
            
            let pointColor = params.sigma + 0.5 * sin(vec3<f32>(1.0, TAU/3.0, TAU*2.0/3.0) + f32(i) * params.gamma);
            let dist = length(uv - circlePoint);
            color += pointColor * ddsad / dist;
        }
    }
    
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    color = sqrt(color) * params.blue - 1.0;
    
    return vec4<f32>(color, 1.0);
}