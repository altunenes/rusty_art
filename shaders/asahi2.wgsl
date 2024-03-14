//see my shadertoy code: https://www.shadertoy.com/view/MX23Wz;
//note, I used petal polar GLSL function from the user BenoitArbelot, (2020): https://www.shadertoy.com/view/ttySz3 
// Global constants
const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn rot(a: f32) -> mat2x2<f32> {
    return mat2x2<f32>(
        cos(a), -sin(a),
        sin(a), cos(a)
    );
}

fn DrawPetalPolar(uv: vec2<f32>, pos: vec2<f32>, size: f32, dir: vec2<f32>, colorDirection: f32) -> vec4<f32> {
    var dist: vec2<f32> = uv - pos;

    let angle: f32 = -atan2(dir.y, dir.x);
    dist = dist * (rot(angle) * 0.8);

    dist.x = dist.x - (size * 0.25);

    let r: f32 = length(dist) * 1.5;
    let a: f32 = atan2(dist.y, dist.x);

    var f: f32 = -1.0;
    if (a > PI * 0.5 || a < -PI * 0.5) {
        f = size * cos(a * 2.0);
    }

    let petalMask: f32 = smoothstep(0.0, -1.0, (r - f) / fwidth(r - f));

    var color: vec3<f32>;
    if (colorDirection > 0.0) {
        color = mix(vec3<f32>(1.0, 1.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), r / (size * 1.0));
    } else {
        color = mix(vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(1.0, 1.0, 0.0), r / (size * 1.0));
    }

    return vec4<f32>(color, petalMask);
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0); 
    var uv: vec2<f32> = 1.3 * (FragCoord.xy - 0.5 * resolution) / resolution.y;
    let bgColor: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
    var fragColor: vec4<f32> = vec4<f32>(bgColor, 1.0);

    let petalSize: f32 = 0.40;
    let space: f32 = (2.5 * PI / 15.0) * (1.0 + 0.2);
    let phase: f32 = sin(u_time.time / 2.0) * PI;

    let left: vec2<f32> = vec2<f32>(-0.5, 0.0);
    let right: vec2<f32> = vec2<f32>(0.5, 0.0);

    // Left side
    for (var i: f32 = 0.0; i < 2.0 * PI; i = i + space) {
        let pos: vec2<f32> = left + vec2<f32>(cos(i), sin(i)) * 0.25;
        let angle: f32 = i + phase;
        let dir: vec2<f32> = vec2<f32>(cos(angle), sin(angle));

        let leftcolor: vec4<f32> = DrawPetalPolar(uv, pos, petalSize, dir, 1.0);
        fragColor = mix(fragColor, leftcolor, leftcolor.a);
    }

    // Right side
    for (var i: f32 = 0.0; i < 2.0 * PI; i = i + space) {
        let pos: vec2<f32> = right + vec2<f32>(cos(i), sin(i)) * 0.25;
        let angle: f32 = i - phase;
        let dir: vec2<f32> = vec2<f32>(cos(angle), sin(angle));

        let colors: vec4<f32> = DrawPetalPolar(uv, pos, petalSize, dir, -1.0);
        fragColor = mix(fragColor, colors, colors.a);
    }

    return fragColor;
}
