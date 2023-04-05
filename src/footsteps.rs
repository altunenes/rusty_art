//click/hold on the space bar to hide the black stripes

use nannou::prelude::*;
const STRIP_WIDTH: f32 = 7.0;
const BLOCK_WIDTH: f32 = 44.0;
const BLOCK_HEIGHT: f32 = 30.0;
const Y_YELLOW: f32 = 40.0;
const Y_BLUE: f32 = 180.0;
const SPEED: f32 = 0.04;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(800, 600)
        .run();
}

struct Model {
    counter: f32,
    stripes_visible: bool,
}



fn model(_app: &App) -> Model {
    Model { counter: 0.0, stripes_visible: true }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.counter += SPEED;
    if model.counter >= app.window_rect().w() {
        model.counter = 0.0;
    }

    if app.keys.down.contains(&Key::Space) {
        model.stripes_visible = !model.stripes_visible;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    if model.stripes_visible {
        let window = app.window_rect();
        for i in (window.left() as i32..window.right() as i32).step_by((2.0 * STRIP_WIDTH) as usize) {
            let x = i as f32;
            draw.rect()
                .x_y(x + STRIP_WIDTH / 2.0, window.y())
                .w_h(STRIP_WIDTH, window.h())
                .color(BLACK);
        }
    }

    let yellow_x = model.counter;
    let blue_x = model.counter;

    draw.rect()
        .x_y(yellow_x + BLOCK_WIDTH, Y_YELLOW + BLOCK_HEIGHT)
        .w_h(BLOCK_WIDTH, BLOCK_HEIGHT)
        .color(YELLOW);

    draw.rect()
        .x_y(blue_x + BLOCK_WIDTH, Y_BLUE + BLOCK_HEIGHT)
        .w_h(BLOCK_WIDTH, BLOCK_HEIGHT)
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}