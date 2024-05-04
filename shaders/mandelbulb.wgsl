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
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        r = length(z);
        if (r > BAILOUT) {
            break;
        }

        let theta: f32 = acos(z.z / r);
        let phi: f32 = atan2(z.y, z.x);
        dr = pow(r, POWER - 1.0) * POWER * dr + 1.0;

        let zr: f32 = pow(r, POWER);
        let newTheta: f32 = theta * POWER;
        let newPhi: f32 = phi * POWER;
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
fn colorize(d: f32, n: vec3<f32>, lightDir: vec3<f32>, time: f32) -> vec3<f32> {
    let angle: f32 = dot(n, lightDir) * params.blue + params.bound;
    let base: vec3<f32> = 0.5 + params.aa * cos(params.iter * vec3<f32>(1.0, 2.0, 3.0) * d + vec3<f32>(0.0, 0.5, 1.0) + time * vec3<f32>(0.3, 0.2, 0.1));
    return mix(base, vec3<f32>(angle, angle * 0.5, angle * 0.25), 0.5);
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
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    let time: f32 = u_time.time * params.gamma;
    let camPos: vec3<f32> = rotateY(vec3<f32>(params.theta, params.alpha, -params.lambda), time);
    var uv: vec2<f32> = (FragCoord.xy - 0.5 * resolution.xy) / resolution.y * vec2<f32>(2.0, 1.0);
    var rayDir: vec3<f32> = normalize(vec3<f32>(uv, params.sigma));
    rayDir = rotateZ(rayDir, sin(time) * 0.5);
    rayDir = rotateY(rayDir, time);

    var totalDist: f32 = 0.0;
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        let p: vec3<f32> = camPos + totalDist * rayDir;
        let dist: f32 = mandelbulb(p);
        totalDist += dist;
        if (dist < SURFACE_DIST || totalDist > MAX_DIST) {
            break;
        }
    }

    var color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    if (totalDist < MAX_DIST) {
        let p: vec3<f32> = camPos + totalDist * rayDir;
        let n: vec3<f32> = normal(p);
        let lightDir: vec3<f32> = normalize(vec3<f32>(0.5, 1.0, -0.5));
        let viewDir: vec3<f32> = normalize(-rayDir);
        let reflectDir: vec3<f32> = reflect(-lightDir, n);
        let diff: f32 = max(dot(n, lightDir), 0.0);
        let spec: f32 = pow(max(dot(viewDir, reflectDir), 0.0), SHININESS);
        let baseColor: vec3<f32> = colorize(totalDist, n, lightDir, u_time.time);
        color = baseColor * (AMBIENT + diff + SPECULAR_COEFF * spec);
    }

    return vec4<f32>(color, 1.0);
}