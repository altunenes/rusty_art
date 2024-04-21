@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;
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
};
@group(2) @binding(2)
var<uniform> params: Params;

const PI: f32 = 3.1415926535897932384626433832795;


fn applyGabor(uv: vec2<f32>, theta: f32) -> vec3<f32> {
    let lambda: f32 = params.lambda;
    let psi: f32 = params.theta*u_time.time;
    let sigma: f32 = params.alpha;  
    let gamma: f32 = params.sigma;   
    let xp: f32 = uv.x * cos(theta) - uv.y * sin(theta);
    let yp: f32 = uv.x * sin(theta) + uv.y * cos(theta);

    let envelope: f32 = 1.2 * exp(-((xp * xp) + (gamma * gamma * yp * yp)) / (2.0 * sigma * sigma));
    let carrier: f32 = cos(2.0 * PI * xp / lambda + u_time.time *params.gamma);
    
    let gabor: f32 = envelope * carrier;
    return vec3<f32>(gabor, gabor, gabor);
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>, @location(0) tex_coords: vec2<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(1920.0, 1080.0);
    let imgColor: vec4<f32> = textureSample(tex, tex_sampler, tex_coords);

    let uv: vec2<f32> = FragCoord.xy / resolution;
    let filterSize: f32 = 12.0;
    let stepSize: f32 = 1.0; 
    
    var filteredColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    let theta: f32 = PI / params.blue; 
    
    var accum: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    
    for(var x: f32 = -filterSize; x <= filterSize; x += stepSize) {
        for(var y: f32 = -filterSize; y <= filterSize; y += stepSize) {
            let offset: vec2<f32> = vec2<f32>(x, y) / resolution;
            let gaborColor: vec3<f32> = applyGabor(offset, theta);
            let imageColor: vec3<f32> = textureSample(tex, tex_sampler, uv + offset).rgb;
            accum = max(accum, gaborColor * imageColor);
        }
    }
    filteredColor = max(filteredColor, accum);
    
    return vec4<f32>(filteredColor, 1.0);
}