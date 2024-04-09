const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn rodrigues(p: vec3<f32>, ax: vec3<f32>, ro: f32) -> vec3<f32> {
    return mix(dot(p, ax) * ax, p, cos(ro)) + sin(ro) * cross(ax, p);
}
struct Params {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma: f32,
    blue:f32,
};
@group(0) @binding(1)
var<uniform> params: Params;
// 3D Voronoi
fn voronoi3d(x: vec3<f32>) -> vec3<f32> {
    var p: vec3<f32> = floor(x);
    var f: vec3<f32> = fract(x);
    var id: f32 = 0.0;
    var res: vec2<f32> = vec2<f32>(2.0);
    for (var k: i32 = -1; k <= 1; k++) {
        for (var j: i32 = -1; j <= 1; j++) {
            for (var i: i32 = -1; i <= 1; i++) {
                var b: vec3<f32> = vec3<f32>(f32(i), f32(j), f32(k));
                var r: vec3<f32> = b - f;
                var d: f32 = dot(r, r);

                var cond: f32 = max(sign(res.x - d), 0.0);
                var nCond: f32 = 1.0 - cond;

                var cond2: f32 = nCond * max(sign(res.y - d), 0.0);
                var nCond2: f32 = 1.0 - cond2;

                id = (dot(p + b, vec3<f32>(1.0, 57.0, 113.0)) * cond) + (id * nCond);
                res = vec2<f32>(d, res.x) * cond + res * nCond;

                res.y = cond2 * d + nCond2 * res.y;
            }
        }
    }
    return vec3<f32>(sqrt(res), abs(id));
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    var p: vec2<f32> = (FragCoord.xy - 0.5 * resolution.xy) / resolution.y;
    let dist_squared: f32 = dot(p, p);
    var S: f32 = 15.0;
    var a: f32;
    var n: vec2<f32>;
    var q: vec2<f32>;
    var rotatedP: vec3<f32>;
    for(var j: f32 = 1.0; j < params.lambda; j = j + 1.0){
        rotatedP = rodrigues(vec3<f32>(p, 0.0), vec3<f32>(0.0, 0.0, 1.0), 5.0 + sin(u_time.time) * 0.01);
        p = rotatedP.xy;
        n = rodrigues(vec3<f32>(n, 0.0), vec3<f32>(0.0, 0.0, 1.0), 5.0 + sin(u_time.time) * 0.01).xy;
        q = p * S + u_time.time * params.theta + sin(u_time.time * params.theta  - dist_squared * 0.0) * 2.0 + j + n;
        a += dot(cos(q) / S, vec2<f32>(params.gamma));
        n -= sin(q);
        S *= params.sigma;
    }
    let result: f32 = params.blue * ((a + params.alpha) + a + a); 
    var uv: vec2<f32> = FragCoord.xy / resolution.xy;
    var rotatedUv: vec3<f32> = rodrigues(vec3<f32>(uv, 0.0), vec3<f32>(0.0, 0.0, 1.0), u_time.time * 0.1);
    var voronoi: vec3<f32> = voronoi3d(vec3<f32>(u_time.time - 0.0, uv));
    var final_: f32 = pow(voronoi.x * 0.1, result * 30.0) * 1.5;
    var phaseR: f32 = (cos(uv.x * 0.1 + u_time.time * 10.0) * 0.5 + 1.5) * 0.5;
    var phaseG: f32 = (cos(uv.x * 0.8 + u_time.time * 10.0 + PI / 3.0) * 0.5 + 2.5) * 0.5;
    var phaseB: f32 = (cos(uv.x * 0.5 + u_time.time * 10.0 + 2.0 * PI / 3.0) * 0.5 + 0.5) * 0.5;
    var wave: f32 = sin(5.0 * u_time.time + rotatedUv.x * PI);
    var neuronColor: vec3<f32> = vec3<f32>(
        0.6 + 0.5 * sin(2.0 * u_time.time + wave),
        0.6 + 0.5 * sin(2.0 * u_time.time + 2.0 * PI / 3.0 + 2.0 * wave),
        0.6 + 0.5 * sin(2.0 * u_time.time + 4.0 * PI / 3.0 + 2.0 * wave)
    );
    return vec4<f32>(
        final_ * neuronColor + vec3<f32>(phaseR, phaseG, phaseB) * final_ * 0.1, 
        1.0);
}
