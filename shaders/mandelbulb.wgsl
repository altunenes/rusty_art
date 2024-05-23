const PI: f32 = 3.141592653589793;
const MAX_STEPS: i32 = 250;
const SURFACE_DIST: f32 = 0.001;
const MAX_DIST: f32 = 150.0;
const BAILOUT: f32 = 4.0;
const AMBIENT: f32 = 0.1;
const SPECULAR_COEFF: f32 = 0.5;
const SHININESS: f32 = 0.5;
const EPS: vec3<f32> = vec3<f32>(0.001, 0.001, 0.001);

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
fn osc2(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * 3.141592653589793 * currentTime / interval) + 1.0);
}
fn osc(minValue: f32, maxValue: f32, interval: f32, pauseDuration: f32, currentTime: f32) -> f32 {
    let cycleTime: f32 = interval * 2.0 + pauseDuration;
    let phase: f32 = currentTime % cycleTime;
    if (phase < interval) {
        return mix(minValue, maxValue, phase / interval);
    } else if (phase < interval + pauseDuration) {
        return minValue;
    } else {
        return mix(minValue, maxValue, (phase - interval - pauseDuration) / interval);
    }
}
fn mandelbulb(pos: vec3<f32>, u_time: TimeUniform) -> f32 {
    let power_osc: f32 = osc(params.iter, params.iter, 20.0, 5.0, u_time.time);
    var z: vec3<f32> = pos;
    var dr: f32 = 1.0;
    var r: f32 = 0.0;
    let influence: f32 = params.bound; 
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        r = length(z);
        if (r > BAILOUT) {
            break;
        }
        let theta: f32 = acos(z.z / r);
        let phi: f32 = atan2(z.y, z.x);

        if (r < 1.0) {
            dr = pow(r, power_osc - 1.0) * power_osc * dr + influence * 0.5;
        } else { 
            dr = pow(r, power_osc - 1.0) * power_osc * dr + influence;
        }
        let zr: f32 = pow(r, power_osc);
        let newTheta: f32 = theta * power_osc;
        let newPhi: f32 = phi * power_osc;

        z = zr * vec3<f32>(sin(newTheta) * cos(newPhi), sin(newPhi) * sin(newTheta), cos(newTheta));
        z += pos;
    }
    return 0.5 * log(r) * r / dr;
}
fn normal(p: vec3<f32>, u_time: TimeUniform) -> vec3<f32> {
    let eps: f32 = osc(params.tt, params.tt, 5.0, 0.0, u_time.time);
    let e: vec3<f32> = vec3<f32>(eps, eps, eps); 

    let d: f32 = mandelbulb(p, u_time);
    let n: vec3<f32> = vec3<f32>(
        d - mandelbulb(p - vec3<f32>(e.x, 0.0, 0.0), u_time),
        d - mandelbulb(p - vec3<f32>(0.0, e.y, 0.0), u_time),
        d - mandelbulb(p - vec3<f32>(0.0, 0.0, e.z), u_time)
    );
    return normalize(n);
}

fn bg(uv: vec2<f32>) -> vec3<f32> {
    let top: vec3<f32> = vec3<f32>(1.0, 0.9, 0.8); 
    let bot: vec3<f32> = vec3<f32>(0.9, 0.1, 0.0);
    return mix(bot, top, pow(uv.y * 0.5 + 0.5, 0.5));
}

fn colorize(pos: vec3<f32>, normal: vec3<f32>, dist: f32, viewDir: vec3<f32>, lightDir: vec3<f32>) -> vec3<f32> {
    let first: vec3<f32> = vec3<f32>(params.lambda, params.theta, params.alpha); 
    let mid: vec3<f32> = vec3<f32>(params.sigma, params.gamma, params.blue); 
    let end: vec3<f32> = vec3<f32>(params.a, params.b, params.c); 
    let fresnel: f32 = pow(1.0 - max(dot(normal, viewDir), 0.0), 2.0);
    let fresnelc: vec3<f32> = mix(first, end, fresnel);
    let depthHue: f32 = 0.5 + 0.5 * sin(dist * 1.1 + dot(normal, lightDir) * 0.5);
    let depth: vec3<f32> = mix(mid, end, depthHue);

    let hShif: f32 = atan2(pos.y, pos.x) * 0.15;
    let shift: vec3<f32> = vec3<f32>(sin(hShif + 1.57), cos(hShif + 1.57), sin(hShif - 1.57));
    let vC: vec3<f32> = vec3<f32>(0.7, 1.0, 0.5) + 0.2 * shift;  

    let bran: f32 = length(pos) % 1.0;
    let branC: vec3<f32> = vec3<f32>(params.d, params.e, params.f) * (0.5 + 0.5 * sin(bran * 12.283185)); 

    var combo: vec3<f32> = mix(fresnelc, depth, 0.5);
    combo = mix(combo, vC, 0.1);

    return mix(combo, branC, 0.5 + 1.0 * sin(bran * 3.14159265));
}

