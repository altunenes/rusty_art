use nannou::prelude::*;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    counter: usize,
    path: Vec<Point2>,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .view(view)
        .build()
        .unwrap();

    let order = 8;
    let n = 2usize.pow(order as u32);
    let total = n * n;
    let window_rect = app.window_rect();
    let len = window_rect.w().min(window_rect.h()) / n as f32;

    let mut path = Vec::with_capacity(total);
    for i in 0..total {
        let mut v = hilbert(i, order);
        v *= len;
        v -= vec2(len * n as f32 / 2.0, len * n as f32 / 2.0);  // shift the curve to the center
        path.push(v);
    }

    Model { counter: 0, path }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.counter < model.path.len() {
        model.counter += 50;
    } else {
        model.counter = 0;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let counter = model.counter;

    for i in 1..counter {
        let hue = map_range(i, 0, model.path.len(), 0.0, 1.0);
        let color = hsl(hue, 1.0, 0.5);
        draw.line()
            .start(model.path[i - 1])
            .end(model.path[i])
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn hilbert(i: usize, order: u8) -> Point2 {
    let points = [
        pt2(0.0, 0.0),
        pt2(0.0, 1.0),
        pt2(1.0, 1.0),
        pt2(1.0, 0.0),
    ];

    let mut index = i & 3;
    let mut v = points[index];

    for j in 1..order {
        index = (i >> (2 * j as usize)) & 3;
        let len = 2f32.powi(j as i32);
        v = match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
                v
            }
            1 => {
                v.y += len;
                v
            }
            2 => {
                v.x += len;
                v.y += len;
                v
            }
            3 => {
                let temp = len - 1.0 - v.x;
                v.x = len - 1.0 - v.y;
                v.y = temp;
                v.x += len;
                v
            }
            _ => unreachable!(),
        };
    }
    v
}