// main snowflake function based from:https://github.com/plotters-rs/plotters/blob/master/plotters/examples/animation.rs

use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

struct Model {
    angle: f32,
    iterations: u32,
    elapsed_frames: u32,
    partial_iteration: f32,
    snowflake: Snowflake,
    inner_snowflake: Snowflake,
    
}

struct Snowflake {
    vertices: Vec<Vec2>,
}

impl Snowflake {
    fn new(iterations: u32, partial_iteration: f32) -> Self {
        let mut vertices = vec![
            vec2(0.0, 1.0),
            vec2((3.0f32).sqrt() / 2.0, -0.5),
            vec2(-(3.0f32).sqrt() / 2.0, -0.5),
            vec2(0.0, 1.0),
        ];

        for _ in 0..iterations {
            vertices = iterate(&vertices, 1.0);
        }

        if partial_iteration > 0.0 {
            vertices = iterate(&vertices, partial_iteration);
        }

        Snowflake { vertices }
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 600)
        .view(view)
        .build()
        .unwrap();

    Model {
        angle: 0.0,
        iterations: 0,
        elapsed_frames: 0,
        partial_iteration: 0.0,
        snowflake: Snowflake::new(0, 0.0),
        inner_snowflake: Snowflake::new(0, 0.0),
    }
}

fn iterate(points: &[Vec2], fraction: f32) -> Vec<Vec2> {
    let mut new_points = Vec::new();
    for i in 0..points.len() - 1 {
        let start = points[i];
        let end = points[i + 1];
        let t = (end - start) / 3.0 * fraction;
        let s = vec2(t.x * 0.5 - t.y * (0.75f32).sqrt(), t.y * 0.5 + (0.75f32).sqrt() * t.x) * fraction;
        new_points.push(start);
        new_points.push(start + t);
        new_points.push(start + t + s);
        new_points.push(start + t * 2.0);
    }

    new_points.push(*points.last().unwrap());

    new_points
}



fn update(_app: &App, model: &mut Model, _update: Update) {
    const SPEED_ROTATION: f32 = 0.0005;
    const SPEED_ITERATION: f32 = 0.0005;

    model.angle += SPEED_ROTATION;



    model.partial_iteration += SPEED_ITERATION;

    if model.partial_iteration > 1.0 {
        model.iterations += 1;
        model.partial_iteration -= 1.0;
    }

    if model.iterations > 5 {
        model.iterations = 0;
        model.partial_iteration = 0.0;
    }

    model.snowflake = Snowflake::new(model.iterations, model.partial_iteration);

    if model.elapsed_frames > 1 {
        let inner_iterations = if model.iterations > 0 { model.iterations - 0 } else { 0 };
        model.inner_snowflake = Snowflake::new(inner_iterations, model.partial_iteration);
    } else {
        model.inner_snowflake = Snowflake::new(0, model.partial_iteration);
    }
    model.elapsed_frames += 1;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let center = pt2(0.0, 0.0);
    draw.background().color(BLACK);
    let outer_snowflake = &model.snowflake;
    let inner_snowflake = &model.inner_snowflake;
    let num_points = outer_snowflake.vertices.len() as f32;
    draw.polyline()
        .weight(2.0)
        .points_colored(outer_snowflake.vertices.iter().enumerate().map(|(i, p)| {
            let progress = i as f32 / num_points;
            let hue = 1.0 - (0.1 + 0.1 * (0.4 + app.time + progress * PI).sin());
            let saturation = 1.0;
            let value = 0.4 + 0.4 * (0.4 + app.time + progress * PI).cos();
            let p = p.rotate(model.angle);
            (p * 250.0 + center, hsv(hue, saturation, value))
        }));
    let num_points = inner_snowflake.vertices.len() as f32;
    draw.polyline()
        .weight(2.0)
        .points_colored(inner_snowflake.vertices.iter().enumerate().map(|(i, p)| {
            let progress = i as f32 / num_points;
            let hue = 0.5 + 0.5 * (0.4 + app.time + progress * PI).sin();
            let saturation = 1.0;
            let value = 0.1 + 0.4 * (0.4 + app.time + progress * PI).cos();
            let p = p.rotate(model.angle);
            (p * 150.0 + center, hsv(hue, saturation, value))
        }));
        let num_points = inner_snowflake.vertices.len() as f32;
        draw.polyline()
            .weight(2.0)
            .points_colored(inner_snowflake.vertices.iter().enumerate().map(|(i, p)| {
                let progress = i as f32 / num_points;
                let hue = 0.5 + 0.5 * (0.4 + app.time + progress * PI).sin();
                let saturation = 1.0;
                let value = 0.4 + 0.4 * (0.4 + app.time + progress * PI).cos();
                let p = p.rotate(model.angle);
                (p * 50.0 + center, hsv(hue, saturation, value))
            }));

    draw.to_frame(app, &frame).unwrap();
}