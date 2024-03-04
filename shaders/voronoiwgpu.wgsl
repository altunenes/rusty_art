@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
const PI: f32 = 3.14;

fn hash3(p: vec3<f32>) -> vec3<f32> {
    let q: vec3<f32> = vec3<f32>(
        dot(p, vec3<f32>(127.1, 311.7, 189.2)),
        dot(p, vec3<f32>(269.5, 183.3, 324.7)),
        dot(p, vec3<f32>(419.2, 371.9, 128.5))
    );
    return fract(sin(q) * 43758.5453);
}

fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn noise(x: vec3<f32>, v: f32) -> f32 {
    let p: vec3<f32> = floor(x);
    let f: vec3<f32> = fract(x);
    let s: f32 = 1.0 + 444.0 * v;
    var va: f32 = 0.0;
    var wt: f32 = 0.0;
    var k: i32 = -2;
    while (k <= 1) {
        var j: i32 = -2;
        while (j <= 1) {
            var i: i32 = -2;
            while (i <= 1) {
                let g: vec3<f32> = vec3<f32>(f32(i), f32(j), f32(k));
                let o: vec3<f32> = hash3(p + g);
                let r: vec3<f32> = g - f + o + 0.5;
                let d: f32 = dot(r, r);
                let w: f32 = pow(1.0 - smoothstep(0.0, 1.414, sqrt(d)), s);
                va += o.z * w;
                wt += w;
                i += 1;
            }
            j += 1;
        }
        k += 1;
    }
    return va / wt;
}

fn fBm(p: vec3<f32>, v: f32) -> f32 {
    var sum: f32 = 0.0;
    let scramb: f32 = osc(0.0, 24.0, 20.0, u_time.time); 
    var amp: f32 = 1.0;
    var mutable_p = p; // Copy the immutable parameter to a mutable variable
    var i: i32 = 0;
    while (i < 4) {
        sum += amp * noise(mutable_p, v);
        amp *= 0.5;
        mutable_p *= 2.0; // Modify the mutable copy
        i += 1;
    }
    return sum;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let uv: vec2<f32> = FragCoord.xy / vec2<f32>(800.0, 600.0); // Assuming a resolution, replace with actual uniform if available
    let p: vec2<f32> = uv * 2.0 - 1.0;
    let rd: vec3<f32> = normalize(vec3<f32>(p.x, p.y, 1.0));
    let pos: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0) * u_time.time + rd * 10.0;

    // Use fBm for UV distortion
    let distortion: f32 = fBm(pos, 0.1) * 0.1;
    let distortedUV: vec2<f32> = uv + vec2<f32>(distortion, distortion);

    // Sample the texture with distorted UV coordinates
    let texColor: vec4<f32> = textureSample(tex, tex_sampler, distortedUV);

    // Output the color
    return vec4<f32>(texColor.rgb, 1.0);
}
