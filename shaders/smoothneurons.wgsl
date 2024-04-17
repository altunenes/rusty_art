const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
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
    let u: vec2<f32> = f * f * (3.0 - 2.0 * f);
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
        p_var *= 2.0;
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
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let t: f32 = u_time.time;
    let xVal4: f32 = oscillate(0.5, 0.51, 15.0, t);

    var n: vec2<f32> = vec2<f32>(0.0, 0.0);
    var q: vec2<f32>;
    var N: vec2<f32> = vec2<f32>(0.0, 0.0);
    var p: vec2<f32> = uv + t / 20.0;
    var S: f32 = 15.0;
    let m: mat2x2<f32> = rotate2D(xVal4);
    var branchFactor: f32 = 1.85;

    for(var j: i32 = 0; j < 40; j = j + 1){
        p *= m;
        n *= m;
        q = p * S + f32(j) + n + t;

        n += branchFactor * sin(q);
        N += branchFactor * cos(q) / S * 2.5;
        branchFactor *= 1.3 * atan(0.975);

        S *= params.theta * tanh(3.45);
    }

    let pulse: f32 = sin(params.blue * t + length(p) + fbm(vec2<f32>(t * 0.1, length(p) * 2.1))) * 0.1 + params.blue;

    let colorOffset: vec3<f32> = vec3<f32>(
        params.lambda * smoothstep(0.4, 5.0, sin(n.x)),
        params.alpha * smoothstep(3.0, 5.0, sin(n.y)),
       params.sigma * smoothstep(0.0, 1.0, cos(n.x))
    );

    let flowColorChange: vec3<f32> = vec3<f32>(
        1.5 * cos(3.0 * t + N.x),
        0.5 * sin(3.0 * t + N.y),
        1.5 * cos(3.0 * t + N.y)
    );

    let flowIntensity: vec3<f32> = vec3<f32>(
        0.001 / length(0.3 * N),
        smoothstep(0.1, 1.0, N.x),
        smoothstep(1.5, 1.0, N.y)
    );

    col = (vec3<f32>(1.5 * pulse, 1.0 * pulse, 3.1 * pulse) * colorOffset + flowColorChange + flowIntensity) * ((1.0 * N.x + 1.0 * N.y + 0.001) + 0.0015 / length(1.0 * N));

    let axonPulse: f32 = sin(t + length(p) * 10.0) * 0.5 + 0.5; 
    let axonColor: vec3<f32> = vec3<f32>(1.2, 0.7, 1.0); 

    let axonEffect: vec3<f32> = smoothstep(0.1, 2.55, axonPulse) * axonColor;

    col += axonEffect; 

    return vec4<f32>(col,1.0);
}