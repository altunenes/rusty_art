const PI: f32 = 3.14159265;
const MAX_ITER: i32 = 100;
const BOUND: f32 = 3.5;
const AA: i32 = 3;
// TimeUniform definition based on my wgpu rust code
struct TimeUniform {
    time: f32,
};
@group(1) @binding(0)
var<uniform> u_time: TimeUniform;
// Oscillation function (for zooming...)
fn osc(minValue: f32, maxValue: f32, interval: f32, currentTime: f32) -> f32 {
    return minValue + (maxValue - minValue) * 0.5 * (sin(2.0 * PI * currentTime / interval) + 1.0);
}
// Time remapping function
fn remapTime(currentTime: f32, startInterval: f32, endInterval: f32, newDuration: f32) -> f32 {
    if (currentTime < startInterval) {
        return currentTime;
    } else if (currentTime >= startInterval && currentTime <= endInterval) {
        let normalizedTime = (currentTime - startInterval) / (endInterval - startInterval);
        return startInterval + normalizedTime * newDuration;
    } else {
        return currentTime + newDuration - (endInterval - startInterval);
    }
}
// Implicit function for fractal calculations
fn implicit(c: vec2<f32>) -> vec2<f32> {
    var z: vec2<f32> = vec2<f32>(0.0, 0.0);
    var i: i32 = 0;
    loop {
        if (i >= MAX_ITER) { break; }
        let xnew: f32 = z.x * z.x - z.y * z.y + c.x;
        z.y = 2.0 * z.x * z.y + c.y;
        z.x = xnew;
        let dampenedTime: f32 = u_time.time * 0.001; 
        z += 0.1 * vec2<f32>(sin(0.001 * dampenedTime), cos(0.001 * dampenedTime));
        if (dot(z, z) > BOUND / 1.2) { break; }
        i = i + 1;
        continue;
    }
    return vec2<f32>(f32(i), dot(z, z));
}
@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let resolution: vec2<f32> = vec2<f32>(800.0, 450.0); // Adjusted resolution
    var col: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0); // Initial color
    // Camera and panning adjustments
    let camSpeed: vec2<f32> = vec2<f32>(0.0002, 0.0002);
    let camPath: vec2<f32> = vec2<f32>(sin(camSpeed.x * u_time.time), cos(camSpeed.y * u_time.time));
    var pan: vec2<f32> = vec2<f32>(0.8030, 0.2579); // Starting pan position
    if (u_time.time > 14.0) {
        let timeSince14: f32 = u_time.time - 14.0;
        pan.y += 0.0002 * timeSince14;
    }
    let scramb: f32 = osc(0.05, 0.0005, 20.0, u_time.time); 
    let zoomLevel: f32 = scramb;
    
    // a constant color for the exterior
    let exteriorColor: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0); // Example: Deep blue color
    
    for (var m: i32 = 0; m < AA; m++) {
        for (var n: i32 = 0; n < AA; n++) {
            let uv: vec2<f32> = ((FragCoord.xy + vec2<f32>(f32(m), f32(n)) / f32(AA) - 0.5 * resolution) / min(resolution.y, resolution.x)
            * zoomLevel + pan + camPath) * 2.033 - vec2<f32>(2.14278);
            let z_and_i: vec2<f32> = implicit(uv);
            let iter_ratio: f32 = z_and_i.x / f32(MAX_ITER);
            let lenSq: f32 = z_and_i.y;
            
            if (iter_ratio >= 1.0) {
                let c1: f32 = pow(clamp(2.00 * sqrt(lenSq), 0.0, 1.0), 0.5);
                let col1: vec3<f32> = 0.5 + 0.5 * sin(1.0 + vec3<f32>(0.0, 0.5, 1.0) + PI * vec3<f32>(2.0 * lenSq) + u_time.time);
                let col2: vec3<f32> = 0.5 + 0.5 * sin(2.1 + PI * vec3<f32>(lenSq) + u_time.time);
                col += 1.5 * sqrt(c1 * col1 * col2);
            } else {
                col += exteriorColor;
            }
        }
    }
    
    col /= f32(AA * AA);
    return vec4<f32>(col, 1.0);
}