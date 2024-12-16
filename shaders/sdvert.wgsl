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

const PI: f32 = 3.14159265358979323846;
const LI_INT: f32 = 1.2; 
const RIM_POW: f32 = 1.0;  
const AO_STR: f32 = 0.05;  
const ENV_STR: f32 = 0.4; 
const IRI_POW: f32 = 0.15;
const FALL_DIST: f32 = 2.5;  
const VIG_STR: f32 = 0.25;
fn rot2d(angle: f32) -> mat2x2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return mat2x2<f32>(c, -s, s, c);
}

fn pentp(angle: f32, radius: f32) -> vec2<f32> {
    return vec2<f32>(cos(angle), sin(angle)) * radius;
}
fn osc(mV: f32, MV: f32, i: f32, t: f32) -> f32 {
    return mV + (MV - mV) * 0.5 * (sin(2.0 * PI * t / i) + 1.0);
}
fn sdPentagon(p: vec2<f32>, r: f32, angle: f32) -> f32 {
    var rotP = p * rot2d(-angle);
    
    let vertices = params.lambda;
    let angleStep = params.theta * PI / vertices;
    
    var d = length(rotP) - r;
    
    for(var i = 0.0; i < vertices; i += 1.0) {
        let a1 = angleStep * i;
        let a2 = angleStep * (i + 1.0);
        
        let p1 = pentp(a1, r);
        let p2 = pentp(a2, r);
        
        let edge = p2 - p1;
        let normal = normalize(vec2<f32>(edge.y, -edge.x));
        let dist = dot(rotP - p1, normal);
        
        d = max(d, dist);
    }
    
    return max(d, 0.1);
}

fn pent(uv: vec2<f32>, size: f32, angle: f32) -> array<vec2<f32>, 5> {
    var vertices: array<vec2<f32>, 5>;
    let r = size * 0.8;
    
    for(var i = 0; i < 5; i++) {
        let a = (f32(i) / 5.0) * 2.0 * PI;
        vertices[i] = pentp(a, r);
    }
    
    let rotation = rot2d(angle);
    for(var i = 0; i < 5; i++) {
        vertices[i] = rotation * vertices[i] + uv;
    }
    
    return vertices;
}

fn lighti(uv: vec2<f32>, pentagon: f32, layer: f32, time: f32, angle: f32) -> f32 {
    let vertices = pent(uv, layer, angle);
    
    let ps1 = sin(layer * 13.37 + time * 0.3);
    let ps2 = cos(layer * 7.54 - time * 0.4);
    let ps3 = sin(layer * 9.21 + time * 0.5);
    let ps4 = cos(layer * 11.13 + time * 0.6);
    let ps5 = sin(layer * 8.45 + time * 0.7);
    
    var ligp: array<vec2<f32>, 5>;
    ligp[0] = vertices[0] + vec2<f32>(cos(time * 0.5 + ps1), sin(time * 0.7 + ps2)) * 0.3;
    ligp[1] = vertices[1] + vec2<f32>(sin(time * 0.3 + ps2), cos(time * 0.4 + ps3)) * 0.3;
    ligp[2] = vertices[2] + vec2<f32>(cos(time * 0.6 + ps3), sin(time * 0.5 + ps4)) * 0.3;
    ligp[3] = vertices[3] + vec2<f32>(sin(time * 0.4 + ps4), cos(time * 0.6 + ps5)) * 0.3;
    ligp[4] = vertices[4] + vec2<f32>(cos(time * 0.5 + ps5), sin(time * 0.3 + ps1)) * 0.3;
    
    var d: array<f32, 5>;
    var f: array<f32, 5>;
    var w: array<f32, 5>;
    
    var tw = 0.0;
    
    for(var i = 0; i < 5; i++) {
        d[i] = length(uv - ligp[i]) * 
            (1.0 + 0.2 * sin(layer * (15.0 + f32(i)) + time * (0.7 + f32(i) * 0.1)));
            
        f[i] = 2.0 / (4.0 + d[i] * FALL_DIST);
        w[i] = 0.2 + 0.1 * sin(layer * (11.0 + f32(i) * 2.0) + time * (0.5 + f32(i) * 0.1));
        tw += w[i];
    }
    
    for(var i = 0; i < 5; i++) {
        w[i] /= tw;
    }
    
    let ao = 1.4 - (layer * AO_STR) * (1.0 + 0.2 * sin(layer * 20.0 + time));
    
    let normal = normalize(vec2<f32>(cos(angle + layer), sin(angle + layer)));
    var rim = 1.1 - abs(dot(normalize(uv), normal));
    rim = pow(rim, RIM_POW);
    
    var vl = 0.0;
    for(var i = 0; i < 5; i++) {
        vl += f[i] * w[i];
    }
    
    let shimmer = sin(layer * 10.0 + time) * cos(layer * 7.0 - time);
    vl *= (1.0 + 0.15 * shimmer);
    
    return vl * ao * LI_INT + rim * 0.4;
}

