const PI: f32 = 3.14159265359;
const TAU: f32 = 6.28;
const LI_STEPS: i32 = 64;
const SH_SOFT: f32 = 64.0;
const AO_STEPS: i32 = 5;
const BOUNCE_LI: f32 = 0.4;

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

struct Dec {
    dist: f32,
    col: vec3<f32>,
    typ: i32,
}

struct SceneData {
    trunk: f32,
    decs: f32,
    star: f32,
    decCol: vec3<f32>,
    trkCol: vec3<f32>,
    strCol: vec3<f32>,
    nearDec: Dec,
}

var<private> scene: SceneData;

fn rot2d(a: f32) -> mat2x2<f32> {
    let s = sin(a);
    let c = cos(a);
    return mat2x2<f32>(c, s, -s, c);
}
fn fmod(x: f32, y: f32) -> f32 {
    return x - y * floor(x/y);
}

fn sdLine(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

fn opModPolarMirror(p: vec2<f32>, theta: f32, offset: f32) -> vec2<f32> {
    var a = atan2(p.y, p.x) - offset;
    a = abs(fmod(a + 0.5 * theta, theta) - 0.5 * theta);
    return length(p) * vec2<f32>(cos(a), sin(a));
}

fn sdSnow(p: vec2<f32>, scale: f32) -> f32 {
    var pp = p / scale;
    pp = opModPolarMirror(pp, radians(360.0) / 6.0, radians(90.0));
    var d = sdLine(pp, vec2<f32>(0.0, 0.0), vec2<f32>(0.75, 0.0));
    d = min(d, sdLine(pp, vec2<f32>(0.5, 0.0), vec2<f32>(0.5, 0.0) + vec2<f32>(0.1, 0.1)));
    d = min(d, sdLine(pp, vec2<f32>(0.25, 0.0), vec2<f32>(0.25, 0.0) + 1.5 * vec2<f32>(0.1, 0.1)));
    return (d - 0.04) * scale;
}

fn sdStar(p: vec2<f32>, r: f32, rf: f32) -> f32 {
    let k1 = vec2<f32>(0.809016994375, -0.587785252292);
    let k2 = vec2<f32>(-k1.x, k1.y);
    var pp = p;
    pp.x = abs(pp.x);
    pp = pp - 2.0 * max(dot(k1, pp), 0.0) * k1;
    pp = pp - 2.0 * max(dot(k2, pp), 0.0) * k2;
    pp.x = abs(pp.x);
    pp.y = pp.y - r;
    let ba = rf * vec2<f32>(-k1.y, k1.x) - vec2<f32>(0.0, 1.0);
    let h = clamp(dot(pp, ba) / dot(ba, ba), 0.0, r);
    return length(pp - ba * h) * sign(pp.y * ba.x - pp.x * ba.y);
}

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}
fn sdBox(p: vec2<f32>, b: vec2<f32>) -> f32 {
    let d = abs(p) - b;
    return length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
}

fn sdHeart(p: vec2<f32>, size: f32) -> f32 {
    var pp = p / size;
    pp.y = pp.y - 0.3;
    let a = atan2(pp.y, pp.x) / PI;
    let r = length(pp);
    let h = abs(a) * 0.5;
    let hh = 0.5 + 0.3 * cos(h * PI * 2.0);
    return (r - hh) * size;
}

fn rnd(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st.xy, vec2<f32>(12.9898, 78.233))) * 43758.5453123);
}

