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

fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn GaborPatch(uv: vec2<f32>, theta: f32, lambda: f32, sigma: f32, gamma: f32, psi: f32) -> f32 {
    let xp: f32 = uv.x * cos(theta) - uv.y * sin(theta);
    let yp: f32 = uv.x * sin(theta) + uv.y * cos(theta);
    let envelope: f32 = exp(-((xp * xp) + (gamma * gamma * yp * yp)) / (2.0 * sigma * sigma));
    let carrier: f32 = cos(2.0 * PI * xp / lambda + psi);
    return envelope * carrier;
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5);
    var color: vec3<f32> = vec3<f32>(0.5, 0.5, 0.5);
    let lambda: f32 =params.lambda;
    let sigma: f32 = params.sigma;
    let gamma: f32 = params.gamma;
    let psi: f32 = u_time.time* params.blue;

    for (var i: i32 = 0; i < 2; i = i + 1) {
        var sign: f32 = 0.0;
        if (i == 0) {
            sign = 1.3;
        } else {
            sign = -1.3;
        }
        for (var t: f32 = -0.6; t <= 0.6; t = t + 0.05) {
            let linePos: vec2<f32> = vec2<f32>(1.8 * t - 0.6, t * sign * (resolution.y / resolution.x));
            let theta: f32 = sign * PI / params.alpha;
            let patchUV: vec2<f32> = uv - linePos;
            let gabor: f32 = GaborPatch(patchUV, theta, lambda, sigma, gamma, psi);
            color = color + 0.5 * vec3<f32>(gabor, gabor, gabor);
        }
    }

    let ballX: f32 = sin(u_time.time * params.theta) * 0.3 + 0.1;
    let distance: f32 = length(uv - vec2<f32>(ballX, 0.0));
    let ball: f32 = smoothstep(0.02, 0.01, distance);
    let red: vec3<f32> = vec3<f32>(1.0, 0.0, 0.0);
    color = mix(color, red, ball);
    color = applyGamma(color,0.5);
    return vec4<f32>(color, 1.0);
}