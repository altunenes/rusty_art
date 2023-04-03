use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    num_circles: usize,
    num_points: usize,
    circle_points: Vec<Vec<Point2>>,
}

fn model(app: &App) -> Model {
    let frequency = 125.0;
    let amplitude = 1.4;
    let phase = 1.0;
    let num_circles = 255;
    let num_points = 155;
    let window_rect = app.window_rect();
    let center = window_rect.xy();
    let radius = window_rect.w().min(window_rect.h()) / 2.0;
    let circle_radius = radius / (num_circles as f32);
    let mut circle_points = Vec::with_capacity(num_circles);
    for i in 0..num_circles {
        let mut points = Vec::with_capacity(num_points);
        for j in 0..num_points {
            let angle = j as f32 * 4.0 * PI / (num_points as f32);
            let x = center.x + angle.sin() * circle_radius * (i as f32 + 2.0);
            let y = center.y + angle.cos() * circle_radius * (i as f32 + 2.0);
            points.push(pt2(x, y));
        }
        circle_points.push(points);
    }
    Model {
        frequency,
        amplitude,
        phase,
        num_circles,
        num_points,
        circle_points,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for i in 0..model.num_circles {
        for j in 0..model.num_points {
            let x =
                model.circle_points[i][j].x + (-25.0 * PI * model.frequency * j as f32 / model.num_points as f32 + model.phase).sin() * model.amplitude;
            let y =
                model.circle_points[i][j].y + (25.0 * PI * model.frequency * j as f32 / model.num_points as f32 + model.phase).cos() * model.amplitude;
            model.circle_points[i][j] = pt2(x, y);
        }
    }
    model.phase += 0.01;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for i in 0..model.num_circles {
        let hue = i as f32 / model.num_circles as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.polyline()
            .weight(1.5)
            .points(model.circle_points[i].clone())
            .color(color);
    }
    /*if app.elapsed_frames() % 10 == 0 {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } */
    draw.to_frame(app, &frame).unwrap();
}
