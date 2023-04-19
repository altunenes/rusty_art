use nannou::prelude::*;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    dot_size: f32,
    animation_paused: bool,
}
fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();

    Model { dot_size: 0.0, animation_paused: false }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    if !model.animation_paused {
        model.dot_size = 0.35 * (0.5 * app.time.sin().powi(4) + 0.5);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.rect().x_y(-win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(YELLOW);
    draw.rect().x_y(win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(PURPLE);
    draw.ellipse()
        .x_y(win.w() / 7.0, 0.0)
        .radius(0.45 * win.h() / 2.0)
        .color(RED);
    draw.ellipse()
        .x_y(-win.w() / 7.0, 0.0)
        .radius(0.45 * win.h() / 2.0)
        .color(RED);
    for x in 0..=120 {
        for y in 0..=120 {
            let uv = pt2(
                (x as f32) / 120.0 * win.w() - win.right() / 1.0,
                (y as f32) / 120.0 * win.h() - win.top() / 1.0,
            );
            let dot_color = if uv.x < 0.0 { PURPLE } else { YELLOW };
            draw.ellipse()
                .xy(uv)
                .radius(model.dot_size * win.w() / 120.0)
                .color(dot_color);
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
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    use nannou::winit::event::{ElementState, WindowEvent};

    if let WindowEvent::MouseInput { button, state, .. } = event {
        if *button == MouseButton::Left && *state == ElementState::Pressed {
            model.animation_paused = !model.animation_paused;
        }
    }
}