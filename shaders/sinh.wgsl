const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

const MI: i32 = 66;
const B: f32 = 67.0;

fn c_mul(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn c_sinh(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(sinh(z.x) * cos(z.y), cosh(z.x) * sin(z.y));
}

fn c_abs(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(abs(sin(z.x)), abs(sin(z.y)));
}

fn c_sinh_pow4(z: vec2<f32>) -> vec2<f32> {
    let sinh_z = c_sinh(z);
    return c_mul(c_mul(sinh_z, sinh_z), c_mul(sinh_z, sinh_z));
}
fn implicit(z: vec2<f32>, c: vec2<f32>, time: f32) -> vec2<f32> {
    var z_local = z; 
    var i: i32 = 0;
    loop {
        if (i >= MI) { break; } // Loop exit condition
        z_local = c_abs(c_sinh_pow4(z_local)) + c;
        z_local = z_local + 0.03 * vec2<f32>(cos(1.05 * time / 4.0), cos(1.05 * time / 4.0));
        if (dot(z_local, z_local) > B * B) {
            break;
        }
        i = i + 1;
    }
    return vec2<f32>(f32(i), dot(z_local, z_local));
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let AA: i32 = 1;
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0); 
    let time: f32 = u_time.time; 

    for (var m: i32 = 0; m < AA; m = m + 1) {
        for (var n: i32 = 0; n < AA; n = n + 1) {
            let uv: vec2<f32> = ((FragCoord.xy - 0.5 * resolution) / min(resolution.y, resolution.x) * 2.0) * 0.5;
            let c_value: f32 = mix(2.2, 2.994254, 0.01 + 0.01 * sin(0.05 * time / 12.0));
            let O: f32 = 0.00000884271 + 0.2021010101 * (sin(0.001 * time / 12.0) + 0.1); 
            let exteriorColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0); 
            let c: vec2<f32> = vec2<f32>(O, c_value);
            let z_and_i = implicit(uv, c, time);
            let ir: f32 = z_and_i.x / f32(MI);
            let lenSq: f32 = ir * z_and_i.y;
            if (ir >= 1.0) {
                let c1: f32 = pow(clamp(2.00 * sqrt(lenSq), 0.0, 1.0), 0.5);
                let col1: vec3<f32> = 0.5 + 0.5 * sin(4.0 + vec3<f32>(0.0, 0.5, 1.0) + PI * vec3<f32>(2.0 * lenSq) + time / 12.0);
                let col2: vec3<f32> = 0.5 + 0.5 * sin(2.1 + PI * vec3<f32>(lenSq) + time / 12.0);
                col = col + 1.5 * sqrt(c1 * col1 * col2);
            } else {
                col = col + exteriorColor;
            }
        }
    }
    col = col / f32(AA * AA);

    return vec4<f32>(col, 1.0);
}
