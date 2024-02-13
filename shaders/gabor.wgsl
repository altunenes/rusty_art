@fragment
fn main(@builtin(position) FragCoord: vec4<f32>) -> @location(0) vec4<f32> {
    let PI: f32 = 3.1415926535897932384626433832795;
    let resolution: vec2<f32> = vec2<f32>(512.0, 512.0); // Assuming a 512x512 window for simplicity
    let time: f32 = 1.0; // Static value for demonstration; replace with dynamic time if needed

    // Calculate UV coordinates normalized to [-1, 1]
    var uv: vec2<f32> = (FragCoord.xy / resolution) * 2.0 - vec2<f32>(1.0, 1.0);

    // Parameters for the Gabor function
    let lambda: f32 = 0.1; // wavelength
    let theta: f32 = 0.0; // orientation, static for simplification
    let psi: f32 = time * 5.5; // phase offset
    let sigma: f32 = 0.1; // standard deviation of the Gaussian envelope
    let gamma: f32 = 1.0; // spatial aspect ratio

    // Rotation transformation
    let xp: f32 = uv.x * cos(theta) - uv.y * sin(theta);
    let yp: f32 = uv.x * sin(theta) + uv.y * cos(theta);

    // Gabor function
    let envelope: f32 = exp(-((xp * xp) + (gamma * gamma * yp * yp)) / (2.0 * sigma * sigma));
    let carrier: f32 = cos(2.0 * PI * xp / lambda + psi);
    let gabor: f32 = envelope * carrier;

    let colorModulation: vec3<f32> = vec3<f32>(0.5) + vec3<f32>(0.5) * cos(1.5 * PI * xp / lambda + vec3<f32>(0.0, 2.0, 4.0));
    let col: vec3<f32> = 0.5 + 0.5 * gabor * colorModulation;

    return vec4<f32>(col, 1.0);
}