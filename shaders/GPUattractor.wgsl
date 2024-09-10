const PI: f32 = 3.141592653589793;
const TAU: f32 = 6.2831855;
const MAX_TRAIL_LENGTH: i32 = 3;

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
    let x = sin(a * p.y) + c * cos(a * p.x);
    let y = sin(b * p.x) + d * cos(b * p.y);
    return vec2<f32>(x, y);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = 16.0 * (FragCoord.xy - 0.5 * resolution) / resolution.y;
    
    let numCircles = 6;
    let numPoints = 15;
    let xVal4444 = oscillate(4.5, 4.5, 5.0, u_time.time);
    let ddsad = oscillate(0.02, 0.01, 12.0, u_time.time);
    let circleRadius = xVal4444 / f32(numCircles);
    var color = vec3<f32>(0.0);

    let a = params.lambda;
    let b = params.theta;
    let c = params.alpha;
    let d = params.sigma;
    let scale = params.gamma;

    for(var i = 0; i < numCircles; i++) {
        for(var j = 0; j < numPoints; j++) {
            var trailPoints: array<vec2<f32>, MAX_TRAIL_LENGTH>;
            
            for(var k = 0; k < MAX_TRAIL_LENGTH; k++) {
                let t = f32(j) / f32(numPoints) * TAU + u_time.time * 0.1 - f32(k) * 0.02;
                let initialPoint = vec2<f32>(cos(t), sin(t)) * f32(i+1) * circleRadius * 0.2;
                
                var attractorPoint = initialPoint;
                for(var l = 0; l < 10; l++) {
                    attractorPoint = cliffordAttractor(attractorPoint, a, b, c, d);
                }
                
                trailPoints[k] = -attractorPoint * scale;
            }
            
            for(var k = 0; k < MAX_TRAIL_LENGTH; k++) {
                let circlePoint = trailPoints[k];
                let pointColor = 0.5 + 0.5 * sin(vec3<f32>(1.0, TAU/3.0, TAU*2.0/3.0) + f32(i) * 0.87);
                let trailFade = 1.0 - f32(k) / f32(MAX_TRAIL_LENGTH);
                let dist = length(uv - circlePoint);
                color += pointColor * ddsad / dist * trailFade;
            }
        }
    }

    color = clamp(color, vec3<f32>(0.0), vec3<f32>(1.0));
    color = sqrt(color) * params.blue - 1.0;
    return vec4<f32>(color, 1.0);
}