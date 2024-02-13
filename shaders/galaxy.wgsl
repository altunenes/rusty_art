
//my first shadertoy code: https://www.shadertoy.com/view/DdyXDt citations:
//some noise functions and Brownian motion: frankenburgh, (2016) https://www.shadertoy.com/view/lty3Rt 
// Dave_Hoskins, (2014) https://www.shadertoy.com/view/MdXSzS ; FabriceNeyret2, (2014) https://www.shadertoy.com/view/XdsSRS
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn fbmslow(p: vec2<f32>) -> f32 {
    var f: f32 = 0.0;
    var p_mod: vec2<f32> = p; 
    f += 1.001 * length(p_mod);
    p_mod = p_mod * 2.02; 
    f += 2.001 * length(p_mod);
    p_mod = p_mod * 2.03;
    f += 0.1250 * length(p_mod);
    p_mod = p_mod * 0.001;
    f += 0.0625 * length(p_mod);
    return f / 2.9375;
}
fn noise(p: vec2<f32>, iTime: f32) -> f32 {
    return fbmslow(p + iTime * 0.05);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let PI: f32 = 3.1415926535897932384626433832795;
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0); 
    let lightDirection: vec3<f32> = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let zoomLevel: f32 = 1.0; 
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5);
    let distanceFromCenter: f32 = length(uv) * 1.0;
    let flow: vec2<f32> = vec2<f32>(noise(uv, u_time.time), noise(uv + vec2<f32>(0.1, 0.1), u_time.time));
    let timeFactor: f32 = 0.1 * u_time.time;
    let adjustedTime: f32 = timeFactor + (5.0 + sin(timeFactor)) * 0.1 / (distanceFromCenter + 0.07);
    let sineTime: f32 = sin(adjustedTime);
    let cosineTime: f32 = cos(adjustedTime);
    uv = uv * mat2x2(cosineTime, sineTime, -sineTime, cosineTime);
    uv = uv + flow * 0.008;
    var baseColor: f32 = 0.0;
    var color1: f32 = 0.0;
    var color2: f32 = 0.0;
    var color3: f32 = 0.0;
    var point: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: i32 = 0; i < 150; i = i + 1) {
        point = 0.09 * f32(i) * vec3<f32>(uv, 1.0);
        point = point + vec3<f32>(0.1, 0.01, -3.5 - sin(timeFactor * 0.1) * 0.01);
        for (var j: i32 = 0; j < 11; j = j + 1) {
            point = abs(point) / dot(point, point) - 0.52;
        }
        let pointIntensity: f32 = dot(point, point) * 0.000828;
        color1 = color1 + pointIntensity * (3.8 + sin(distanceFromCenter * 13.0 + 3.5 - timeFactor * 2.0));
        color2 = color2 + pointIntensity * (1.5 + sin(distanceFromCenter * 13.5 + 2.2 - timeFactor * 3.0));
        color3 = color3 + pointIntensity * (2.4 + sin(distanceFromCenter * 14.5 + 1.5 - timeFactor * 2.5));
    }
    baseColor = (3.1 / (1.3 + zoomLevel)) * length(vec2<f32>(point.x, point.y)) * 0.12;
    color1 = color1 * 0.5;
    color2 = color2 * 0.5; 
    color3 = smoothstep(0.18, 0.0, distanceFromCenter);
    color3 = color3 * 0.3;
    var direction: vec3<f32> = normalize(vec3<f32>(uv, 0.0)); 
    let sundot: f32 = dot(lightDirection, direction);
    var finalColor: vec3<f32> = vec3<f32>(baseColor, (color1 + baseColor) * 0.25, color2);
    finalColor = finalColor + color3 * 2.9;
    finalColor.g = finalColor.g + color3 * 0.45;
    return vec4<f32>(finalColor, 1.0);
}