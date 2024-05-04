@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
const PI: f32 = 3.14;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(2) @binding(2)
var<uniform> params: Params;
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
    let s: f32 = 1.0 + params.lambda * v;
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
    let scramb: f32 = osc(0.0, params.blue, 20.0, u_time.time); 
    var amp: f32 = scramb;
    var mutable_p = p; 
    var i: i32 = 0;
    while (i < 4) {
        sum += amp * noise(mutable_p, v);
        amp *= 0.3;
        mutable_p *= 2.0; 
        i += 1;
    }
    return sum;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); // Use the actual resolution (with image I recommend)
    let uv: vec2<f32> = FragCoord.xy / resolution;
    let p: vec2<f32> = uv; 
    let rd: vec3<f32> = normalize(vec3<f32>(p.x, p.y, 1.0));
    let pos: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0) * u_time.time + rd * params.gamma;

    let center: vec2<f32> = vec2<f32>(params.theta, params.theta);
    let toCenter: vec2<f32> = center - uv;
    let distanceFromCenter: f32 = length(toCenter);
    let adjustedDistance: f32 = distanceFromCenter * params.alpha - params.alpha;

    let distortionStrength: f32 = fBm(pos, params.sigma) * params.sigma;
    let distortionDirection: vec2<f32> = normalize(toCenter) * adjustedDistance;
    let distortedUV: vec2<f32> = uv + distortionDirection * distortionStrength;

    let texColor: vec4<f32> = textureSample(tex, tex_sampler, distortedUV);
    return vec4<f32>(texColor.rgb, 1.0);
}