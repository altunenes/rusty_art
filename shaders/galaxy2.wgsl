const PI: f32 = 3.1415926535897932384626433832795;

struct TimeUniform {
    time: f32,
};

struct Params {
    lambda: f32,
    theta: f32,
    sigma: f32,
    gamma: f32,
    alpha: f32,
    delta: f32,
    eta: f32,
    rho: f32,
    phi: f32,
    psi: f32,
    omega: f32,
    blue: f32,
    noise: f32,
    noise2: f32,
    color: f32,
    fw: f32,
    fh: f32,
    fcx: f32,
    fcy: f32,
    red: f32,
    green: f32,
    blue2: f32,
};

@group(0) @binding(1)
var<uniform> params: Params;

@group(1) @binding(0)
var<uniform> u_time: TimeUniform;

const LIGHT_DIRECTION: vec3<f32> = vec3<f32>(0.57735, 0.57735, 0.57735);

fn applyGamma(color: vec3<f32>, gamma: f32) -> vec3<f32> {
    return pow(color, vec3<f32>(1.0 / gamma));
}

fn oscillate(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}

fn fbm_slow(p: vec2<f32>) -> f32 {
    var f = 0.0;
    var q = p;
    f += 1.001 * length(q); q *= 2.02;
    f += 2.001 * length(q); q *= 2.03;
    f += 0.1250 * length(q); q *= 0.001;
    f += 0.0625 * length(q);
    return f / 2.9375;
}

fn noise(p: vec2<f32>) -> f32 {
    return fbm_slow(p + u_time.time * params.gamma);
}

fn hash3(p: vec3<f32>) -> vec3<f32> {
    var q = fract(p * vec3<f32>(443.8975, 397.2973, 491.1871));
    q += dot(q.zxy, q.yxz + 19.19);
    return fract((q.xxy + q.yxx) * q.zyx);
}

fn voronoi3d(x: vec3<f32>) -> vec3<f32> {
    let p = floor(x);
    let f = fract(x);

    var id = 0.0;
    var res = vec2<f32>(100.0);
    
    for (var k: i32 = -1; k <= 1; k++) {
        for (var j: i32 = -1; j <= 1; j++) {
            for (var i: i32 = -1; i <= 1; i++) {
                let b = vec3<f32>(f32(i), f32(j), f32(k));
                let r = b - f + hash3(p + b);
                let d = dot(r, r);

                if (d < res.x) {
                    id = dot(p + b, vec3<f32>(1.0, 57.0, 113.0));
                    res.y = res.x;
                    res.x = d;
                } else if (d < res.y) {
                    res.y = d;
                }
            }
        }
    }

    return vec3<f32>(sqrt(res), abs(id));
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

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    var uv = (FragCoord.xy / resolution) - 0.5;
    let distanceFromCenter = length(uv) * params.alpha;
    let flow = vec2<f32>(noise(uv), noise(uv + vec2<f32>(0.1)));

    let timeFactor = params.theta * u_time.time;
    let adjustedTime = timeFactor + (params.delta + sin(timeFactor)) * 0.1 / (distanceFromCenter + 0.07);
    let sineTime = sin(adjustedTime);
    let cosineTime = cos(adjustedTime);
    uv = mat2x2<f32>(cosineTime, sineTime, -sineTime, cosineTime) * uv;
    uv += flow * params.lambda;
    uv *= mat2x2<f32>(cos(u_time.time * 0.5), sin(u_time.time * 0.5), -sin(u_time.time * 0.5), cos(u_time.time * 0.5)) * params.fcx * 25.662;

    var baseColor = 0.0;
    var color1 = params.noise;
    var color2 = params.noise2;
    var color3 = 0.0;
    var point: vec3<f32>;
    let scramb14 = oscWithPause(params.psi, params.psi + 0.0058, 10.0, 8.0, u_time.time);

    for (var i: i32 = 0; i < 100; i++) {
        point = params.eta * f32(i) * vec3<f32>(uv, params.fh);
        point += vec3<f32>(0.1, 0.01, -params.fw - sin(timeFactor * 0.1) * 0.01);

        for (var j: i32 = 0; j < 11; j++) {
            point = abs(point) / dot(point, point) - scramb14;
        }

        let pulse123 = mix(params.omega, params.omega + 0.001, 0.5 + 0.5 * sin(u_time.time));
        let pointIntensity = dot(point, point) * pulse123;
        color1 += pointIntensity * (params.rho + sin(distanceFromCenter * 13.0 + 3.5 - timeFactor * 25.0));
        color2 += pointIntensity * (params.phi + sin(distanceFromCenter * 13.5 + 2.2 - timeFactor * 3.0));
        color3 += pointIntensity * (24.4 + sin(distanceFromCenter * 14.5 + 1.5 - timeFactor * 2.5));
    }

    let scramb2345222 = oscWithPause(1.2, 4.2, 5.0, 5.0, u_time.time);
    let scradd = oscWithPause(0.1, 1.8, 5.0, 5.0, u_time.time);
    let pulse2 = mix(5.25, 3.45, 0.5 + 0.5 * sin(u_time.time));

    let voronoiPoint = voronoi3d(point + vec3<f32>(u_time.time * params.gamma));
    let voronoiIntensity = voronoiPoint.x * pulse2;
    let pulse123 = mix(0.8, 0.1, 0.5 + 0.5 * sin(u_time.time));

    color1 += voronoiIntensity * scramb2345222;
    color2 += voronoiIntensity * pulse123;
    color3 += voronoiIntensity * scradd;

    let colorFlow = vec3<f32>(sin(voronoiPoint.x), sin(voronoiPoint.y), sin(voronoiPoint.z + u_time.time * 0.5));
    color1 *= colorFlow.r * params.color;
    color2 *= colorFlow.g * params.color;
    color3 *= colorFlow.b * params.color;

    let pulse = mix(0.3, 0.1, 0.5 + 0.5 * sin(u_time.time));
    let pulse3 = mix(0.2, 0.1, 0.5 + 0.5 * sin(u_time.time));
    let pulse34 = mix(1.3, 4.1, 0.5 + 0.5 * sin(u_time.time));
    let pulse34555 = mix(0.3, 0.32, 0.5 + 0.5 * sin(u_time.time));
    let pulse34555444 = mix(0.12, 0.66, 0.5 + 0.5 * sin(u_time.time));

    baseColor = (3.1 / (pulse34 + params.blue)) * length(point.xy) * params.sigma;
    color1 = color1 * pulse;
    color2 = color2 * pulse3;
    color3 = smoothstep(params.fcy, 0.0, distanceFromCenter);
    color3 = color3 * pulse34555;

    let direction = normalize(vec3<f32>(uv, 0.0));
    let sundot = dot(vec3<f32>(0.57735), direction);

    var finalColor = vec3<f32>(baseColor, (color1 + baseColor) * 0.19, color2);
    finalColor = finalColor + color3 * 2.9;
    finalColor.r += color3 * params.red;
    finalColor.g += color3 * params.green;
    finalColor.b += color3 * params.blue2;

    finalColor = applyGamma(finalColor, 0.5);
    return vec4<f32>(finalColor, 1.0);
}