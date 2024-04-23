//        Citations //
// 3D Noise function translated from "https://github.com/stegu/webgl-noise", Stefan Gustavson.

const PI: f32 = 3.141592653589793;

struct TimeUniform {
    time: f32,
};

@group(1) @binding(0) var<uniform> u_time: TimeUniform;

fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
struct Params {
    lambda: f32,
    theta: f32,
    alpha: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
    noise:f32,
    noise2:f32,
    color:f32,
    fw:f32,
    fh:f32,
    fcx:f32,
    fcy:f32,
};

@group(0) @binding(1) var<uniform> params: Params;

fn modf(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
}

fn permute(x: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(
        modf((x.x * 34.0 + 1.0) * x.x, 289.0),
        modf((x.y * 34.0 + 1.0) * x.y, 289.0),
        modf((x.z * 34.0 + 1.0) * x.z, 289.0),
        modf((x.w * 34.0 + 1.0) * x.w, 289.0)
    );
}

fn taylorInvSqrt(r: vec4<f32>) -> vec4<f32> {
    return 1.79284291400159 - 0.85373472095314 * r;
}

fn snoise(v: vec3<f32>) -> f32 {
    let C: vec2<f32> = vec2<f32>(1.0/6.0, 1.0/3.0);
    let D: vec4<f32> = vec4<f32>(0.0, 0.5, 1.0, 2.0);
    var i: vec3<f32> = floor(v + dot(v, vec3<f32>(C.y)));
    let x0: vec3<f32> = v - i + dot(i, vec3<f32>(C.x));
    let g: vec3<f32> = step(x0.yzx, x0.xyz);
    let l: vec3<f32> = 1.0 - g;
    let i1: vec3<f32> = min(g, l.zxy);
    let i2: vec3<f32> = max(g, l.zxy);
    let x1: vec3<f32> = x0 - i1 + C.x;
    let x2: vec3<f32> = x0 - i2 + 2.0 * C.x;
    let x3: vec3<f32> = x0 - 1.0 + 3.0 * C.x;
    i = vec3<f32>(
        modf(i.x, 289.0),
        modf(i.y, 289.0),
        modf(i.z, 289.0)
    );
    let p: vec4<f32> = permute(permute(permute(i.z + vec4<f32>(0.0, i1.z, i2.z, 1.0))
                    + i.y + vec4<f32>(0.0, i1.y, i2.y, 1.0))
                    + i.x + vec4<f32>(0.0, i1.x, i2.x, 1.0));
    let n_: f32 =1.0 / 7.0;
    let ns: vec3<f32> = n_ * D.wyz - D.xzx;
    let j: vec4<f32> = p - 49.0* floor(p * ns.z * ns.z);
    let x_: vec4<f32> = floor(j * ns.z);
    let y_: vec4<f32> = floor(j - 7.0 * x_);
    let x: vec4<f32> = x_ * ns.x + vec4<f32>(ns.y);
    let y: vec4<f32> = y_ * ns.x + vec4<f32>(ns.y);
    let h: vec4<f32> = 1.0 - abs(x) - abs(y);
    let b0: vec4<f32> = vec4<f32>(x.xy, y.xy);
    let b1: vec4<f32> = vec4<f32>(x.zw, y.zw);
    let s0: vec4<f32> = floor(b0) * 2.0 + 1.0;
    let s1: vec4<f32> = floor(b1) * 2.0 + 1.0;
    let sh: vec4<f32> = -step(h, vec4<f32>(0.0));
    let a0: vec4<f32> = b0.xzyw + s0.xzyw * sh.xxyy;
    let a1: vec4<f32> = b1.xzyw + s1.xzyw * sh.zzww;
    var p0: vec3<f32> = vec3<f32>(a0.xy, h.x);
    var p1: vec3<f32> = vec3<f32>(a0.zw, h.y);
    var p2: vec3<f32> = vec3<f32>(a1.xy, h.z);
    var p3: vec3<f32> = vec3<f32>(a1.zw, h.w);
    let norm: vec4<f32> = taylorInvSqrt(vec4<f32>(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;
    var m: vec4<f32> = max(0.6 - vec4<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), vec4<f32>(0.0));
    m = m * m;
    return 42.0 * dot(m * m, vec4<f32>(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);

    var uv = 2.0 * FragCoord.xy / resolution.xy - vec2<f32>(1.0, 1.0);
    uv.x *= resolution.x / resolution.y;
    uv.y += params.theta;
    uv.x = abs(uv.x);
    var largeScaleNoise: f32 = 0.0;
    var smallScaleNoise: f32 = 0.0;
    var freq: f32 = params.lambda;
    var amp: f32 =0.5;
    for (var i: i32 = 0; i < 5; i++) {
        largeScaleNoise += amp * snoise(vec3<f32>(uv * freq, u_time.time * params.blue));
        smallScaleNoise += amp * snoise(vec3<f32>(uv * freq * 10.0, u_time.time * params.blue));
        freq *= params.sigma;
        amp *= params.alpha;
    }
    largeScaleNoise = smoothstep(params.noise2, params.gamma, largeScaleNoise);
    smallScaleNoise = smoothstep(0.4, 0.6, smallScaleNoise);
    let inkBaseColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.1);
    let inkHighlights: vec3<f32> = vec3<f32>(params.color, 0.0, 0.0); 
    let highlightMix: f32 = mix(smallScaleNoise, largeScaleNoise, 0.5); 
    let inkColor: vec3<f32> = mix(inkBaseColor, inkHighlights, highlightMix);
    let focusWidth: f32 = params.fw;
    let focusHeight: f32 = params.fh;
    let focusCenterX: f32 = params.fcx;
    let focusCenterY: f32 = params.fcy;
    let vignetteX: f32 = smoothstep(focusCenterX - focusWidth / 2.0, focusCenterX - focusWidth / 2.0 + 0.1, abs(uv.x))
                      * (1.0 - smoothstep(focusCenterX + focusWidth / 2.0 - 0.1, focusCenterX + focusWidth / 2.0, abs(uv.x)));
    let vignetteY: f32 = smoothstep(focusCenterY - focusHeight / 2.0, focusCenterY - focusHeight / 2.0 + 0.1, abs(uv.y))
                      * (1.0 - smoothstep(focusCenterY + focusHeight / 2.0 - 0.1, focusCenterY + focusHeight / 2.0, abs(uv.y)));
    let vignette: f32 = vignetteX * vignetteY;
    let paperColor: vec3<f32> = vec3<f32>(0.36, 0.4, 0.4);
    let color: vec3<f32> = mix(paperColor, inkColor, step(0.01, largeScaleNoise) * vignette);
    var screenColor: vec3<f32> = 1.0 - (1.0 - color) * (1.0 - color);
    screenColor = applyGamma(screenColor, 0.5);

    return vec4<f32>(screenColor, 1.0);
}




