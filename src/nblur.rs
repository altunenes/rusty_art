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
    blur_strength: f32,
}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).build().unwrap();
    Model {
        img,
        blur_strength: 1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.blur_strength += 0.05;
    let noise_amount = 10;
    for pixel in model.img.pixels_mut() {
        let r = (pixel[0] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
        let g = (pixel[1] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
        let b = (pixel[2] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
        let a = pixel[3];
        *pixel = nannou::image::Rgba([r, g, b, a]);
    }
    model.img = nannou::image::imageops::blur(&model.img, model.blur_strength);
}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    
    let pixel_size = 1.0;
    for (x, y, pixel) in model.img.enumerate_pixels() {
        let rgba = srgba(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0, pixel[3] as f32 / 255.0);
        let position = pt2(x as f32 * pixel_size - model.img.width() as f32 / 2.0, (model.img.height() as f32 - y as f32) * pixel_size - model.img.height() as f32 / 2.0);
        draw.rect().color(rgba).w_h(pixel_size, pixel_size).xy(position);
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}