const PI: f32 = 3.141592653589793;
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
fn draw_gabor_patch(center: vec2<f32>, lambda: f32, psi: f32, sigma: f32, gamma: f32, contrast: f32, spatial_freq: f32, uv: vec2<f32>) -> vec3<f32> {
    let theta: f32 = rand(center) * 2.0 * PI;
    
    let uv_offset: vec2<f32> = 2.0 * (uv - center);
    let xp: f32 = uv_offset.x * cos(theta) - uv_offset.y * sin(theta);
    let yp: f32 = uv_offset.x * sin(theta) + uv_offset.y * cos(theta);
    let envelope: f32 = exp(-((xp * xp) + (gamma * gamma * yp * yp)) / (2.0 * sigma * sigma));
    let carrier: f32 = cos(2.0 * PI * xp / (lambda * spatial_freq) + u_time.time * 5.5);
    let gabor: f32 = envelope * carrier;

    return contrast * vec3<f32>(gabor);
}
fn draw_mandelbrot_gabors(uv: vec2<f32>) -> vec3<f32> {
    var colResult: vec3<f32> = vec3<f32>(0.0);
    let maxIter: i32 = 15;
    for (var x: f32 = -2.0; x < 0.5; x += 0.1) {
        for (var y: f32 = -1.0; y < 1.0; y += 0.1) {
            let c: vec2<f32> = vec2<f32>(x, y);
            var z: vec2<f32> = vec2<f32>(0.0);
            var isInSet: bool = true;
            for (var i: i32 = 0; i < maxIter; i = i + 1) {
                z = vec2<f32>(z.x * z.x - z.y * z.y + c.x, 2.0 * z.x * z.y + c.y);
                if (length(z) > 2.0) {
                    isInSet = false;
                    break;
                }
            }
            if (isInSet) {
                let patchColor: vec3<f32> = draw_gabor_patch(c, 0.1, 0.0, 0.1, 1.0, 0.5, 1.0, uv);
                colResult += patchColor;
            }
        }
    }
    return colResult;
}

@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let iResolution: vec2<f32> = vec2<f32>(640.0, 480.0);
    let uv: vec2<f32> = 3.0*(FragCoord.xy - 0.5 * iResolution) / min(iResolution.x, iResolution.y);
    let colBackground: vec3<f32> = vec3<f32>(0.1);
    let colMandelbrot: vec3<f32> = draw_mandelbrot_gabors(uv + vec2<f32>(-0.9, 0.1));
    let finalColor: vec3<f32> = colBackground + colMandelbrot;
    return vec4<f32>(finalColor, 1.0);
}