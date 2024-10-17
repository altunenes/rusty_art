const MAX_STEPS: i32 = 155;
const MAX_DIST: f32 = 155.0;
const SURF_DIST: f32 = 0.001;
const PI: f32 = 3.14159265359;
const LIGHT_SPEED: f32 = 0.5;
const LIGHT_INTENSITY: f32 = 2.0;
const LIGHT_RADIUS: f32 = 144.0;
const BAILOUT: f32 = 4.0;
const AMBIENT: f32 = 0.1;
const SPECULAR_COEFF: f32 = 0.5;
const SHININESS: f32 = 0.5;
struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
struct Params {
    lambda: f32,
    theta: f32,
    alpha: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
    aa: f32,
    iter: f32,
    bound: f32,
    tt: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
};

@group(0) @binding(1)
var<uniform> params: Params;
fn oscWithPause(minValue: f32, maxValue: f32, interval: f32, pauseDuration: f32, currentTime: f32) -> f32 {
    let cycleTime: f32 = interval * 2.0 + pauseDuration;
    let phase: f32 = currentTime % cycleTime;
    if (phase < interval) {
        return mix(maxValue, minValue, phase / interval);
    } else if (phase < interval + pauseDuration) {
        return minValue;
    } else {
        return mix(minValue, maxValue, (phase - interval - pauseDuration) / interval);
    }
}
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h: f32 = clamp(0.1 + 0.1 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}
fn fold(p: vec2<f32>) -> vec2<f32> {
    let r: f32 = 0.5;
    return vec2<f32>(abs(p.x + r) - abs(p.x - r) - p.x, p.y);
}
fn sdKleinian(p: vec3<f32>, time: f32) -> f32 {
    var scale: f32 = params.gamma;
    let offset: f32 = params.blue;
    let scramb: f32 = oscWithPause(params.lambda, params.theta, params.alpha, params.sigma, time);

    var pp: vec3<f32> = p;

    for (var i: i32 = 0; i < 12; i = i + 1) {
        pp = -1.0 + 2.0 * fract(0.5 * pp + 0.5);
        
        let r2: f32 = dot(pp, pp);
        let k: f32 = max(scramb / r2, 0.1);
        pp *= k;
        scale *= k;
    }
    let ap: vec3<f32> = abs(pp);
    var d: f32 = (ap.x - offset) / scale;
    d = smin(d, (ap.y - offset) / scale, 0.01);
    d = smin(d, (ap.z - offset) / scale, 0.01);
    
    return d;
}

fn getNormal(p: vec3<f32>, time: f32) -> vec3<f32> {
    let eps: f32 = oscWithPause(params.tt, params.tt, 5.0, 0.0, time);
    let e: vec3<f32> = vec3<f32>(eps, eps, eps); 

    let d: f32 = sdKleinian(p, time);
    let n: vec3<f32> = vec3<f32>(
        d - sdKleinian(p - vec3<f32>(e.x, 0.0, 0.0), time),
        d - sdKleinian(p - vec3<f32>(0.0, e.y, 0.0), time),
        d - sdKleinian(p - vec3<f32>(0.0, 0.0, e.z), time)
    );
    return normalize(n);
}
fn rayMarchFractal(ro: vec3<f32>, rd: vec3<f32>, time: f32) -> f32 {
    var dO: f32 = 0.0;
    
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        let p: vec3<f32> = ro + rd * dO;
        let dS: f32 = sdKleinian(p, time);
        dO += dS;
        if (dO > MAX_DIST || abs(dS) < SURF_DIST) {
            break;
        }
    }
    return dO;
}
fn softShadow(ro: vec3<f32>, rd: vec3<f32>, k: f32, time: f32) -> f32 {
    var res: f32 = params.a;
    var ph: f32 = 1e20;
    var dO: f32 = params.tt;
    
    for (var i: i32 = 0; i < 12; i = i + 1) {
        let p: vec3<f32> = ro + rd * dO;
        let dS: f32 = sdKleinian(p, time);
        let y: f32 = dS * dS / (144.0 * ph);
        let d: f32 = sqrt(max(dS * dS - y * y, 0.0));
        res = min(res, k * d / max(0.0, dO - y));
        ph = dS;
        dO += dS;
        if (dO > MAX_DIST || res < params.b) {
            break;
        }
    }
    return clamp(res, 0.0, 1.0);
}
fn palette(d: f32) -> vec3<f32> {
    return mix(vec3<f32>(params.e, params.f, params.g), vec3<f32>(params.c, params.d, params.e), d);
}
fn rotate2D(p: vec2<f32>, a: f32) -> vec2<f32> {
    let c: f32 = cos(a);
    let s: f32 = sin(a);
    let rotation: mat2x2<f32> = mat2x2<f32>(c, s, -s, c);
    return rotation * p;
}
fn mapParticles(p: vec3<f32>, time: f32) -> f32 {
    var p_mut: vec3<f32> = p;
    for (var i: i32 = 0; i < 2; i = i + 1) {
        let t: f32 = time;
        let p_xz: vec2<f32> = rotate2D(vec2<f32>(p_mut.x, p_mut.z), t);
        p_mut = vec3<f32>(p_xz.x - 0.5, p_mut.y, p_xz.y - 0.5);
    }
    return dot(sign(p_mut), p_mut) / 1.5;
}

