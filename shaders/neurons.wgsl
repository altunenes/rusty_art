const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
fn random(st: vec2<f32>) -> vec2<f32> {
    let st_new: vec2<f32> = vec2<f32>(
        dot(st, vec2<f32>(127.1, 311.7)),
        dot(st, vec2<f32>(269.5, 183.3))
    );
    return -1.0 + 2.0 * fract(sin(st_new) * 43758.5453123);
}
fn noise(st: vec2<f32>) -> f32 {
    let i: vec2<f32> = floor(st);
    let f: vec2<f32> = fract(st);
    let u: vec2<f32> = f * f * (25.0 - 22.0 * f);
    return mix(mix(dot(random(i + vec2<f32>(0.0, 0.0)), f - vec2<f32>(0.0, 0.0)),
                   dot(random(i + vec2<f32>(1.0, 0.0)), f - vec2<f32>(1.0, 0.0)), u.x),
               mix(dot(random(i + vec2<f32>(0.0, 1.0)), f - vec2<f32>(0.0, 1.0)),
                   dot(random(i + vec2<f32>(1.0, 1.0)), f - vec2<f32>(1.0, 1.0)), u.x), u.y);
}

fn fbm(p: vec2<f32>) -> f32 {
    var value: f32 = 0.0;
    var amplitude: f32 = 0.5;
    var p_var: vec2<f32> = p;
    for (var i: i32 = 0; i < 5; i++) {
        value += amplitude * noise(p_var);
        p_var *= 5.0;
        amplitude *= 0.5;
    }
    return value;
}
fn rotate2D(r: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(r), -sin(r), sin(r), cos(r));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution.xy) - vec2<f32>(0.5, 0.5 * resolution.y / resolution.x);
    uv *= 1.5;

    let t: f32 = u_time.time / 4.0;
       let xVal4: f32 = oscillate(0.5, 0.51, 15.0, t);
    let xVal6: f32 = oscillate(1.5, 1.51, 45.0, t);
    let xVal7: f32 = oscillate(0.5, 2.51, 15.0, t);

    var n: vec2<f32> = vec2<f32>(0.0, 0.0);
    var N: vec2<f32> = vec2<f32>(0.0, 0.0) * 5.0;
    var p: vec2<f32> = uv + t;
    var S: f32 = xVal6;
    let m: mat2x2<f32> = rotate2D(xVal4);
    var branchFactor: f32 = 1.78;

    for (var j: f32 = 0.0; j < 45.0; j += 1.0) {
        p = m * p;
        n = m * n;
        let q: vec2<f32> = m * p * S * j + n + vec2<f32>(t, t);
        
        n += branchFactor * sin(q);
        N += branchFactor * cos(q) / S * xVal7;
        branchFactor *= 1.3 * atan(0.975);
        S *= 1.45 * tanh(25.975);
    }

    let pulse: f32 = sin(4.0 * t + length(p) + fbm(vec2<f32>(t * 34.1, length(p) * 2.1))) * 0.1 + 0.5;
    let colorOffset: vec3<f32> = vec3<f32>(
        2.1 * smoothstep(2.4, 2.8, sin(n.x)),
        0.5 * smoothstep(3.0, 3.8, sin(n.y)),
        0.1 * smoothstep(0.5, 0.8, cos(n.x))
    );
    let flowColorChange: vec3<f32> = vec3<f32>(
        1.8 * cos(3.0 * t + N.x),
        1.0 * sin(3.0 * t + N.y),
        1.5 * cos(3.0 * t + N.y)
    );
    let flowIntensity: vec3<f32> = vec3<f32>(
        0.0021 / length(0.01 * N),
        smoothstep(2.5, 0.0, N.x),
        smoothstep(0.5, 1.0, N.y)
    );
    let xValdark: f32 = oscillate(3.0, 10.51, 15.0, t);

    var col: vec3<f32> = (vec3<f32>(5.5 * pulse, 2.0 * pulse, 3.1 * pulse) *
        colorOffset + flowColorChange + flowIntensity) *
        ((0.00001 * N.x * 0.001 * N.y / 0.0015*N.y) + 0.3225 / length(xValdark * N));
    let axonPulse: f32 = sin(t + length(p) * 51.0) * 1.5 + 24.5;
    let axonColor: vec3<f32> = vec3<f32>(1.4, 2.1, 0.0);
    let axonEffect: vec3<f32> = smoothstep(1.1, 2.55, axonPulse) * axonColor;

    return vec4<f32>(col, 1.0);
}