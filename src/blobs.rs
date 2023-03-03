use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    circles: Vec<(Point2, f32)>,
    angle: f32,
    speed: f32,
}

fn model(_app: &App) -> Model {
    Model {
        circles: Vec::new(),
        angle: 0.0,
        speed: 0.11,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += model.speed;
    let r = model.angle.sqrt() * 50.0;
    let x = r * model.angle.cos();
    let y = r * model.angle.sin();
    let center = pt2(x, y);
    let radius = 1.0 + 20.0 * (model.angle % 1.0);
    model.circles.push((center, radius));
    if model.circles.len() > 1000 {
        model.circles.remove(0);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for (i, &(center, radius)) in model.circles.iter().enumerate() {
        let hue = i as f32 / model.circles.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.ellipse()
            .xy(center)
            .radius(radius)
            .color(color)
            .stroke_weight(2.0)
            .stroke(STEELBLUE);
    }
    draw.to_frame(app, &frame).unwrap();
}