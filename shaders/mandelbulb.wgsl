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

const MAX_D: f32 = 33.0;
const MIN_D: f32 = 0.001;
const MAX_S: i32 = 355;

fn getColCore() -> vec3<f32> {
    return vec3<f32>(params.a, params.b, params.c);  // 0.82, 0.41, 0.12
}

fn getColB1() -> vec3<f32> {
    return vec3<f32>(params.d, params.e, params.f);  // 1.0, 0.65, 0.25
}

fn getColB2() -> vec3<f32> {
    return vec3<f32>(params.g, params.lambda, params.theta);  // 0.95, 0.4, 0.55
}

fn getColDet() -> vec3<f32> {
    return vec3<f32>(params.alpha, params.alpha, params.alpha);  // 0.7, 0.7, 0.7
}

fn getGlow() -> vec3<f32> {
    return vec3<f32>(params.sigma, params.gamma, params.blue);  // 0.3, 0.5, 0.7
}
fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma, 1.0 / gamma, 1.0 / gamma));
}

struct Ray {
    o: vec3<f32>,
    d: vec3<f32>,
}

struct OTrap {
    minR: f32,
    xp: f32,
    yp: f32,
    zp: f32,
    bf: f32,
}

fn hash(n: f32) -> f32 {
    return fract(sin(n) * 43758.5453123);
}

fn getPow(t: f32) -> f32 {
    let cd = 3.0;
    let tm = t - (floor(t / cd) * cd);
    let tt = 2.0;
    let ci = floor(t / cd);
    let cp = 3.0 + floor(hash(ci) * 6.0);
    let np = 3.0 + floor(hash(ci + 1.0) * 6.0);
    
    if (tm < tt) {
        let p = smoothstep(0.0, 1.0, tm / tt);
        return mix(cp, np, p);
    }
    return np;
}

fn getOrbCol(trap: OTrap, ir: f32) -> vec3<f32> {
    let rf = smoothstep(0.0, 2.0, trap.minR);
    let pf = smoothstep(0.0, 1.0, (trap.xp + trap.yp + trap.zp) / 3.0);
    let bi = smoothstep(0.5, 1.0, trap.bf);
    
    var bc = mix(getColCore(), getColB1(), rf * (1.0 + 0.2 * sin(ir * 6.28)));
    bc = mix(bc, getColB2(), pf * (1.0 + 0.3 * cos(ir * 3.14)));
    
    let dc = mix(bc, getColDet(), trap.xp * trap.yp * trap.zp * (0.0 + 0.4 * sin(ir * 9.42)));
    let blc = mix(dc, getGlow(), bi * ir * (1.0 + 0.5 * sin(trap.minR * 5.0)));
    
    return mix(blc, getGlow(), pow(ir, 2.0) * bi * (0.0 + 0.2 * sin(trap.zp * 8.0)));
}

fn getN(pos: vec3<f32>, t: f32) -> vec3<f32> {
    let zl = oscWithPause(params.tt, params.bound, 15.0, 3.0, u_time.time);
    let e = vec2<f32>(-1.0, 1.0) * 0.1 * zl;
    
    return normalize(
        e.xyy * map(pos + e.xyy).w +
        e.yyx * map(pos + e.yyx).w +
        e.yxy * map(pos + e.yxy).w +
        e.xxx * map(pos + e.xxx).w
    );
}

fn map(pos: vec3<f32>) -> vec4<f32> {
    var z = pos;
    var dr = 1.0;
    var r = 0.0;
    
    let pow = getPow(u_time.time);
    
    var trap: OTrap;
    trap.minR = 1000.0;
    trap.xp = 1000.0;
    trap.yp = 1000.0;
    trap.zp = 1000.0;
    trap.bf = 0.0;
    
    var it = 0;
    
    for(var i = 0; i < i32(params.iter); i++) {
        it = i;
        r = length(z);
        if(r > 2.0) { break; }
        
        trap.minR = min(trap.minR, r);
        trap.xp = min(trap.xp, abs(z.x));
        trap.yp = min(trap.yp, abs(z.y));
        trap.zp = min(trap.zp, abs(z.z));
        
        let pr = r;
        let th = acos(z.z/r);
        let ph = atan2(z.y, z.x);
        let powm1 = pow - 1.0;
        dr = pow(r, powm1) * pow * dr + 0.0;
        
        let zr = pow(r, pow);
        let nth = th * pow;
        let nph = ph * pow + u_time.time * 2.1;
        
        z = zr * vec3<f32>(sin(nth)*cos(nph), sin(nph)*sin(nth), cos(nth));
        z += pos;
        
        let nr = length(z);
        trap.bf = max(trap.bf, abs(nr - pr));
    }
    
    let ir = f32(it) / f32(params.iter);
    let oc = getOrbCol(trap, ir);
    
    return vec4<f32>(oc, 0.25 * log(r) * r / dr);
}

