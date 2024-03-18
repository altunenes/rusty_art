struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let R: vec2<f32> = vec2<f32>(800.0, 450.0);
    let f: vec2<f32> = FragCoord.xy;
    let v: vec2<f32> = sin(7.0 * (f + f - R) / R.y + u_time.time);
    var c_local: vec4<f32> = vec4<f32>(sin(4.0 * atan2(v.y, v.x) + 2.0 * u_time.time - 16.0 * length(v)) - 0.1, 0.0, 0.0, 0.0);
    c_local = 0.5 + c_local / fwidth(c_local.x);
    return vec4<f32>(c_local.x, c_local.x, c_local.x, 1.0); 
}
