use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    lines: Vec<(Point2, Point2)>,
    angle: f32,
    speed: f32,
    line_length: f32,
}

fn model(_app: &App) -> Model {
    Model {
        lines: Vec::new(),
        angle: 0.0,
        speed: 1.05,
        line_length: 15.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += model.speed;
    let r = model.angle.sqrt() * 10.0;
    let x = r * model.angle.cos();
    let y = r * model.angle.sin();
    let start = pt2(x, y);
    let end = pt2(x + model.line_length, y + model.line_length);
    model.lines.push((start, end));
    if model.lines.len() > 1000 {
        model.lines.remove(0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for (i, &(start, end)) in model.lines.iter().enumerate() {
        let hue = i as f32 / model.lines.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.line()
            .start(start)
            .end(end)
            .color(color)
            .stroke_weight(5.0);
    }

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}