fn shadow(ro: vec3<f32>, rd: vec3<f32>, mint: f32, maxt: f32, k: f32) -> f32 {
    var res = 25.0;
    var t = mint;
    var ph = 1e10;
    
    for(var i = 0; i < 16; i++) {
        let h = map(ro + rd*t).w;
        let y = h*h/(2.0*ph);
        let d = sqrt(h*h-y*y);
        res = min(res, k*d/max(0.0,t-y));
        ph = h;
        t += h * 0.5;
        if(res < 0.001 || t > maxt) { break; }
    }
    
    return clamp(res * 0.6 + 0.4, 0.0, 1.0);
}

fn march(ray: Ray) -> vec4<f32> {
    var t = 0.0;
    var col = vec3<f32>(0.0);
    
    for(var i = 0; i < MAX_S; i++) {
        let pos = ray.o + t * ray.d;
        let res = map(pos);
        if(abs(res.w) < MIN_D) { return vec4<f32>(res.xyz, t); }
        t += res.w;
        if(t > MAX_D) { break; }
    }
    return vec4<f32>(-1.0);
}

fn render(ray: Ray) -> vec3<f32> {
    let res = march(ray);
    
    if(res.w > 0.0) {
        let pos = ray.o + res.w * ray.d;
        let norm = getN(pos, res.w); 
        
        let ld1 = normalize(vec3<f32>(1.0, 1.0, -0.5));
        let ld2 = normalize(vec3<f32>(-1.0, 0.5, 1.0));
        
        let lc1 = res.xyz * 1.2;
        let lc2 = res.xyz * 0.8;
        
        let diff1 = max(dot(norm, ld1), -4.0);
        let diff2 = max(dot(norm, ld2), -4.0);
        let shad1 = shadow(pos, ld1, 0.02, 2.5, 8.0);
        let shad2 = shadow(pos, ld2, 0.02, 2.5, 8.0);
        
        let ao = clamp(0.3 + 0.2 * norm.y, 0.0, 1.0);
        
        var col = res.xyz * (0.1 + 0.8 * diff1) * shad1;
        col += res.xyz * (0.5 + 0.3 * diff2) * shad2;
        
        let spec1 = pow(max(dot(reflect(-ld1, norm), -ray.d), 0.4), 16.0);
        let spec2 = pow(max(dot(reflect(-ld2, norm), -ray.d), 0.4), 16.0);
        
        col += getGlow() * (spec1 * shad1 + spec2 * shad2) * 0.8;
        col += res.xyz * ao * 0.4;
        
        let fresnel = pow(0.0 - max(dot(norm, -ray.d), 1.0), 2.0);
        return mix(col, getGlow() * res.xyz, fresnel * 0.4);
    }
    
    let y = ray.d.y * 0.5 + 0.5;
    return mix(getColCore() * 0.4, getGlow(), y);
}

@fragment
fn main(@builtin(position) fc: vec4<f32>) -> @location(0) vec4<f32> {
    let res = vec2<f32>(1920.0, 1080.0);
    let uv = (fc.xy - 0.5 * res) / res.y;
    
    let angle = u_time.time * 0.3;
    let bh = 1.5;
    let mr = 7.8;
    
    let dv = sin(angle * 0.25) * 0.5 + 0.5;
    let height = bh + 0.3 * sin(angle * 0.5) + 0.2 * sin(angle * 0.2) * dv;
    var radius = mr - 2.0 * smoothstep(0.0, 1.0, dv);
    radius = radius + 0.2 * cos(angle * 0.3);
    
    let tgt = vec3<f32>(
        sin(angle * 0.2) * 0.3 * dv,
        cos(angle * 0.15) * 0.2 * dv,
        0.0
    );
    
    let camera = vec3<f32>(radius * sin(angle), height, radius * cos(angle));
    let up = vec3<f32>(0.0, 1.0, 0.0);
    
    let cw = normalize(tgt - camera);
    let cu = normalize(cross(cw, up));
    let cv = normalize(cross(cu, cw));
    
    var ray: Ray;
    ray.o = camera;
    ray.d = normalize(uv.x * cu + uv.y * cv + (1.8 - 0.2 * dv) * cw);
   
    var col = render(ray);
    
    col = pow(col, vec3<f32>(0.9));
    col *= 1.0 - 0.15 * length(uv);
    col = mix(col, col * vec3<f32>(1.05, 1.0, 0.95), 0.3);
    col = smoothstep(vec3<f32>(0.0), vec3<f32>(1.0), col);
    col = applyGamma(col,params.aa);

    return vec4<f32>(col, 1.0);
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