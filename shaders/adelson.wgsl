struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};

@group(0) @binding(1)
var<uniform> params: Params;
const MAX_STEPS: i32 = 100;
const MAX_DIST: f32 = 100.0;
const SURF_DIST: f32 = 0.001;
const CHECKER_SCALE: f32 = 0.1;
const BOARD_SIZE: f32 = 2.0;
const LIGHT_DIR: vec3<f32> = vec3<f32>(-0.5773503, 0.7071068, -0.2886751); 
const DOT_SIZE: f32 = 0.02;
const DOT_POS1: vec2<f32> = vec2<f32>(-0.25, 0.15);
const DOT_POS2: vec2<f32> = vec2<f32>(0.05, -0.25);
const PI: f32 = 3.14159265359;
fn sdCylinder(p: vec3<f32>) -> f32 {
    var p_adj = p;
    p_adj.x -= -0.6;
    p_adj.z -= -0.6;
    let CYLINDER_RADIUS: f32 = params.sigma;
    let CYLINDER_HEIGHT:f32 = params.theta; 
    let d = abs(vec2<f32>(length(p_adj.xz), p_adj.y)) - vec2<f32>(CYLINDER_RADIUS, CYLINDER_HEIGHT);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0)));
}

fn sdPlane(p: vec3<f32>) -> f32 {
    let q = abs(p.xz);
    if (q.x > BOARD_SIZE || q.y > BOARD_SIZE) {
        return 1000.0;
    }
    return p.y;
}

fn dotss(p: vec2<f32>) -> f32 {
    let d1 = length(p - DOT_POS1);
    let d2 = length(p - DOT_POS2);
    if (min(d1, d2) < DOT_SIZE) {
        return 1.0;
    }
    return 0.0;
}

fn checkerboard(p: vec2<f32>) -> f32 {
    if (abs(p.x) > BOARD_SIZE || abs(p.y) > BOARD_SIZE) {
        return -1.0;
    }
    let grid = floor(p / CHECKER_SCALE);
    return fract((grid.x + grid.y) * 0.5) * 2.0;
}
fn getSceneDistance(p: vec3<f32>) -> vec2<f32> {
    let cylinder = sdCylinder(p);
    let plane = sdPlane(p);
    let id = select(0.0, 1.0, cylinder < plane);
    return vec2<f32>(min(cylinder, plane), id);
}
fn getNormal(p: vec3<f32>) -> vec3<f32> {
    let e = vec2<f32>(0.01, 0.0);
    let n = vec3<f32>(
        getSceneDistance(p + e.xyy).x - getSceneDistance(p - e.xyy).x,
        getSceneDistance(p + e.yxy).x - getSceneDistance(p - e.yxy).x,
        getSceneDistance(p + e.yyx).x - getSceneDistance(p - e.yyx).x
    );
    return normalize(n);
}
fn getShadow(p: vec3<f32>, lightDir: vec3<f32>) -> f32 {
    var res = 1.0;
    var t = 0.1;
    let k = 16.0;
    
    for(var i = 0; i < 32; i++) {
        let h = getSceneDistance(p + lightDir * t).x;
        res = min(res, k * h / t);
        t += h;
        if (res < 0.001 || t > 20.0) {
            break;
        }
    }
    return clamp(res, 0.1, 1.0);
}

fn render(ro: vec3<f32>, rd: vec3<f32>) -> vec3<f32> {
    var col = vec3<f32>(0.1);
    var t = 0.0;
    var res = vec2<f32>(0.0);

    for(var i = 0; i < MAX_STEPS; i++) {
        let p = ro + rd * t;
        res = getSceneDistance(p);
        if (res.x < SURF_DIST || t > MAX_DIST) {
            break;
        }
        t += res.x;
    }
    if (t < MAX_DIST) {
        let p = ro + rd * t;
        let n = getNormal(p);
        let shadow = getShadow(p + n * 0.02, LIGHT_DIR);
        
        if (res.y > 0.5) {
            col = vec3<f32>(0.2, 0.8, 0.2);
        } else {
            let check = checkerboard(p.xz);
            if (check < 0.0) {
                col = vec3<f32>(0.1);
            } else {
                col = mix(vec3<f32>(0.15), vec3<f32>(0.85), check);
                let isDot = dotss(p.xz);
                if (isDot > 0.5) {
                    col = vec3<f32>(0.8, 0.0, 0.0);
                }
            }
        }
        
        let diff = max(dot(n, LIGHT_DIR), 0.0);
        let amb = 0.099;
        
        let isDot = dotss(p.xz);
        if (isDot < 0.5 || res.y > 0.5) {
            col *= amb + (1.0 - amb) * diff * shadow;
            let ao = 1.0 - 0.2 * (1.0 - shadow);
            col *= ao;
        }
    }
    
    return col;
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution = vec2<f32>(1920.0, 1080.0); 
    var uv = (vec2<f32>(FragCoord.x / resolution.x, 1.0 - FragCoord.y / resolution.y) * 2.0 - vec2<f32>(1.0, 1.0));    
    let angle = u_time.time * params.blue* PI / 30.0;
    let camHeight = 2.0;
    let camDist = 1.5;
    let la = clamp(angle, -1.0472, 1.0472);
    let ro = vec3<f32>(
        camDist * sin(angle),
        camHeight / sin(la),
        camDist * cos(angle)
    );
    let ta = vec3<f32>(0.0, 0.5, 0.0);
    let ww = normalize(ta - ro);
    let uu = normalize(cross(ww, vec3<f32>(0.0, 1.0, 0.0)));
    let vv = normalize(cross(uu, ww));
    let rd = normalize(uv.x * uu + uv.y * vv + params.lambda * ww); 
    var col = render(ro, rd);
    col = pow(col, vec3<f32>(params.gamma));

    return vec4<f32>(col, 1.0);
}