use nannou::noise::{NoiseFn, Perlin, Turbulence};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().raw_event(raw_window_event).size(800, 800).view(view).build().unwrap();

    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}


fn draw_eye(draw: &Draw, center: Point2, t: f32) {
    let iris_radius= 100.0;
    let outer_iris_radius = iris_radius * 1.2;
    let edge_samples = 10;
    for i in 0..edge_samples{
        let ratio = i as f32 / edge_samples as f32;
        let color = hsv(0.0,0.0,1.0);
        let radius = map_range(ratio, 0.0, 1.0, iris_radius, outer_iris_radius);
        draw.ellipse()
        .xy(center)
        .radius(radius)
        .color(color);
    }

    let perlin = Turbulence::new(Perlin::new()); 
    let samples = 100;
    for i in 0..samples {
        let angle = i as f32 / samples as f32 * TAU;
        let r = iris_radius * (1.0+ perlin.get([angle as f64, t as f64]) as f32 * 0.1);
        let x = r*angle.cos();
        let y = r*angle.sin();
        let color = hsv(0.1, 0.6, 0.4 + perlin.get([x as f64, y as f64, t as f64]) as f32 * 0.01); 
        draw.line()
        .start(center)
        .end(center + vec2(x,y))
        .color(color)
        .weight(1.0);
    }

    draw.ellipse()
    .xy(center)
    .radius(40.0)
    .color(BLACK);
    let white_shift = vec2(t.sin()*10.0, (t/2.0).cos()*10.0);
    let white_radius= 10.0 + t.sin()*5.0;
    draw.ellipse() 
    .xy(center + white_shift)
    .radius(white_radius)
    .color(WHITE);
}


fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(GRAY);

    let win = app.window_rect();
    let eye_distance = 150.0;
    let t = app.time;  

    draw_eye(&draw, win.xy() + vec2(-eye_distance, 0.0), t);

    draw_eye(&draw, win.xy() + vec2(eye_distance, 0.0), t);

    draw.to_frame(app, &frame).unwrap();
}
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}