fn evl(uv: vec2<f32>, angle: f32, layer: f32, time: f32) -> f32 {
    let ld = normalize(vec2<f32>(cos(time), sin(time)));
    let normal = normalize(vec2<f32>(cos(angle), sin(angle)));
    var el = dot(normal, ld);
    el = el * 0.5 + 0.5;
    
    let depth = 1.0 - (layer / 1.5);
    let layeref = sin(layer * 4.0 + time) * 0.5 + 0.5;
    
    return mix(el, layeref, 0.5) * depth * ENV_STR;
}

@fragment
fn main(@builtin(position) fc: vec4<f32>) -> @location(0) vec4<f32> {
    let bg = osc(0.6, 0.6, 8.0, u_time.time);
    var fragColor = vec4<f32>(bg);
    var hue: vec4<f32>;
    
    let screen_size = 0.8 * vec2<f32>(1920.0, 1080.0);
    
    let t = u_time.time * 0.5;
    var angle = 0.25;
    let fp = cos(t * 0.5) * PI * 0.25;
    let global = osc(0.4, 1.5, 8.0, u_time.time);
    let asd = osc(params.alpha, params.sigma, 25.0, u_time.time);

    var i = params.gamma;
    while(i > 0.003) {
        let layer = i * 1.0;
        
        let fold = sin(t + layer * 0.2) * cos(t * 0.5 + layer * 0.1);
        let wave = cos(t * 0.7 + layer * 0.15) * sin(t * 0.3 + i);
        
        let temp_angle = fp;
        angle -= sin(angle - sin(temp_angle)) * (0.5 + 1.5 * sin(layer));
        
        let newf = sign(sin(layer * params.b)) * sin(t + i * 2.0);
        
        var uv = 2.7 * (fc.xy + fc.xy - screen_size) / screen_size.y;
        uv.y -= 0.3;
        uv.x -= 0.3;
        uv *= rot2d(i + (angle + newf) + fold);
        
        let pentagon = sdPentagon(uv, i, angle + t * (1.0 + 0.2 * sin(layer)));
        let alpha = smoothstep(0.0, 0.2, (pentagon - 0.1) * screen_size.y * 0.15);
        
        let lpawaa = lighti(uv, pentagon, i, t, angle);
        let ell = evl(uv, angle, i, t);
        
        let des = osc(0.5, 1.0, 5.0, u_time.time);
        let des2 = osc(0.1, 1.0, 5.0, u_time.time);
        
        let intensecol = params.d + 
            0.2 * sin(layer * 1.37 + t) * 
            cos(layer * 12.54 - t * 0.4) * 
            sin(angle * 3.0 + t * 0.7);
            
        let shift = params.c + 
            0.1 * cos(layer * 9.21 + t * 0.5) * 
            sin(angle * 5.0 - t * 0.3);
            
        hue = sin(i / des2 + angle / des + vec4<f32>(params.blue, 2.0, 1.0, 1.0) + fold * 1.5) * shift + intensecol;
        
        let litColor = hue * (lpawaa * global + ell);
        
        let dff = params.a - (i - 0.003) / (1.5 - 0.003);
        let iri = sin(dot(uv, uv) * 4.0 + t) * IRI_POW * dff + 0.95;
        var wc = litColor * vec4<f32>(iri, iri * 0.98, iri * 1.02, 1.0);
        
        let des4 = osc(0.2, 0.2, 10.0, u_time.time);
        let des12 = osc(0.1, 0.1, 10.0, u_time.time);

        let mixFactor = des4 / (pentagon + 0.02) * (1.0 - dff * 0.25);
        fragColor = mix(wc, fragColor, alpha) * 
                   mix(vec4<f32>(1.0), hue + 0.5 * alpha * des12 * (uv.x / pentagon + lpawaa), mixFactor);
                   
        i -= asd;
    }

    let des2 = osc(0.7, 0.7, 5.0, u_time.time);
    fragColor = vec4<f32>(fragColor.rgb * (des2 + global * params.aa), 1.0);
    
    let vuv = (fc.xy - 0.5 * vec2<f32>(1920.0, 1080.0)) / 1080.0;
    let vignette = 1.0 - dot(vuv, vuv) * VIG_STR;

    return vec4<f32>(fragColor.rgb * vignette, 1.0);
}

