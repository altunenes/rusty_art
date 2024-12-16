const PI: f32 = 3.14159265358979323846;
const FLT_MAX: f32 = 1.0;
fn fmod(x: f32, y: f32) -> f32 {
    return x - y * floor(x/y);
}
struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
    aa:f32,
    iter:f32,
    bound:f32,
    tt:f32,
    a:f32,
    b:f32,
    c:f32,
    d:f32,
    e:f32,
    f:f32,
    g:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;

fn hash(n: f32) -> f32 {
    return fract(sin(n) * 43758.5453123);
}

fn sdLine(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba)/dot(ba, ba), 0.0, 1.0);
    return length(pa - ba*h);
}

fn opModPolarMirrored(p: vec2<f32>, theta: f32, offset: f32) -> vec2<f32> {
    var a = atan2(p.y, p.x) - offset;
    a = abs(fmod(a + 0.5 * theta, theta) - 0.5 * theta);
    return length(p) * vec2<f32>(cos(a), sin(a));
}
//REF: https://www.shadertoy.com/view/wsGSD3
fn sdSnowflake(p: vec2<f32>) -> f32 {
    var modP = opModPolarMirrored(p, radians(params.gamma) / 6.0, radians(90.0));
    var d = sdLine(modP, vec2<f32>(0.0, 0.0), vec2<f32>(0.75, 0.0));
    d = min(d, sdLine(modP, vec2<f32>(0.5, 0.0), vec2<f32>(0.5, 0.0) + vec2<f32>(0.1, 0.1)));
    d = min(d, sdLine(modP, vec2<f32>(0.25, 0.0), vec2<f32>(0.25, 0.0) + 1.5 * vec2<f32>(0.1, 0.1)));
    return d - params.blue;
}

fn getSceneDist(p: vec2<f32>) -> f32 {
    let angle = u_time.time * 0.5;
    let rot = mat2x2<f32>(cos(angle), sin(angle), -sin(angle), cos(angle));
    let rotP = rot * p;
    return sdSnowflake(rotP);
}

fn getNorm(p: vec2<f32>) -> vec2<f32> {
    let normal1 = oscWithPause(params.b, params.c, params.d, 0.0, u_time.time);
    let eps = vec2<f32>(normal1, 0.0);
    
    return normalize(vec2<f32>(
        getSceneDist(p + eps.xy) - getSceneDist(p - eps.xy),
        getSceneDist(p + eps.yx) - getSceneDist(p - eps.yx)
    ));
}

fn calcLighting(ro: vec2<f32>, rd: vec2<f32>, t: f32) -> vec3<f32> {
    let p = ro + rd * t;
    let normal = getNorm(p);
    let c = osc(0.3, 0.7, 5.0, u_time.time);

    let angle = atan2(rd.y, rd.x);
    let baseColor = vec3<f32>(0.7, c, 1.0) + 0.2 * cos(angle + vec3<f32>(2.0, 2.0, 1.0));
    
    let fresnel = pow(1.0 - abs(dot(normal, -rd)), 5.0);
    let b = osc(0.1, 0.5, 5.0, u_time.time);
    
    let ao = smoothstep(0.0, 0.1, getSceneDist(p + normal * b));
    
    return baseColor * (0.5 + 0.2 * fresnel) * (0.5 + 0.5 * ao);
}

@fragment
fn main(@builtin(position) fc: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(1920.0, 1080.0);
    let aspect = resolution.y / resolution.x;
    var uv = -1.8 + 3.5 * (fc.xy / resolution);
    uv.y *= aspect;
    
    var total = vec3<f32>(0.0);
    let samples = i32(params.lambda);
    
    for(var i = 0; i < samples; i++) {
        let fi = f32(i);
        let angle = 2.0 * PI * (fi / f32(samples));
        let rd = vec2<f32>(cos(angle), sin(angle));
        
        var t = 0.0;
        let maxSteps = i32(params.a);
        var hit = false;
        
        for(var step = 0; step < maxSteps; step++) {
            let d = getSceneDist(uv + rd * t);
            if(d < 0.001) {
                hit = true;
                break;
            }
            t += max(d * 0.5, 0.01);
            if(t > 2.0) { break; }
        }
        
        if(hit) {
            total += calcLighting(uv, rd, t);
        }
    }
    
    total /= f32(samples);
    
    let d = getSceneDist(uv);
    let zoomLevel = oscWithPause(-6.0, -33.0, 5.0, 33.0, u_time.time);
    
    let glow = vec3<f32>(params.theta, params.alpha, params.sigma) * exp(zoomLevel * abs(d));
    total += glow * 0.5;
    
    let exposure = 1.5;
    let color = pow(total * exposure, vec3<f32>(1.0/2.2));
    
    let bgCol = vec3<f32>(0.1, 0.15, 0.2) + 0.05 * cos(length(uv) - u_time.time * 0.3);
    var finalColor = mix(bgCol, color, smoothstep(0.0, 0.01, total.r));
    finalColor = applyGamma(finalColor, params.aa);

    return vec4<f32>(finalColor, 1.0);
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn oscWithPause(minValue: f32, maxValue: f32, interval: f32, pauseDuration: f32, currentTime: f32) -> f32 {
    let cycleTime = interval * 2.0 + pauseDuration;
    let phase = currentTime - (floor(currentTime / cycleTime) * cycleTime);
    if (phase < interval) {
        return mix(maxValue, minValue, phase / interval);
    } else if (phase < interval + pauseDuration) {
        return minValue;
    } else {
        return mix(minValue, maxValue, (phase - interval - pauseDuration) / interval);
    }
}

fn osc(mV: f32, MV: f32, i: f32, t: f32) -> f32 {
    return mV + (MV - mV) * 0.5 * (sin(2.0 * PI * t / i) + 1.0);
}