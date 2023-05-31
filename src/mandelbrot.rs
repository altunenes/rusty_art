use nannou::prelude::*;
use std::f64::consts::PI;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MAX_ITER_START: usize = 0;
const RESOLUTION: u32 = 2;
const MAX_ITER_LIMIT: usize = 100;

struct Model {
    max_iter: usize,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(_app: &App) -> Model {
    _app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    Model {
        max_iter: MAX_ITER_START,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.max_iter < MAX_ITER_LIMIT {
        model.max_iter += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    for y in (0..HEIGHT).step_by(RESOLUTION as usize) {
        for x in (0..WIDTH).step_by(RESOLUTION as usize) {
            let (scaled_x, scaled_y) = scale_coords(x, y);
            let c = (scaled_x, scaled_y);
            let mut z = (0.0, 0.0);
            let mut cnt = 0;
            while cnt < model.max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
                z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                cnt += 1;
            }

            let hue = cnt as f32 / model.max_iter as f32;
            let saturation = 1.0;
            let value = 0.4 + 0.4 * (0.4 + app.time + hue * PI as f32).cos();

            draw.rect()
                .w_h(RESOLUTION as f32, RESOLUTION as f32)
                .x_y(
                    x as f32 - WIDTH as f32 / 2.0,
                    y as f32 - HEIGHT as f32 / 2.0,
                )
                .color(hsv(hue, saturation, value));
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn scale_coords(x: u32, y: u32) -> (f64, f64) {
    let scaled_x = (x as f64 / WIDTH as f64) * 2.6 - 2.1;
    let scaled_y = (y as f64 / HEIGHT as f64) * 2.4 - 1.2;
    (scaled_x, scaled_y)
}