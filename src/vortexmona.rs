use nannou::prelude::*;
use nannou::image::{open, RgbaImage};
use std::path::PathBuf;
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: RgbaImage,
    angle_offset: f32,
}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).build().unwrap();
    Model {
        img,
        angle_offset: 0.0,
    }
}
fn update(_app: &App, _model: &mut Model, _update: Update) {
}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    let pixel_size = 1.0;
    let center = pt2(model.img.width() as f32 / 400.0 * pixel_size, model.img.height() as f32 / 400.0 * pixel_size);
    for (x, y, pixel) in model.img.enumerate_pixels() {
        let rgba = srgba(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0, pixel[3] as f32 / 255.0);
        let position = pt2(x as f32 * pixel_size - center.x, (model.img.height() as f32 - y as f32) * pixel_size - center.y);
        let distance_to_center = position.distance(pt2(0.0, 0.0));
        let angle = position.angle() + model.angle_offset + app.time * 0.005 * distance_to_center;
        let new_x = center.x + distance_to_center * angle.cos();
        let new_y = center.y + distance_to_center * angle.sin();
        draw.rect().color(rgba).w_h(pixel_size, pixel_size).xy(pt2(new_x, new_y));
    }
    draw.to_frame(app, &frame).unwrap();
}