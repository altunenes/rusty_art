@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

struct TimeUniform {
    time: f32,
};

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

const PI: f32 = 3.14;

struct Params {
    lambda: f32,
    theta: f32,
    alpha: f32,
    sigma: f32,
    gamma: f32,
    blue: f32,
};

@group(2) @binding(2)
var<uniform> params: Params;

fn Hash11(n: f32) -> f32 {
    return fract(sin(n) * 12.5453123);
}

fn osc2(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn StaticPoint(cell: vec2<f32>) -> vec2<f32> {
    let freq = Hash11(dot(cell, vec2<f32>(12.9898, 4.1414))) * 2.0 + 1.0;
    let phase = Hash11(dot(cell, vec2<f32>(63.7264, 10.873)));
    let amp = 0.5 + 0.4 * Hash11(dot(cell, vec2<f32>(305.21, 532.83)));
    return vec2<f32>(cos(phase * 6.2831), sin(phase * 6.2831)) * amp;
}

fn points(cell: vec2<f32>, time: f32) -> vec2<f32> {
    let freq = Hash11(dot(cell, vec2<f32>(12.9898, 4.1414))) * params.gamma +params.gamma;
    let phase = Hash11(dot(cell, vec2<f32>(63.7264, 10.873)));
    let amp = 0.5 + 0.4 * Hash11(dot(cell, vec2<f32>(305.21, 532.83)));
    let t = time * freq + phase * 6.2831;
    return vec2<f32>(cos(t), sin(t)) * amp;
}

fn sminExp(a: f32, b: f32, k: f32) -> f32 {
    let res = exp(-k * a) + exp(-k * b);
    return -log(res) / k;
}

fn borders(minDist: f32, smd: f32) -> f32 {
    let edgeWidth = params.blue;
    let edgeBlend = smoothstep(minDist, minDist + edgeWidth, smd);
    return 1.0 - edgeBlend;
}

fn smoothVoronoi(uv: vec2<f32>, time: f32, border: ptr<function, f32>, smd: ptr<function, f32>, ctp: ptr<function, vec2<f32>>, closepoint: ptr<function, vec2<f32>>) -> f32 {
    let g = floor(uv);
    let f = fract(uv);
    let POWER = osc2(params.alpha, params.alpha, 11.0, time);

    let k = 1.0;
    var minDist = POWER;
    *smd = 1.0;
    *ctp = vec2<f32>(0.0);
    *closepoint = vec2<f32>(0.0);
    for (var y = -2; y <= 2; y = y + 1) {
        for (var x = -2; x <= 2; x = x + 1) {
            let lattice = vec2<f32>(f32(x), f32(y));
            let offset = points(g + lattice, time);
            let staticOffset = StaticPoint(g + lattice);
            let point = lattice + offset - f;
            let staticPoint = lattice + staticOffset - f;
            let dist = dot(point, point);
            if (dist < minDist) {
                *smd = minDist;
                minDist = dist;
                *ctp = g + lattice + offset;
                *closepoint = g + lattice + staticOffset;
            } else if (dist < *smd) {
                *smd = dist;
            }
        }
    }
    *border = borders(minDist, *smd);
    return sqrt(minDist);
}

fn calcNormal(uv: vec2<f32>, time: f32) -> vec3<f32> {
    let eps = 0.001;
    var border: f32 = 0.0;
    var smd: f32 = 0.0;
    var ctp: vec2<f32> = vec2<f32>(0.0);
    var closepoint: vec2<f32> = vec2<f32>(0.0);
    let dist = smoothVoronoi(uv, time, &border, &smd, &ctp, &closepoint);
    let dx = smoothVoronoi(uv + vec2<f32>(eps, 0.0), time, &border, &smd, &ctp, &closepoint) - dist;
    let dy = smoothVoronoi(uv + vec2<f32>(0.0, eps), time, &border, &smd, &ctp, &closepoint) - dist;
    return normalize(vec3<f32>(dx, dy, eps));
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0); 
    let uv: vec2<f32> = params.lambda*(FragCoord.xy / resolution);
    let time = u_time.time;
    var border: f32 = 0.0;
    var smd: f32 = 0.0;
    var ctp: vec2<f32> = vec2<f32>(0.0);
    var closepoint: vec2<f32> = vec2<f32>(0.0);
    let dist = smoothVoronoi(uv, time, &border, &smd, &ctp, &closepoint);

    let textureColor = textureSample(tex, tex_sampler, closepoint.xy / params.lambda);

    let normal = calcNormal(uv, time);
    let lightDir = normalize(vec3<f32>(1.0, 1.0, 1.0));
    let lightIntensity = max(dot(normal, lightDir), params.theta);

    let lighting = mix(vec3<f32>(0.1), vec3<f32>(1.0), lightIntensity);
    var color = textureColor.rgb * lighting;

    color = mix(color, vec3<f32>(1.0, 1.0, 1.0), border * params.sigma);

    return vec4<f32>(color, 1.0);
}
