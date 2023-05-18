use nannou::prelude::*;
struct Model {
    time: f32,
}
fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
fn model(_app: &App) -> Model {
    Model { time: 0.0 }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    model.time = app.time;
}
fn gabor(x: f32, y: f32, kx_ratio: f32, ky_ratio: f32, theta: f32, sigma: f32, width: f32, height: f32) -> f32 {
    let kx = kx_ratio * std::f32::consts::PI / width;
    let ky = ky_ratio * std::f32::consts::PI / height;
    let x_theta = theta.cos() * x + theta.sin() * y;
    let y_theta = -theta.sin() * x + theta.cos() * y;
    (-0.5 * (x_theta.powi(2) / sigma.powi(2) + y_theta.powi(2) / sigma.powi(2))).exp()
        * (2.0 * std::f32::consts::PI * (kx * x_theta + ky * y_theta)).cos()
}
fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
    let kx_ratio = 12.0;
    let ky_ratio = 5.0;
    let theta = (30.0_f32 + _model.time).to_radians();
    let sigma = 128.0;
    let width = 128.0;
    let height = 128.0;
    let step_x = win.w() / width;
    let step_y = win.h() / height;
    for i in 0..width as i32 {
        for j in 0..height as i32 {
            let x = step_x * i as f32 - win.w() / 2.0;
            let y = step_y * j as f32 - win.h() / 2.0;
            let value = gabor(x, y, kx_ratio, ky_ratio, theta, sigma, width, height);
            let color = nannou::color::rgb(value.abs(), value.abs(), value.abs());
            draw.rect().x_y(x, y).w_h(step_x, step_y).color(color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
          .project_path()
          .expect("failed to locate project directory")
          .join("frames") 
          .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path); 
    } 
}