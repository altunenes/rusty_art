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


fn sat(a: f32) -> f32 {
    return clamp(a, 0.0, 1.0);
}
fn r2d(a: f32) -> mat2x2<f32> {
    let c: f32 = cos(a);
    let s: f32 = sin(a);
    return mat2x2<f32>(c, -s, s, c);
}
fn smin(a: f32, b: f32, k: f32) -> f32 {
    let h: f32 = sat(params.noise + params.noise2 * (b - a) / k);
    return mix(b, a, h) - k * h * (1.0 - h);
}
fn boxSDF(p: vec3<f32>, b: vec3<f32>, offset: f32) -> f32 {
    var mP: vec3<f32> = p;
    mP.y -= offset;
    let d: vec3<f32> = abs(mP) - b;
    return length(max(d, vec3<f32>(0.0))) + min(max(d.x, max(d.y, d.z)), 0.0);
}

fn wingSDF(p: vec3<f32>, start: vec3<f32>, end: vec3<f32>, control: vec3<f32>, thickness: f32) -> f32 {
    let t: f32 = clamp((p.y - start.y) / (end.y - start.y), 0.4, 1.0);
    let onCurve: vec3<f32> = mix(mix(start, control, t), mix(control, end, t), t);
    let th: f32 = mix(thickness, thickness * params.lambda, t);
    return length(p - onCurve) - th;
}

fn psiSymbol(p: vec3<f32>) -> f32 {
    let wingControlLeft: vec3<f32> = vec3<f32>(-0.35, 0.0, 0.0);
    let wingControlRight: vec3<f32> = vec3<f32>(0.35, 0.0, 0.0);
    let wingThickness: f32 =params.alpha;
    let stemOffset: f32 = -0.1;
    let stemHeight: f32 = 0.28;
    let stem: f32 = boxSDF(p, vec3<f32>(params.theta, stemHeight, params.theta), stemOffset);
    let leftWing: f32 = wingSDF(p, vec3<f32>(-0.05, params.fcx - stemOffset, 0.0), vec3<f32>(-0.05, -0.1, 0.0), wingControlLeft, wingThickness);
    let rightWing: f32 = wingSDF(p, vec3<f32>(0.05, params.fcy - stemOffset, 0.0), vec3<f32>(0.05, -0.1, 0.0), wingControlRight, wingThickness);
    let wings: f32 = smin(leftWing, rightWing, 0.05);
    return smin(stem, wings, 0.05);
}

fn scene(p: vec3<f32>, time: f32) -> f32 {
    let rotation: mat2x2<f32> = r2d(time * 0.7);
    let rotated_p: vec2<f32> = rotation * p.xz;

    var mutableP: vec3<f32> = p;
    mutableP.x = rotated_p.x;
    mutableP.z = rotated_p.y; 

    return psiSymbol(mutableP);
}

fn normal(p: vec3<f32>, time: f32) -> vec3<f32> {
    let h: f32 = 0.0001;
    return normalize(vec3<f32>(
        scene(p + vec3<f32>(1.0, -1.0, -1.0) * h, time) - scene(p + vec3<f32>(-1.0, 1.0, -1.0) * h, time),
        scene(p + vec3<f32>(-1.0, 1.0, -1.0) * h, time) - scene(p + vec3<f32>(-1.0, -1.0, 1.0) * h, time),
        scene(p + vec3<f32>(-1.0, -1.0, 1.0) * h, time) - scene(p + vec3<f32>(-1.0, -1.0, -1.0) * h, time)
    ));
}
fn softShadow(ro: vec3<f32>, rd: vec3<f32>, mint: f32, maxt: f32, k: f32) -> f32 {
    var res: f32 = 1.0;
    var t: f32 = mint;
    loop {
        let h: f32 = scene(ro + rd * t, u_time.time );
        res = min(res, k * h / t);
        t += clamp(h, 0.02, 0.1);
        if (h < 0.001 || t > maxt) {
            break;
        }
    }
    return clamp(res, 0.0, 1.0);
}
fn fakeCubemap(direction: vec3<f32>, time: f32) -> vec3<f32> {
    let normalizedDirection: vec3<f32> = normalize(direction);
    let theta: f32 = acos(normalizedDirection.y);
    let phi: f32 = atan2(normalizedDirection.z, normalizedDirection.x);
    let timeFactor: f32 = sin(time * params.fh) *  params.fh + params.fh;
    let skyColor: vec3<f32> = vec3<f32>(0.1, 0.2, 0.5) + params.blue * vec3<f32>(
        cos(phi + timeFactor * PI), 
        sin(phi + PI / 4.0 + timeFactor * PI), 
        cos(phi + PI / 2.0 + timeFactor * PI)
    );
    let groundColor: vec3<f32> = vec3<f32>(params.color, 0.1, 0.1) + params.fw * vec3<f32>(
        sin(phi + timeFactor * PI), 
        cos(phi - PI / 4.0 + timeFactor * PI), 
        sin(phi - PI / 2.0 + timeFactor * PI)
    );
    let horizonColor: vec3<f32> = mix(skyColor, groundColor, sin(theta) * sin(theta) + 0.1 * sin(time * 0.2));
    return mix(groundColor, horizonColor, normalizedDirection.y * 0.5 + 0.5 + 0.1 * cos(time * 0.3));
}
fn calculateLighting(p: vec3<f32>, n: vec3<f32>, lightPos: vec3<f32>, cameraPos: vec3<f32>, baseColor: vec3<f32>, time: f32) -> vec3<f32> {
    let lightDir: vec3<f32> = normalize(lightPos - p);
    let viewDir: vec3<f32> = normalize(cameraPos - p);
    let halfDir: vec3<f32> = normalize(lightDir + viewDir);
    let diff: f32 = max(dot(n, lightDir), 0.0);
    let spec: f32 = pow(max(dot(n, halfDir), 0.0), 32.0);
    let shadow: f32 = softShadow(p + n * params.sigma, lightDir, params.sigma, 2.5, 32.0);
    let reflection: vec3<f32> = fakeCubemap(reflect(-viewDir, n), time);
    var color: vec3<f32> = (baseColor * diff + vec3<f32>(1.0) * spec) * shadow;
    color = mix(color, reflection, 0.25);
    return color;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = vec2<f32>(FragCoord.x / resolution.x, 1.0 - FragCoord.y / resolution.y) * 2.0 - vec2<f32>(1.0, 1.0);
    uv.x *= resolution.x / resolution.y;
    let ro: vec3<f32> = vec3<f32>(0.0, 0.0, params.gamma);
    let rd: vec3<f32> = normalize(vec3<f32>(uv, 1.0));
    var depth: f32 = 0.0;
    loop {
        let pos: vec3<f32> = ro + rd * depth;
        let d: f32 = scene(pos, u_time.time);
        if (d < 0.01 || depth > 15.0) {
            break;
        }
        depth += d;
    }

    var col: vec3<f32>;
    if (depth < 15.0) {
        let p: vec3<f32> = ro + depth * rd;
        let n: vec3<f32> = normal(p, u_time.time);
        let lightPos: vec3<f32> = vec3<f32>(0.0, 0.5, -1.0);
        let baseColor: vec3<f32> = vec3<f32>(1.0, 0.8, 0.6);
        col = calculateLighting(p, n, lightPos, ro, vec3<f32>(1.0, 0.8, 0.6), u_time.time);
    } else {
        col = fakeCubemap(rd, u_time.time); 
    }

    return vec4<f32>(col, 1.0);
}