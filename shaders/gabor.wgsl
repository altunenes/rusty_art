struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let PI: f32 = 3.1415926535897932384626433832795;
    let resolution: vec2<f32> = vec2<f32>(512.0, 512.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution) * 2.0 - vec2<f32>(1.0, 1.0);
    let lambda: f32 = params.lambda; 
    let theta: f32 = params.theta; // Orientation
    let psi: f32 = u_time.time * 5.5;
    let sigma: f32 = params.sigma; // Standard deviation of the Gaussian envelope
    let gamma: f32 = 1.0; // Spatial aspect ratio
    // Rotation transformation
    let xp: f32 = uv.x * cos(theta) - uv.y * sin(theta);
    let yp: f32 = uv.x * sin(theta) + uv.y * cos(theta);
    // Gabor function
    let envelope: f32 = exp(-((xp * xp) + (gamma * gamma * yp * yp)) / (2.0 * sigma * sigma));
    let carrier: f32 = cos(2.0 * PI * xp / lambda + psi);
    let gabor: f32 = envelope * carrier;
    let colorModulation: vec3<f32> = vec3<f32>(0.5) + vec3<f32>(0.5) * cos(1.5 * PI * xp / lambda + vec3<f32>(0.0, 2.0, 4.0));
    let col: vec3<f32> = 0.5 + 0.5 * gabor * colorModulation;
    return vec4<f32>(col, 1.0);
}