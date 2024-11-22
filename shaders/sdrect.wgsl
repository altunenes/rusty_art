const PI: f32 = 3.14159265358979323846;
const LIGHT_INTENSITY: f32 = 1.2;
const RIM_POWER: f32 = 1.0;  
const AO_STRENGTH: f32 = 0.05;
const ENV_LIGHT_STRENGTH: f32 = 0.4;
const IRIDESCENCE_POWER: f32 = 0.15;
const FALLOFF_DISTANCE: f32 = 2.5;   
const VIGNETTE_STRENGTH: f32 = 0.25;  

struct TimeUniform {
    time: f32,
};
struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};

@group(0) @binding(1)
var<uniform> params: Params;
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

fn rot(a: f32) -> mat2x2<f32> {
    let c = cos(a);
    let s = sin(a);
    return mat2x2<f32>(c, s, -s, c);
}

fn osc(mn: f32, mx: f32, iv: f32, n: f32) -> f32 {
    return mn + (mx - mn) * 0.5 * (sin(2.0 * PI * n / iv) + 1.0);
}

fn sdRect(p: vec2<f32>, r: f32, a: f32) -> f32 {
    let w = r * 0.8;
    let h = r * 0.4;
    var rp = p * rot(-a);
    let d = abs(rp) - vec2<f32>(w, h);
    let dst = min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0)));
    return max(dst, 0.1);
}

fn getVerts(uv: vec2<f32>, s: f32, a: f32) -> array<vec2<f32>, 4> {
    var v: array<vec2<f32>, 4>;
    let w = s * 0.8;
    let h = s * 0.4;
    v[0] = vec2<f32>(-w, h);
    v[1] = vec2<f32>(w, h);
    v[2] = vec2<f32>(w, -h);
    v[3] = vec2<f32>(-w, -h);
    let r = rot(a);
    for(var i = 0; i < 4; i++) {
        v[i] = r * v[i] + uv;
    }
    return v;
}

fn getLI(uv: vec2<f32>, rect: f32, l: f32, t: f32, a: f32) -> f32 {
    let v = getVerts(uv, l, a);
    let ps1 = sin(l * 13.37 + t * 0.3);
    let ps2 = cos(l * 7.54 - t * 0.4);
    let ps3 = sin(l * 9.21 + t * 0.5);
    let ps4 = cos(l * 11.13 + t * 0.6);
    
    let lp1 = v[0] + vec2<f32>(cos(t * 0.5 + ps1), sin(t * 0.7 + ps2)) * 0.3;
    let lp2 = v[1] + vec2<f32>(sin(t * 0.3 + ps2), cos(t * 0.4 + ps3)) * 0.3;
    let lp3 = v[2] + vec2<f32>(cos(t * 0.6 + ps3), sin(t * 0.5 + ps4)) * 0.3;
    let lp4 = v[3] + vec2<f32>(sin(t * 0.4 + ps4), cos(t * 0.6 + ps1)) * 0.3;
    
    let d1 = length(uv - lp1) * (1.0 + 0.2 * sin(l * 15.0 + t));
    let d2 = length(uv - lp2) * (1.0 + 0.2 * cos(l * 12.0 - t));
    let d3 = length(uv - lp3) * (1.0 + 0.2 * sin(l * 9.0 + t * 0.7));
    let d4 = length(uv - lp4) * (1.0 + 0.2 * cos(l * 11.0 - t * 0.8));
    
    let f1 = 2.0 / (4.0 + d1 * FALLOFF_DISTANCE);
    let f2 = 2.0 / (4.0 + d2 * FALLOFF_DISTANCE);
    let f3 = 2.0 / (4.0 + d3 * FALLOFF_DISTANCE);
    let f4 = 2.0 / (4.0 + d4 * FALLOFF_DISTANCE);
    
    var w1 = 0.25 + 0.1 * sin(l * 11.0 + t);
    var w2 = 0.25 + 0.1 * cos(l * 13.0 - t);
    var w3 = 0.25 + 0.1 * sin(l * 17.0 + t * 0.5);
    var w4 = 0.25 + 0.1 * cos(l * 15.0 - t * 0.7);
    
    let tw = w1 + w2 + w3 + w4;
    w1 /= tw;
    w2 /= tw;
    w3 /= tw;
    w4 /= tw;
    
    let ao = 1.4 - (l * AO_STRENGTH) * (1.0 + 0.2 * sin(l * 20.0 + t));
    let n = normalize(vec2<f32>(cos(a + l), sin(a + l)));
    var rm = 1.1 - abs(dot(normalize(uv), n));
    rm = pow(rm, RIM_POWER);
    
    let vl = f1 * w1 + f2 * w2 + f3 * w3 + f4 * w4;
    let sh = sin(l * 10.0 + t) * cos(l * 7.0 - t);
    
    return vl * (1.0 + 0.15 * sh) * ao * LIGHT_INTENSITY + rm * 0.4;
}