fn getSceneDist(p: vec2<f32>) -> f32 {
    return min(scene.trunk, min(scene.decs, scene.star));
}
fn snowNoise(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
fn traceShadow(p: vec2<f32>, lightPos: vec2<f32>) -> f32 {
    let dir = normalize(lightPos - p);
    let dist = length(lightPos - p);
    var result = 1.0;
    var t = 0.1;
    let softness = SH_SOFT * (0.5 + 0.5 * (t/dist));
    
    for(var i = 0; i < LI_STEPS; i++) {
        let pos = p + dir * t;
        let h = getSceneDist(pos);
        
        if(h < 0.001) {
            let shadowMult = h * softness / t;
            result = min(result, shadowMult);
        }
        
        t += max(0.005, abs(h));
        if(t > dist) { break; }
    }
    
    return clamp(result, 0.0, 1.0);
}

fn calcAO(p: vec2<f32>, n: vec2<f32>) -> f32 {
    var ao = 0.0;
    let delta = params.iter;
    
    for(var i = 0; i < AO_STEPS; i++) {
        let dist = delta * f32(i + 1);
        let samplePoint = p + n * dist;
        let d = getSceneDist(samplePoint);
        ao += max(0.0, (dist - d) / dist);
    }
    
    return 1.0 - ao * params.alpha;
}

fn getLightContrib(p: vec2<f32>, lightPos: vec2<f32>, lightCol: vec3<f32>, intensity: f32) -> vec3<f32> {
    let dist = length(lightPos - p);
    let shadow = traceShadow(p, lightPos);
    let falloff = 0.3 / (1.0 + dist * dist * 7.1);
    return lightCol * shadow * falloff * intensity * 3.0;
}

fn calcScene(uv: vec2<f32>) {
    scene.decs = 1e10;
    scene.decCol = vec3<f32>(0.0);
    scene.nearDec.dist = 1e10;
    scene.nearDec.col = vec3<f32>(0.0);
    scene.nearDec.typ = -1;
    
    var rowf: f32 = 0.0; 
    for(var row = 0; row < 8; row++) {
        rowf = f32(row); 
        let y = 0.7 - rowf * 0.2;
        let width = mix(0.05, 0.5, rowf/7.0);
        let points = row + 2;
        
        var if32: f32 = 0.0;
        for(var i = 0; i < points; i++) {
            if32 = f32(i); 
            let denom = f32(points - 1);
            let x = mix(-width, width, if32/denom);
            let xm = x + sin(y * 5.0 + u_time.time * 1.5) * 0.03;
            let ym = y + sin(x * 3.0 + u_time.time) * 0.02;
            
            let pos = vec2<f32>(xm, ym);
            
            let itemCol = vec3<f32>(
                0.5 + 0.5 * sin(rowf * 0.5 + u_time.time + 0.0),
                0.5 + 0.5 * sin(rowf * 0.5 + u_time.time + 2.1),
                0.5 + 0.5 * sin(rowf * 0.5 + u_time.time + 4.2)
            );
            
            var decoration: f32;
            let typ = i32(fmod(rowf + if32, 4.0));
            let decUv = uv - pos;
            
            if(typ == 0) {
                let snowUv = rot2d(u_time.time + rowf * 0.5) * decUv;
                decoration = sdSnow(snowUv, 0.08);
                let col = itemCol * vec3<f32>(0.8, 0.9, 1.0);
                
                if(decoration < scene.nearDec.dist) {
                    scene.nearDec.dist = decoration;
                    scene.nearDec.col = col;
                    scene.nearDec.typ = typ;
                }
            } else if(typ == 1) {
                let starUv = rot2d(-u_time.time * 0.3 + if32 * 0.5) * decUv;
                decoration = sdStar(starUv * 3.0, 0.1, 0.3) / 3.0;
                let col = itemCol * vec3<f32>(1.0, 0.8, 0.2);
                
                if(decoration < scene.nearDec.dist) {
                    scene.nearDec.dist = decoration;
                    scene.nearDec.col = col;
                    scene.nearDec.typ = typ;
                }
            } else if(typ == 2) {
                decoration = sdCircle(decUv, 0.03);
                let col = itemCol * vec3<f32>(1.0, 0.2, 0.2);
                
                if(decoration < scene.nearDec.dist) {
                    scene.nearDec.dist = decoration;
                    scene.nearDec.col = col;
                    scene.nearDec.typ = typ;
                }
            } else {
                let heartUv = rot2d(sin(u_time.time + rowf) * 0.2) * decUv;
                decoration = sdHeart(heartUv, 0.04);
                let col = itemCol * vec3<f32>(1.0, 0.3, 0.5);
                
                if(decoration < scene.nearDec.dist) {
                    scene.nearDec.dist = decoration;
                    scene.nearDec.col = col;
                    scene.nearDec.typ = typ;
                }
            }
        }
    }
    
    scene.decs = scene.nearDec.dist;
    scene.decCol = scene.nearDec.col;
    
    let trunkUv = uv - vec2<f32>(0.0, -1.1);
    let woodGrain = sin(trunkUv.x * 1.0 + trunkUv.y * 50.0) * 0.5 + 0.5;
    scene.trunk = sdBox(trunkUv, vec2<f32>(0.08, 0.3));
    scene.trkCol = mix(
        vec3<f32>(0.3, 0.2, 0.1),
        vec3<f32>(0.4, 0.25, 0.15),
        woodGrain
    );
    
    let starPos = vec2<f32>(0.0, 0.9);
    var starUv = uv - starPos;
    starUv = rot2d(u_time.time * 0.5) * starUv;
    scene.star = sdStar(starUv * 2.0, 0.15, 0.4) / 2.0;
    scene.strCol = vec3<f32>(1.0, 0.9, 0.5);
}
fn snowy(uv: vec2<f32>) -> f32 {
    var snow = 0.0;
    let random = snowNoise(uv);
    
    for(var k = 0; k < 4; k++) {
        for(var i = 0; i < 12; i++) {
            let cellSize = 1.0 + (f32(i) * 1.5);
            let downSpeed = 1.1 + (sin(u_time.time * 0.4 + f32(k + i * 20)) + 1.0) * 0.00018;
            
            let snowUV = uv + vec2<f32>(
                0.01 * sin((u_time.time + f32(k * 6185)) * 0.6 + f32(i)) * (5.0 / f32(i)),
                downSpeed * (u_time.time + f32(k * 1352)) * (1.0 / f32(i))
            );
            
            let uvStep = (ceil((snowUV) * cellSize - vec2<f32>(0.5, 0.5)) / cellSize);
            
            let x = snowNoise(uvStep.xy + vec2<f32>(12.0, 315.156) * f32(k)) - 0.5;
            let y = snowNoise(uvStep.xy + vec2<f32>(23.0, 95.0) * f32(k)) - 0.5;
            
            let randMag1 = sin(u_time.time * 2.5) * 0.7 / cellSize;
            let randMag2 = cos(u_time.time * 2.5) * 0.7 / cellSize;
            
            let d = 5.0 * distance(
                (uvStep.xy + vec2<f32>(x * sin(y), y) * randMag1 + vec2<f32>(y, x) * randMag2),
                snowUV.xy
            );
            
            let snowProb = snowNoise(uvStep.xy + vec2<f32>(32.4691, 94.615));
            if(snowProb < 0.08) {
                let snowflake = (x + 1.0) * 0.4 * clamp(1.9 - d * (15.0 + (x * 6.3)) * (cellSize / 1.4), 0.0, 1.0);
                snow = snow + snowflake;
            }
        }
    }
    
    return snow;
}
@fragment
@fragment
fn main(@builtin(position) fc: vec4<f32>) -> @location(0) vec4<f32> {
    let res = vec2<f32>(1920.0, 1080.0);
    let uv = -1.5 * (2.0 * fc.xy - res.xy) / min(res.x, res.y);
    var col = vec3<f32>(0.08, 0.09, 0.11) + vec3<f32>(params.b, params.c, params.d) * length(uv);
    
    calcScene(uv);
    
    let light1Pos = vec2<f32>(0.0, 1.2);
    let light2Pos = vec2<f32>(sin(0.8) * 0.8, cos(u_time.time * 0.5));
    let light3Pos = vec2<f32>(cos(0.8) * -0.8, cos(u_time.time * 0.5));
    
    let eps = vec2<f32>(0.001, 0.0);
    let normal = normalize(vec2<f32>(
        getSceneDist(uv + eps.xy) - getSceneDist(uv - eps.xy),
        getSceneDist(uv + eps.yx) - getSceneDist(uv - eps.yx)
    ));
    
    let ao = calcAO(uv, normal);
    
    let lighting = 
        getLightContrib(uv, light1Pos, vec3<f32>(1.0, 0.9, 0.7), params.g) +
        getLightContrib(uv, light2Pos, vec3<f32>(0.7, 0.8, 1.0), params.e) +
        getLightContrib(uv, light3Pos, vec3<f32>(1.0, 0.8, 0.6), params.f);
    
    let trunkMask = smoothstep(0.001, 0.0, scene.trunk);
    col = mix(col, scene.trkCol, trunkMask);
    
    let starMask = smoothstep(0.002, 0.0, scene.star);
    let starGlow = exp(-scene.star * params.sigma);
    col = mix(col, scene.strCol, starMask);
    col = col + scene.strCol * starGlow * params.tt;
    
    let decMask = smoothstep(params.theta, 0.0, scene.decs);
    col = mix(col, scene.nearDec.col, decMask);
    
    let snow = snowy(uv);
    let gradient = (1.0 - uv.y) * 0.3;
    
    let objectMask = max(max(trunkMask, starMask), decMask);
    let snowAmount = snow * (1.0 - objectMask);

    var snowColor = vec3<f32>(0.9, 0.95, 1.0);
    snowColor = snowColor + vec3<f32>(0.05, 0.02, 0.0) * snowNoise(uv + vec2<f32>(u_time.time, u_time.time));
    col = col * (params.lambda + lighting) * ao;
    col = pow(col, vec3<f32>(0.9));

    col = col + snowAmount * snowColor * 0.8;
    col = col + gradient * vec3<f32>(0.4, 0.8, 1.0) * 0.1;
    col = col + snowNoise(uv) * 0.01;
    col = col * (1.0 - length(uv) * params.bound);
    col = mix(col, col * vec3<f32>(params.gamma, params.blue, params.a), 0.3);
    col = applyGamma(col, params.aa);

    return vec4<f32>(col, 1.0);
}