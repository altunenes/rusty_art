const PI: f32 = 3.1415926535897932384626433832795;
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
fn oscilation(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    var p: vec2<f32> = (2.0 * vec2<f32>(FragCoord.x, resolution.y - FragCoord.y) - resolution) / min(resolution.y, resolution.x);
    p *= 0.2 * oscilation(0.01, params.lambda, 10.5, u_time.time);
    p.y = -0.1 - p.y * 1.2 + abs(p.x) * (1.0 - abs(p.x));
    let r: f32 = length(p);
    let mX: f32 = (sin(u_time.time * 0.5) + 1.0) * params.theta;
    let mY: f32 = (cos(u_time.time * 0.5) + 1.0) * params.theta;
    var hcol: vec3<f32> = vec3<f32>((FragCoord.xy) / resolution.y, mX);
    for (var i: i32 = 0; i < 45; i++) {
        let xVal: f32 = oscilation(1.1, 1.4, 8.0, u_time.time);
        let yVal: f32 = oscilation(params.alpha, params.sigma, 8.0, u_time.time);
        let zVal: f32 = oscilation(params.gamma, params.blue, 8.0, u_time.time);
        let tempHcol: vec3<f32> = vec3<f32>(xVal, yVal, zVal) * (abs((abs(hcol) / abs(dot(hcol, hcol))) - vec3<f32>(1.0, 1.0, mY)));
        hcol.x = tempHcol.x;
        hcol.z = tempHcol.y;
        hcol.y = tempHcol.z;
    }
    var bcol: vec3<f32> = vec3<f32>(0.0);
    let gradientFactor: f32 = FragCoord.y / resolution.y;
    var tempBcol: vec3<f32> = vec3<f32>(1.3, 0.850, 0.3) - gradientFactor;
    bcol.x = tempBcol.x - gradientFactor;
    bcol.y = tempBcol.y - gradientFactor;
    bcol.z = tempBcol.z - gradientFactor;
    bcol += vec3<f32>(gradientFactor, gradientFactor, gradientFactor);
    bcol.r *= 1.01;
    bcol.g *= sin(u_time.time) * 0.5 + 0.5;
    bcol.b *= cos(u_time.time) * 0.5 + 0.5;
    var col: vec3<f32> = mix(bcol, hcol, smoothstep(-0.15, 0.15, (0.5 - r)));
        col = applyGamma(col, 0.5);
    return vec4<f32>(col, 1.0);
}