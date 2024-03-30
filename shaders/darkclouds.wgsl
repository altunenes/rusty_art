const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

fn random(st: vec2<f32>) -> vec2<f32> {
    let st_transformed: vec2<f32> = vec2<f32>(
        dot(st, vec2<f32>(127.1, 311.7)),
        dot(st, vec2<f32>(269.5, 183.3))
    );
    return -1.0 + 2.0 * fract(sin(st_transformed) * 43758.5453123);
}

fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn noise(st: vec2<f32>) -> f32 {
    let i: vec2<f32> = floor(st);
    let f: vec2<f32> = fract(st);
    let u: vec2<f32> = f * f * ((3.0 - 2.0) * f);
    return mix(mix(dot(random(i + vec2<f32>(0.0, 0.0)), f - vec2<f32>(0.0, 0.0)),
                   dot(random(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0)), u.x),
               mix(dot(random(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0)),
                   dot(random(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0)), u.x), u.y);
}
fn rotate2D(r: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(r), -sin(r),
                       sin(r), cos(r));
}
fn sinh(x: f32) -> f32 {
    return (exp(x) - exp(-x)) / 2.0;
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let iResolution: vec2<f32> = vec2<f32>(640.0, 480.0);
    let uv: vec2<f32> = 3.0*(FragCoord.xy - 0.5 * iResolution) / min(iResolution.x, iResolution.y);

    var col: vec3<f32> = vec3<f32>(0.0);
    let t: f32 = u_time.time;
   let oscillationFactors: vec3<f32> = vec3<f32>(oscillate(0.5, 0.51, 15.0, t), oscillate(2.0, 2.51, 15.0, t), oscillate(0.5, 0.51, 15.0, t));

    var N: vec2<f32> = vec2<f32>(0.0);
    var p: vec2<f32> = uv + t / 20.0;
    var S: f32 = oscillationFactors.y;
    let m: mat2x2<f32> = rotate2D(oscillationFactors.x);
    let branchFactor: f32 = 1.78;

var n: vec2<f32> = vec2<f32>(0.0, 0.0); 

for (var j: i32 = 0; j < 45; j = j + 1) {
    p *= m;
    n *= m; 

    let q: vec2<f32> = p * S * f32(j) + n + t;
    n += branchFactor * cos(q);
    N += branchFactor * cos(q) / S * oscillationFactors.z;

    S *= 1.4 * sinh(0.9);
}

    let colorOffset: vec3<f32> = vec3<f32>(
        0.1 * smoothstep(0.4, 1.0, sin(N.x)),
        0.5 * smoothstep(1.0, 1.0, sin(N.x)),
        0.1 * smoothstep(0.5, 1.0, cos(N.x))
    );

    let flowColorChange: vec3<f32> = vec3<f32>(
        1.5 * cos(1.0 * t + N.x),
        0.5 * sin(1.0 * t + N.y),
        1.5 * cos(1.0 * t + N.y)
    );

    let flowIntensity: vec3<f32> = vec3<f32>(
        0.1 / length(1.03 * N),
        smoothstep(5.5, 25.0, N.x),
        smoothstep(5.5, 1.0, N.y)
    );
    col = (vec3<f32>(0.5, 0.0, 2.1) * colorOffset + flowColorChange + 4.5 * flowIntensity) *
          ((0.5 * N.x * 0.5 * N.y) + .0015 / length(1.0 * N));
    return vec4<f32>(col, 1.0);
}