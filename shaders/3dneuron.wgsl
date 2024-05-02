const PI: f32 = 3.141592653589793;
const MAX_STEPS: i32 = 32;
const MAX_DISTANCE: f32 = 45.0;
const SURFACE_DIST: f32 = 0.01;
const NUM_DENDRITES: i32 = 15;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0) var<uniform> u_time: TimeUniform;
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn modf(x: f32, y: f32) -> f32 {
    return x - y * floor(x / y);
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
fn r2d(a: f32) -> mat2x2<f32> {
    let c: f32 = cos(a);
    let s: f32 = sin(a);
    return mat2x2<f32>(c, -s, s, c);
}
fn random(seed: f32) -> f32 {
    return fract(sin(seed * params.alpha) * params.sigma);
}
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h: f32 = clamp(0.5 + 0.5 * (b - a) / k, 0.0, 1.0);
    return mix(b, a, h) - k * h * (1.0 - h);
}
fn sphereSDF(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
}
fn sdSegment(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
    let pa: vec3<f32> = p - a;
    let ba: vec3<f32> = b - a;
    let h: f32 = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h) - r;
}
fn dendriteSDF(p: vec3<f32>, origin: vec3<f32>, seed: f32) -> f32 {
    let angle: f32 = random(seed) * params.lambda * PI;
    let dir: vec3<f32> = normalize(vec3<f32>(cos(angle), sin(angle), random(seed + 1.0) * 2.0 - 1.0));
    var dendrite: vec3<f32> = origin + dir * 0.5;
    var d: f32 = sdSegment(p, origin, dendrite, 0.05);
    var lengthFactor: f32;
    var branchAngle: f32;
    for (var i: i32 = 1; i < 5; i++) {
        let nextSeed: f32 = seed + f32(i) * 0.5;
        lengthFactor = random(nextSeed + 0.3) * 0.25 + 0.15;
        branchAngle = random(nextSeed + 0.5) * PI * 0.5 - PI * 0.25;
        let nextDir: vec3<f32> = normalize(vec3<f32>(cos(angle + branchAngle), sin(angle + branchAngle), random(nextSeed + 1.0) * 2.0 - 1.0));
        let nextSegment: vec3<f32> = dendrite + nextDir * lengthFactor;
        d = smin(d, sdSegment(p, dendrite, nextSegment, 0.01), 0.1);
        dendrite = nextSegment;
    }
    return d;
}
fn axonSDF(p: vec3<f32>, origin: vec3<f32>, seed: f32, thickness: f32) -> vec4<f32> {
    let angle: f32 = random(seed) * params.theta * PI;
    let dir: vec3<f32> = normalize(vec3<f32>(cos(angle), sin(angle), random(seed + 1.0) * 2.0 - 1.0));
    let axon: vec3<f32> = origin + dir * 0.5;
    var d: f32 = sdSegment(p, origin, axon, thickness);
    var lastPoint: vec3<f32> = axon;
    let segmentCount: i32 = 7;
    for (var i: i32 = 1; i < segmentCount; i = i + 1) {
        let nextSeed: f32 = seed + f32(i) * 0.1;
        let nextDir: vec3<f32> = normalize(vec3<f32>(cos(angle + random(nextSeed)), sin(angle + random(nextSeed)), random(nextSeed + 1.0) * 2.0 - 1.0));
        let nextSegment: vec3<f32> = lastPoint + nextDir * 0.5;
        d = smin(d, sdSegment(p, lastPoint, nextSegment, thickness), 0.1);
        lastPoint = nextSegment;
    }
    let terminalSeed: f32 = seed + f32(segmentCount) * 0.1;
    let terminalDir: vec3<f32> = normalize(vec3<f32>(cos(angle + random(terminalSeed)), sin(angle + random(terminalSeed)), random(terminalSeed + 1.0) * 2.0 - 1.0));
    lastPoint = lastPoint + terminalDir * 0.3;
    return vec4<f32>(d, lastPoint);
}
fn terminalSDF(p: vec3<f32>, start: vec3<f32>, seed: f32) -> f32 {
    let thickness: f32 = 0.01; 
    var d: f32 = 1e3;
    for (var i: i32 = 0; i < 4; i = i + 1) {
        let angle: f32 = random(seed + f32(i)) * 3.0 * PI;
        let dir: vec3<f32> = normalize(vec3<f32>(cos(angle), sin(angle), random(seed + f32(i) + 1.0) * 2.0 - 1.0));
        var terminal: vec3<f32> = start + dir * 0.2;
        d = min(d, sdSegment(p, start, terminal, thickness));
        for (var j: i32 = 1; j < 3; j = j + 1) {
            let nextSeed: f32 = seed + f32(j) * 0.05;
            let nextDir: vec3<f32> = normalize(vec3<f32>(cos(angle + random(nextSeed)), sin(angle + random(nextSeed)), random(nextSeed + 1.0) * 2.0 - 1.0));
            let nextSegment: vec3<f32> = terminal + nextDir * 0.2;
            d = smin(d, sdSegment(p, terminal, nextSegment, thickness), 0.05);
            terminal = nextSegment;
        }
    }
    return d;
}
fn sceneSDF(p: vec3<f32>, iTime: f32) -> f32 {
    var p_mod: vec3<f32> = p;
    let rotation: mat2x2<f32> = r2d(iTime * 0.7);
    let rotated_xz: vec2<f32> = rotation * p_mod.xz;
    p_mod.x = rotated_xz.x;
    p_mod.z = rotated_xz.y;
    p_mod.y -= 1.0;
    let soma: f32 = sphereSDF(p_mod, 0.4);  
    var dendrites: f32 = 1e3;
    let axonThickness: f32 = 0.04; 
    for (var i: i32 = 0; i < NUM_DENDRITES; i = i + 1) {
        dendrites = smin(dendrites, dendriteSDF(p_mod, vec3<f32>(0.0, 0.0, 0.0), f32(i)), 0.3);
    }
    let axonData: vec4<f32> = axonSDF(p_mod, vec3<f32>(0.0, 0.0, 0.0), 1.5, axonThickness);
    let axon: f32 = axonData.x;
    let axonEndPoint: vec3<f32> = axonData.yzw;
    let terminals: f32 = terminalSDF(p_mod, axonEndPoint, 2.5); 
    var neuron: f32 = smin(soma, dendrites, 0.2);
    neuron = smin(neuron, axon, 0.2);
    neuron = smin(neuron, terminals, 0.3); 
    return neuron;
}
fn normal(p: vec3<f32>, time: f32) -> vec3<f32> {
    let h: f32 = 0.01;
    return normalize(vec3<f32>(
        sceneSDF(p + vec3<f32>(h, 0.0, 0.0), time) - sceneSDF(p - vec3<f32>(h, 0.0, 0.0), time),
        sceneSDF(p + vec3<f32>(0.0, h, 0.0), time) - sceneSDF(p - vec3<f32>(0.0, h, 0.0), time),
        sceneSDF(p + vec3<f32>(0.0, 0.0, h), time) - sceneSDF(p - vec3<f32>(0.0, 0.0, h), time)
    ));
}
fn diffuseLighting(p: vec3<f32>, lightPos: vec3<f32>, normal: vec3<f32>) -> f32 {
    let lightDir: vec3<f32> = normalize(lightPos - p);
    return max(dot(lightDir, normal), 0.0);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = params.gamma*(vec2<f32>(FragCoord.x / resolution.x, 1.0 - FragCoord.y / resolution.y) * 2.0 - vec2<f32>(1.0, 1.0));
    uv.x *= resolution.x / resolution.y;
    let camPos: vec3<f32> = vec3<f32>(0.0, 0.0, 3.0);
    let rayDir: vec3<f32> = normalize(vec3<f32>(uv, -1.0));
    var p: vec3<f32> = camPos;
    var dist: f32;
    var totalDist: f32 = 0.0;
    let axonActivationPhase: f32 = modf(u_time.time, params.blue);
    let lightYellow: vec3<f32> = vec3<f32>(params.noise,params.noise2, params.color);
    let mixAmount2: f32 = 0.5 + 0.5 * (sin(axonActivationPhase * 2.0 * PI) * 0.5 + 0.5);
    let actionPotentialColor: vec3<f32> = mix(vec3<f32>(1.0, 0.7, 0.2), lightYellow, mixAmount2);
    let baseRed: vec3<f32> = vec3<f32>(0.5, 0.0, 0.0);
    let mixAmount: f32 = 0.5;
    let nucleusColor: vec3<f32> = mix(vec3<f32>(1.0, 0.3, 0.5), baseRed, mixAmount);
    for (var i: i32 = 0; i < MAX_STEPS; i = i + 1) {
        dist = sceneSDF(p, u_time.time);
        if (dist < SURFACE_DIST) {
            break;
        }
        totalDist += dist;
        p += dist * rayDir;
        if (totalDist > MAX_DISTANCE) {
            break;
        }
    }
    var color: vec3<f32> = vec3<f32>(0.0);
    if (dist < SURFACE_DIST) {
        let normal: vec3<f32> = normal(p,u_time.time);
        let lightIntensity: f32 = diffuseLighting(p, vec3<f32>(2.0, 2.0, 5.0), normal);
        let coreRadius: f32 = params.fw;
        let coreBorder: f32 = 0.2;
        let coreDistance: f32 = length(p - vec3<f32>(0.0, 1.0, 0.0));
        let coreIntensity: f32 = smoothstep(coreRadius, coreRadius - coreBorder, coreDistance);
        let coreColor: vec3<f32> = vec3<f32>(params.fh, params.fcx, params.fcyf);
        color = mix(color, coreColor, coreIntensity);
        let axonData: vec4<f32> = axonSDF(camPos, vec3<f32>(0.0), 2.5, 0.08);
        let axonLength: f32 = length(axonData.yzw - camPos);
        let activationPoint: f32 = axonActivationPhase * axonLength;
        let pointDistanceAlongAxon: f32 = dot(p - camPos, normalize(rayDir));
        let glowIntensity: f32 = exp(-5.0 * abs(pointDistanceAlongAxon - activationPoint));
        let glowingActionPotential: vec3<f32> = actionPotentialColor * glowIntensity;
        let haloRadius: f32 = 0.1;
        let haloIntensity: f32 = smoothstep(haloRadius, 0.0, abs(pointDistanceAlongAxon - activationPoint));
        let haloColor: vec3<f32> = vec3<f32>(1.0, 1.0, 0.6) * haloIntensity;
        color = mix(color, glowingActionPotential, clamp(glowIntensity, 0.0, 1.0));
        color = mix(color, haloColor, haloIntensity);
        let fresnel: f32 = pow(1.0 + dot(rayDir, normal), 4.0);
        color = mix(color, vec3<f32>(1.0), fresnel * lightIntensity);
    }
     return vec4<f32>(color, 1.0);
}
