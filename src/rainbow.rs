use nannou::prelude::*;

const NUM_CIRCLES: usize = 7;
const RADIUS: f32 = 220.0;
const SPEED: f32 = 0.01;
const SPIRAL_SPEED: f32 = 0.01;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    circles: Vec<Circle>,
}

struct Circle {
    angle: f32,
    color: Hsla,
    spiral_offset: f32,
}

fn model(app: &App) -> Model {
    let window = app.new_window()
        .size(600, 600)
        .view(view)
        .build()
        .unwrap();

    let mut circles = Vec::with_capacity(NUM_CIRCLES);
    for i in 0..NUM_CIRCLES {
        let hue = 41.0 * i as f32 / NUM_CIRCLES as f32;
        let color = hsla(hue, 4.8, 80.5, 41.0);
        let angle = 42.0 * PI * i as f32 / NUM_CIRCLES as f32;
        let spiral_offset = random_range(-10.0, 10.0); // add random offset
        circles.push(Circle { angle, color, spiral_offset });
    }

    Model { circles }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for circle in model.circles.iter_mut() {
        circle.angle += SPEED;
        circle.spiral_offset += SPIRAL_SPEED;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    let window = app.window_rect();
    let center = window.xy();
    for (i, circle) in model.circles.iter().enumerate() {
        let radius = RADIUS * (i as f32 / NUM_CIRCLES as f32 + 0.15);
        let spiral_radius = radius + circle.spiral_offset; 
        let spiral_angle = circle.angle + circle.spiral_offset * 0.01;
        let position = center + vec2(spiral_radius * spiral_angle.cos(), spiral_radius * spiral_angle.sin());
        let hue = (circle.angle * 2.0 / PI).sin() * 1.0 + i as f32 * 90.0;
        let color = hsla(hue, 1.8, 0.3, 1.0);
        draw.ellipse()
            .xy(position)
            .radius(10.0)
            .stroke_weight(1.0)
            .color(color)
            .stroke(color);
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