fn getEL(uv: vec2<f32>, a: f32, l: f32, t: f32) -> f32 {
    let ld = normalize(vec2<f32>(cos(t), sin(t)));
    let n = normalize(vec2<f32>(cos(a), sin(a)));
    var el = dot(n, ld);
    el = el * 0.5 + 0.5;
    let dp = 1.0 - (l / 1.5);
    let le = sin(l * 4.0 + t) * 0.5 + 0.5;
    return mix(el, le, 0.5) * dp * ENV_LIGHT_STRENGTH;
}

@fragment
fn main(@builtin(position) fc: vec4<f32>) -> @location(0) vec4<f32> {
    var col = vec4<f32>(0.5);
    var h: vec4<f32>;
    let ss = 1.3 * vec2<f32>(1920.0, 1080.0);
    let t = u_time.time * 0.5;
    var ang = 0.25;
    let fp = cos(t * 0.5) * PI * 0.25;
    let gl = osc(0.4, 1.5, 8.0, u_time.time);
    let asd = osc(0.3, 0.07, 25.0, u_time.time);

    var i: f32 = 1.5;
    while(i > 0.003) {
        let l = i * 1.0;
        let fd = sin(t + l * 0.2) * cos(t * 0.5 + l * 0.1);
        let w = cos(t * 0.7 + l * 0.15) * sin(t * 0.3 + i);
        let ta = fp;
        ang -= sin(ang - sin(ta)) * (0.5 + 0.5 * sin(l));
        let af = sign(sin(l * 0.5)) * sin(t + i * 2.0);
        var uv = 2.7 * (fc.xy + fc.xy - ss) / ss.y;
        uv.y += 0.5;
        uv.x += 1.0;
        uv *= rot(i + (ang + af) + fp);
        let rect = sdRect(uv, i, ang + t * (1.0 + 0.2 * sin(l)));
        let a = smoothstep(0.0, 0.2, (rect - 0.1) * ss.y * 0.15);
        let li = getLI(uv, rect, l, t, ang);
        let el = getEL(uv, ang, l, t);
        let d1 = osc(0.5, 1.0, 5.0, u_time.time);
        let d2 = osc(0.1, 1.0, 5.0, u_time.time);
        let ci = 0.8 + 0.2 * sin(l * 24.37 + t) * cos(l * 12.54 - t * 0.4) * sin(ang * 3.0 + t * 0.7);
        let cs = 0.2 + 0.1 * cos(l * 9.21 + t * 0.5) * sin(ang * 5.0 - t * 0.3);
        h = sin(i / d2 + ang / d1 + vec4<f32>(params.blue, 2.0, 3.0, 1.0) + fd * 0.5) * cs + ci;
        let lc = h * (li * gl + el);
        let df = params.lambda- (i - 0.003) / (1.5 - 0.003);
        let ir = sin(dot(uv, uv) * 4.0 + t) * IRIDESCENCE_POWER * df + 0.95;
        var cwi = lc * vec4<f32>(ir, ir * 0.98, ir * 1.02, 1.0);
        let d4 = osc(0.15, 0.2, 10.0, u_time.time);
        let mf = d4 / (rect + params.sigma) * (1.0 - df * 0.25);
        col = mix(cwi, col, a) * mix(vec4<f32>(1.0), h + params.theta * a * 0.1*(uv.x / rect + li), mf);
        i -= asd;
    }
    
    col = vec4<f32>(col.rgb * (params.gamma + gl * 0.4), 1.0);
    let vuv = (fc.xy - 0.5 * vec2<f32>(1920.0, 1080.0)) / 1080.0;
    let v = 1.0 - dot(vuv, vuv) * VIGNETTE_STRENGTH;
    return vec4<f32>(col.r * v, col.g * v, col.b * v, col.a);
}