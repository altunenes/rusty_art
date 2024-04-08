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
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let R: vec2<f32> = vec2<f32>(800.0, 450.0);
    let f: vec2<f32> = FragCoord.xy;
    let v: vec2<f32> = sin(params.lambda * (f + f - R) / R.y + u_time.time);
    var c_local: vec4<f32> = vec4<f32>(sin(params.theta * atan2(v.y, v.x) + params.alpha* u_time.time - params.sigma * length(v)) - params.gamma, 0.0, 0.0, 0.0);
    c_local = params.blue + c_local / fwidth(c_local.x);
    return vec4<f32>(c_local.x, c_local.x, c_local.x, 1.0); 
}