fn rotateZ(p: vec3<f32>, a: f32) -> vec3<f32> {
    let s: f32 = sin(a);
    let c: f32 = cos(a);
    return vec3<f32>(c * p.x - s * p.y, s * p.x + c * p.y, p.z);
}
fn rotateY(p: vec3<f32>, a: f32) -> vec3<f32> {
    let s: f32 = sin(a);
    let c: f32 = cos(a);
    return vec3<f32>(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}


fn light(n: vec3<f32>, lightDir: vec3<f32>, viewDir: vec3<f32>, reflectDir: vec3<f32>, u_time: TimeUniform) -> vec3<f32> {
    var diff: f32 = max(dot(n, lightDir), 0.2); 
    let spec: f32 = pow(max(dot(viewDir, reflectDir), 0.0), SHININESS); 
    let shadow: f32 = smoothstep(0.3, 1.0, diff);
    diff *= shadow; 
    let POWER2: f32 = osc(params.g, params.g, 10.0, 3.0, u_time.time);
    return vec3<f32>(POWER2) + diff + SPECULAR_COEFF * spec;
}
fn MARCH(ro: vec3<f32>, rd: vec3<f32>, minDist: f32, maxDist: f32, u_time: TimeUniform) -> f32 {
    var depth: f32 = 0.0;
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        let pos: vec3<f32> = ro + rd * depth;
        let dist: f32 = mandelbulb(pos, u_time); 
        if (dist < minDist) {
            return depth; 
        }
        depth += dist * clamp(dist / minDist, 0.1, 1.0);
        if (depth >= maxDist) {
            break; 
        }
    }
    return maxDist; 
}
fn Sharp(color: vec3<f32>, neighbor1: vec3<f32>, neighbor2: vec3<f32>) -> vec3<f32> {
    return color + 0.1 * (color - (neighbor1 + neighbor2) * 0.5);
}

fn tone(color: vec3<f32>) -> vec3<f32> {
    return color / (color + vec3<f32>(1.0));
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let AA_LEVEL: i32 = i32(params.aa);
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv: vec2<f32> = 0.8 * (FragCoord.xy - 0.5 * resolution) / resolution.y;
    var finalColor: vec3<f32> = vec3<f32>(0.0);
    var accumm: vec3<f32> = vec3<f32>(0.0); 
    var neighbor: i32 = 0;
    let backgroundColor: vec3<f32> = bg(uv);
    let cameraaaa: f32 = osc2(-3.0, -5.0, 25.0, u_time.time);
    for (var i: i32 = 0; i < AA_LEVEL; i = i + 1) {
        for (var j: i32 = 0; j < AA_LEVEL; j = j + 1) {
            let sampleUV: vec2<f32> = uv + (vec2<f32>(f32(i), f32(j)) - 0.5 * vec2<f32>(f32(AA_LEVEL) - 1.0)) / vec2<f32>(resolution.y, resolution.y) / f32(AA_LEVEL);
            let camPos: vec3<f32> = rotateY(vec3<f32>(0.0, 0.0, cameraaaa), u_time.time * 0.25);
            var rayDir: vec3<f32> = normalize(vec3<f32>(sampleUV, 2.0));
            rayDir = rotateZ(rayDir, sin(u_time.time * 0.25) * 1.5);
            rayDir = rotateY(rayDir, u_time.time * 0.25);
            let totalDist: f32 = MARCH(camPos, rayDir, SURFACE_DIST, MAX_DIST,u_time);
            if (totalDist < MAX_DIST) {
                let p: vec3<f32> = camPos + totalDist * rayDir;
                let n: vec3<f32> = normal(p,u_time);
                let lightDir: vec3<f32> = normalize(vec3<f32>(0.5, 1.0, -0.5));
                let viewDir: vec3<f32> = normalize(-rayDir);
                let reflectDir: vec3<f32> = reflect(-lightDir, n);
                let color: vec3<f32> = colorize(p, n, totalDist, viewDir, lightDir);
                let lightEffect: vec3<f32> = light(n, lightDir, viewDir, reflectDir,u_time);
                finalColor += color * lightEffect;
                accumm += color; 
                neighbor += 1;
            } else {
                finalColor += backgroundColor;
            }
        }
    }
    
    if (neighbor > 1) {
        finalColor = Sharp(finalColor, accumm / f32(neighbor), finalColor / f32(neighbor));
    }
    finalColor = tone(finalColor);  
    finalColor = applyGamma(finalColor, 0.5);
    return vec4<f32>(finalColor, 1.0);
}