fn rayMarchParticles(ro: vec3<f32>, rd: vec3<f32>, time: f32) -> vec4<f32> {
    var t: f32 = 0.0;
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var d: f32;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        let p: vec3<f32> = ro + rd * t;
        d = mapParticles(p, time) * 0.5;
        if (d < 0.02) {
            break;
        }
        if (t > 1.0) {
            break;
        }
        col += palette(length(p) * 8.0) / (300.0 * d);
        t += d;
    }
    return vec4<f32>(col, 1.0 / d);
}

fn getLightPosition(time: f32) -> vec3<f32> {
    return vec3<f32>(
        sin(time * LIGHT_SPEED) * 6.0,
        cos(time * LIGHT_SPEED * 0.7) * 2.0,
        sin(time * LIGHT_SPEED * 1.3) * 3.0
    );
}

fn render(ro: vec3<f32>, rd: vec3<f32>, time: f32) -> vec3<f32> {
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let d: f32 = rayMarchFractal(ro, rd, time);
    
    if (d < MAX_DIST) {
        let p: vec3<f32> = ro + rd * d;
        let n: vec3<f32> = getNormal(p, time);
        let lightPos: vec3<f32> = getLightPosition(time);
        let l: vec3<f32> = normalize(lightPos - p);
        
        let diff: f32 = max(dot(n, l), 0.0);
        let spec: f32 = pow(max(dot(reflect(-l, n), -rd), 0.0), 123.0);
        let fresnel: f32 = pow(1.0 - max(dot(n, -rd), 0.0), 2.0);
        
        let shadow: f32 = softShadow(p, l, 1.0, time);
        
        let objCol: vec3<f32> = params.bound + 0.5 * cos(vec3<f32>(0.0, 0.0, 2.0) * PI * 1.0 + length(p) * 0.0 + time * 0.0);
        let lightDistance: f32 = length(lightPos - p);
        let lightIntensity: f32 = 1.0 / (1.0 + lightDistance * lightDistance * 0.1);
        let lightColor: vec3<f32> = vec3<f32>(1.0, 0.8, 0.6) * LIGHT_INTENSITY * lightIntensity;
        
        let fractalShading1: vec3<f32> = objCol * (diff * shadow + 0.01);
        let fractalShading2: vec3<f32> = lightColor * spec * shadow;
        let fractalShading3: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0) * fresnel;
        var fractalShading: vec3<f32> = fractalShading1 + fractalShading2 + fractalShading3;
        let particleCol: vec4<f32> = rayMarchParticles(p, l, time);
        fractalShading += particleCol.rgb * particleCol.a;
        let fogsss: f32 = oscWithPause(2.0, 0.0, 18.0, 6.0, time);
        let fogAmount: f32 = 1.0 - exp(-0.06 * d * d);
        let lightFog: f32 = exp(-lightDistance / LIGHT_RADIUS);
        let fogColor: vec3<f32> = mix(vec3<f32>(fogsss, fogsss, fogsss), lightColor, lightFog);
        fractalShading = mix(fractalShading, fogColor, fogAmount);
        
        col = fractalShading;
    }
    
    return col;
}
fn AA(uv: vec2<f32>, time: f32, ro: vec3<f32>) -> vec3<f32> {
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let AA_size: f32 = params.aa;
    var count: f32 = 0.0;
    
    for (var aaY: f32 = 0.0; aaY < 2.0; aaY = aaY + 1.0) {
        for (var aaX: f32 = 0.0; aaX < 2.0; aaX = aaX + 1.0) {
            let offset: vec2<f32> = vec2<f32>(aaX, aaY) / 2.0 - vec2<f32>(0.5, 0.5);
            let aauv: vec2<f32> = uv + offset * AA_size / 512.0; 
            let rd: vec3<f32> = normalize(vec3<f32>(aauv.x, aauv.y, 1.5));
            col += render(ro, rd, time);
            count += 1.0;
        }
    }
    
    return col / count;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1280.0, 720.0);
    let uv: vec2<f32> = params.iter * (FragCoord.xy - 0.5 * resolution) / resolution.y;
    let t: f32 = u_time.time * 0.1;
    let radius: f32 = 5.0; 
    let ro: vec3<f32> = vec3<f32>(
        radius * cos(t),
        2.0 * sin(t * 0.5),
        radius * sin(t)
    );
    let lookAt: vec3<f32> = vec3<f32>(0.0, 1.0, 0.0);
    let col: vec3<f32> = AA(uv, u_time.time, ro);
    let colGamma: vec3<f32> = pow(col, vec3<f32>(1.4545, 1.4545, 1.4545));
    return vec4<f32>(colGamma, 1.0);
}
