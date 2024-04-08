const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn rotate2D(r: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(r), -sin(r), sin(r), cos(r));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5 * resolution.y / resolution.x);
    uv *= 1.5;
    let t: f32 = u_time.time;
    let xVal4: f32 = oscillate(params.lambda, params.lambda, 15.0, t);
    let xVal6: f32 = oscillate(params.blue, params.blue, 15.0, t);
    let xVal7: f32 = oscillate(0.5, 3.51, 10.0, t);
    var p: vec2<f32> = uv;
    var n: vec2<f32> = vec2<f32>(0.0, 0.0);
    var N: vec2<f32> = vec2<f32>(0.5, 0.5);
    let m: mat2x2<f32> = rotate2D(xVal4);
    var S: f32 = xVal6;
    var branchFactor: f32 = params.theta;
    for (var j: f32 = 0.0; j < 25.0; j += 1.0) {
        p = p * m;
        n = n * m;
        let q: vec2<f32> = p * S * j + n + vec2<f32>(t, t);

        n += branchFactor * sin(q);
        N += branchFactor * cos(q) / S * xVal7;
        S *= 1.245 * tanh(2.975);
    }
    let colorOffset: vec3<f32> = vec3<f32>(
        0.01 * smoothstep(0.0, 5.0, sin(n.x)),
        0.05 * smoothstep(0.0, 0.0, sin(n.y)),
        0.01 * smoothstep(0.0, 5.0, cos(n.x))
    );
    let flowColorChange: vec3<f32> = vec3<f32>(
        0.05 * cos(3.0 * t + N.x),
        0.05 * sin(1.0 * t + N.y),
        0.05 * cos(0.0 * t + N.y)
    );
    let flowIntensity: vec3<f32> = vec3<f32>(
        0.0001 / length(1.03 * N),
        smoothstep(0.5, 5.0, N.x),
        smoothstep(0.5, 0.0, N.y)
    );
    let axonColor: vec3<f32> = vec3<f32>(0.02, 0.1, 0.0);
    let axonEffect: vec3<f32> = smoothstep(4.1, 2.55,25.0) * axonColor;
    var col: vec3<f32> = (vec3<f32>(0.5, 0.0 , 0.1) *
        colorOffset + flowColorChange + flowIntensity) *
        ((params.gamma * N.x *  params.gamma* N.y + params.sigma) + params.alpha / length(1.0 * N)) + axonEffect;
    return vec4<f32>(col, 1.0);
}