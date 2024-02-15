const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
fn fbm(p_initial: vec2<f32>) -> f32 {
    var value: f32 = 0.0;
    var amplitude: f32 = 0.5;
    var p: vec2<f32> = p_initial; // Use a mutable copy of the function parameter
    for (var i: i32 = 0; i < 5; i = i + 1) {
        value = value + amplitude;
        p = p * 2.0;
        amplitude = amplitude * 0.5;
    }
    return value;
}
fn rotate2D(r: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(r), -sin(r), sin(r), cos(r));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0);
    var uv: vec2<f32> = (FragCoord.xy / resolution) - vec2<f32>(0.5, 0.5 * resolution.y / resolution.x);
    uv *= 1.5;
    let t: f32 = u_time.time;
    let xVal4: f32 = oscillate(0.5, 0.5, 15.0, t);
    let xVal6: f32 = oscillate(4.0, 1.51, 15.0, t);
    let xVal7: f32 = oscillate(0.5, 1.51, 10.0, t);

    var p: vec2<f32> = uv;
    var n: vec2<f32> = vec2<f32>(0.0, 0.0);
    var N: vec2<f32> = vec2<f32>(1.5, 0.5);
    let m: mat2x2<f32> = rotate2D(xVal4);
    var S: f32 = xVal6;
    var branchFactor: f32 = 1.78;

    for (var j: f32 = 0.0; j < 45.0; j += 1.0) {
        p = p * m;
        n = n * m;
        let q: vec2<f32> = p * S * j + n + vec2<f32>(t, t); //static view: (change t values)

        n += branchFactor * sin(q);
        N += branchFactor * cos(q) / S * xVal7;
        S *= 1.245 * tanh(2.975);
    }
    let baseColor: vec3<f32> = vec3<f32>(0.1, 0.2, 0.5); 
    let colorVariation: vec3<f32> = vec3<f32>(
        0.3 * sin(1.0 + N.x),
        0.5 * sin(1.0 + N.y),
        0.8 * cos(1.0 + N.y)
    );
    var col: vec3<f32> = baseColor + colorVariation;
    let complementaryColor: vec3<f32> = vec3<f32>(0.1, 0.2, 0.1); 
    let complementaryVariation: f32 = 1.5 + 0.0 * sin(2.*PI * uv.x * uv.y + t); //0.0 * (the custom...)
    //let xVal1: f32 = oscillate(0.2, 0.001, 5.0, t);
    //remove if you feel awkard :-P
    let distanceFromCenter: f32 = length(uv - vec2<f32>(0.2, 0.2));
    let complementaryIntensity: f32 = smoothstep(0.1, 0.35, distanceFromCenter);
    col = mix(col, complementaryColor, complementaryVariation * complementaryIntensity);
    return vec4<f32>(col, 1.0);
}