use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .fullscreen()

        .run();
}

struct Model {
    points: Vec<Point2>,
    angle: f32,
    scale: f32,
}

fn model(_app: &App) -> Model {
    Model {
        points: Vec::new(),
        angle: 0.0,
        scale: 1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let golden_angle = (1.0 + 5.0_f32.sqrt()) * std::f32::consts::PI;
    let r = model.points.len() as f32 * model.scale;
    let x = r * model.angle.cos();
    let y = r * model.angle.sin();
    let pos = pt2(x, y);
    model.points.push(pos);
    model.angle += golden_angle;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for (i, &pos) in model.points.iter().enumerate() {
        let hue = i as f32 / model.points.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        let radius = (i as f32).sqrt() * 2.0;
        draw.ellipse()
            .xy(pos)
            .radius(radius)
            .color(color)
            .stroke_weight(1.0)
            .stroke(WHITE);
    }
    draw.to_frame(app, &frame).unwrap();
}