const PI: f32 = 3.1415926535897932384626433832795;

struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

fn oscilation(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 600.0); 
    var p: vec2<f32> = (2.0 * vec2<f32>(FragCoord.x, resolution.y - FragCoord.y) - resolution) / min(resolution.y, resolution.x);
    p *= 0.2 * oscilation(0.01, 5.0, 10.5, u_time.time);
    p.y = -0.1 - p.y * 1.2 + abs(p.x) * (1.0 - abs(p.x));
    let r: f32 = length(p);
    let mX: f32 = (sin(u_time.time * 0.5) + 1.0) * 0.2;
    let mY: f32 = (cos(u_time.time * 0.5) + 1.0) * 0.2;
    var hcol: vec3<f32> = vec3<f32>((FragCoord.xy) / resolution.y, mX);
    for (var i: i32 = 0; i < 45; i++) {
        let xVal: f32 = oscilation(1.1, 1.4, 8.0, u_time.time);
        let yVal: f32 = oscilation(0.850, 0.899, 8.0, u_time.time);
        let zVal: f32 = oscilation(0.6, 0.8, 8.0, u_time.time);
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
    let col: vec3<f32> = mix(bcol, hcol, smoothstep(-0.15, 0.15, (0.5 - r)));
    return vec4<f32>(col, 1.0);
}