const PI: f32 = 3.141592653589793;
const MAX_STEPS: i32 = 150;
const SURFACE_DIST: f32 = 0.001;
const MAX_DIST: f32 = 150.0;
const BAILOUT: f32 = 2.0;
const POWER: f32 = 8.0;
const AMBIENT: f32 = 0.2;
const SPECULAR_COEFF: f32 = 0.5;
const SHININESS: f32 = 1.0;
const EPS: vec3<f32> = vec3<f32>(0.001, 0.001, 0.001);
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
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
fn mandelbulb(pos: vec3<f32>) -> f32 {
    var z: vec3<f32> = pos;
    var dr: f32 = 1.0;
    var r: f32 = 0.0;
    var zerg: f32 = POWER+params.iter;
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        r = length(z);
        if (r > BAILOUT) {
            break;
        }
        let theta: f32 = acos(z.z / r);
        let phi: f32 = atan2(z.y, z.x);
        dr = pow(r, zerg - 1.0) * zerg * dr + 1.0;
        let zr: f32 = pow(r, zerg);
        let newTheta: f32 = theta * zerg;
        let newPhi: f32 = phi * zerg;
        z = zr * vec3<f32>(sin(newTheta) * cos(newPhi), sin(newPhi) * sin(newTheta), cos(newTheta));
        z = z + pos;
    }
    return 0.5 * log(r) * r / dr;
}
fn normal(p: vec3<f32>) -> vec3<f32> {
    let d: f32 = mandelbulb(p);
    return normalize(vec3<f32>(
        d - mandelbulb(p - vec3<f32>(EPS.x, 0.0, 0.0)),
        d - mandelbulb(p - vec3<f32>(0.0, EPS.y, 0.0)),
        d - mandelbulb(p - vec3<f32>(0.0, 0.0, EPS.z))
    ));
}
fn colorize(pos: vec3<f32>, normal: vec3<f32>, dist: f32, time: f32) -> vec3<f32> {
    let baseColor: vec3<f32> = vec3<f32>(0.3, 0.3, 0.3);
    let iter_ratio: f32 = clamp(dist / 0.0, 1.0, 1.0);
    let lenSq: f32 = length(pos);
    let exteriorColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    if (iter_ratio >= 1.0) {
        let col1: vec3<f32> = 0.5 + params.sigma * sin(params.a + vec3<f32>(params.b, params.c, params.d) + PI * vec3<f32>(2.0 * lenSq) + time / 2.0);
        let col2: vec3<f32> = 0.5 + params.blue * sin(1.5 + PI * vec3<f32>(lenSq) + time / 2.0);
        return baseColor + 1.0 * sqrt(col1 * col2);
    } else {
        return mix(baseColor, exteriorColor, params.e);
    }
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
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let AA_LEVEL: i32 = i32(params.aa);
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    let time: f32 = u_time.time * params.bound;
    let camPos: vec3<f32> = rotateY(vec3<f32>(params.theta, params.alpha, -params.lambda), time);
    var finalColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let totalSamples: f32 = f32(AA_LEVEL * AA_LEVEL);
    for (var i: i32 = 0; i < AA_LEVEL; i = i + 1) {
        for (var j: i32 = 0; j < AA_LEVEL; j = j + 1) {
            let sampleUV: vec2<f32> = (FragCoord.xy - 0.5 * resolution.xy) / resolution.y + 
            (vec2<f32>(f32(i), f32(j)) - 0.5 * vec2<f32>(f32(AA_LEVEL) - 1.0)) / resolution.y / f32(AA_LEVEL);
            var rayDir: vec3<f32> = normalize(vec3<f32>(sampleUV, 2.0));
            rayDir = rotateZ(rayDir, sin(time) * 0.5);
            rayDir = rotateY(rayDir, time);
            var totalDist: f32 = 0.0;
            for (var k: i32 = 0; k < MAX_STEPS; k = k + 1) {
                let p: vec3<f32> = camPos + totalDist * rayDir;
                let dist: f32 = mandelbulb(p);
                totalDist += dist;
                if (dist < SURFACE_DIST || totalDist > MAX_DIST) {
                    break;
                }
            }
            if (totalDist < MAX_DIST) {
                let p: vec3<f32> = camPos + totalDist * rayDir;
                let n: vec3<f32> = normal(p);
                let lightDir: vec3<f32> = normalize(vec3<f32>(0.5, 1.0, -0.5));
                let viewDir: vec3<f32> = normalize(-rayDir);
                let reflectDir: vec3<f32> = reflect(-lightDir, n);
                let diff: f32 = max(dot(n, lightDir), 0.0);
                let spec: f32 = pow(max(dot(viewDir, reflectDir), 0.0), SHININESS);
                let baseColor: vec3<f32> = colorize(p, n, totalDist, u_time.time);
                finalColor += baseColor * (AMBIENT + diff + SPECULAR_COEFF * spec);
            }
        }
    }
    finalColor = applyGamma(finalColor,params.gamma);
    finalColor /= totalSamples;
    return vec4<f32>(finalColor, 1.0);
}
