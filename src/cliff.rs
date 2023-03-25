use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    time: f32,
}

fn model(_app: &App) -> Model {
    Model { time: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time += 0.01;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let y_frequency = 150.0;
    let x_amplitude = 200.0;
    let y_amplitude = 100.0;
    let phase_shift = model.time;
    let point_count = 45;
    let hue_range = 0.9;
    let hue_offset = 0.1;

    let hue_values: Vec<f32> = (0..point_count)
        .map(|i| {
            let hue = i as f32 / point_count as f32 * hue_range + hue_offset;
            hue % 1.0
        })
        .collect();

    let points: Vec<Point2> = (0..point_count)
        .map(|i| {
            let x = (i as f32 / point_count as f32 * 12.0 * PI).sin() * x_amplitude;
            let y = ((i as f32 / point_count as f32 * 1.0 * PI * y_frequency) + phase_shift).sin()
                * y_amplitude;
            pt2(x, y)
        })
        .collect();

    for i in 0..point_count - 1 {
        let start = points[i];
        let end = points[i + 1];
        let hue_start = hue_values[i];
        let hue_end = hue_values[i + 1];
        let color_start = hsla(hue_start, 1.0, 0.5, 1.0);
        let color_end = hsla(hue_end, 1.0, 0.5, 1.0);
        draw.line()
            .start(start)
            .end(end)
            .weight(15.0)
            .color(color_start)
            .color(color_end);
    }

    draw.to_frame(app, &frame).unwrap();
}

