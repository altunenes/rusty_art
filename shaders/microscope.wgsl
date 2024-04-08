const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
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
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn fbm(p_initial: vec2<f32>) -> f32 {
    var value: f32 = 0.0;
    var amplitude: f32 = 0.5;
    var p: vec2<f32> = p_initial;
    for (var i: i32 = 0; i < 5; i = i + 1) {
        value = value + amplitude;
        p = p * 2.0;
        amplitude = amplitude * 0.5;
    }
    return value;
}
fn rotate2D(r: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(r), -sin(r), sin(r), cos(r));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5 * resolution.y / resolution.x);
    uv *= 1.5;
    let t: f32 = u_time.time;
    let xVal4: f32 = oscillate(0.5, 0.5, 15.0, t);
    let xVal6: f32 = oscillate(4.0, 1.51, 15.0, t);
    let xVal7: f32 = oscillate(params.lambda, params.theta, 10.0, t);

    var p: vec2<f32> = uv;
    var n: vec2<f32> = vec2<f32>(0.0, 0.0);
    var N: vec2<f32> = vec2<f32>(1.5, 0.5);
    let m: mat2x2<f32> = rotate2D(xVal4);
    var S: f32 = xVal6;
    var branchFactor: f32 = 1.78;

    for (var j: f32 = 0.0; j < 45.0; j += 1.0) {
        p = p * m;
        n = n * m;
        let q: vec2<f32> = p * S * j + n + vec2<f32>(t, t);

        n += branchFactor * sin(q);
        N += branchFactor * cos(q) / S * xVal7;
        S *= 1.245 * tanh(params.blue);
    }
    let baseColor: vec3<f32> = vec3<f32>(0.1, 0.2, 0.5); 
    let colorVariation: vec3<f32> = vec3<f32>(
        params.alpha * sin(1.0 + N.x),
        params.sigma * sin(1.0 + N.y),
        params.gamma * cos(1.0 + N.y)
    );
    var col: vec3<f32> = baseColor + colorVariation;
    let complementaryColor: vec3<f32> = vec3<f32>(0.1, 0.2, 0.1); 
    let complementaryVariation: f32 = 1.5 + 0.0 * sin(2.*PI * uv.x * uv.y + t); //0.0 * (the custom...)
    let xVal1: f32 = oscillate(0.2, 0.001, 5.0, t);
    let distanceFromCenter: f32 = length(uv - vec2<f32>(0.2, 0.2));
    let complementaryIntensity: f32 = smoothstep(0.1, 0.35, xVal1);
    col = mix(col, complementaryColor, complementaryVariation * complementaryIntensity);
    col = applyGamma(col,0.5);
    return vec4<f32>(col, 1.0);
}