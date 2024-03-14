use nannou::prelude::*;
use image::{GrayImage,Luma, open, RgbImage, Pixel};
use butter2d::butterworth;
use std::path::PathBuf;
fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}
struct Model {
    img: GrayImage, 
    cutoff_frequency_ratio: f64,
    increment: f64,
    osc_direction: f64, 
}
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/lena.png");
    let img = open(img_path).unwrap().to_rgb8();
    let gray_img = convert_to_grayscale(&img);
    app.new_window()
       .size(gray_img.width(), gray_img.height())
       .view(view)
       .build()
       .unwrap();
    Model {
        img: gray_img,
        cutoff_frequency_ratio: 0.0000001,
        increment: 0.1,
        osc_direction: 1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.cutoff_frequency_ratio += model.increment * model.osc_direction;
    if model.cutoff_frequency_ratio >= 0.4 || model.cutoff_frequency_ratio <= 0.01 {
        model.osc_direction *= -1.0;
    }
    let img_path = get_image_path("images/lena.png");
    let img = open(img_path).expect("Failed to open image").to_rgb8();
    let gray_img = convert_to_grayscale(&img);
    let (filtered_img, _) = butterworth(
        &gray_img,
        model.cutoff_frequency_ratio,
        true, 
        2.0,  
        false, 
        0 
    );
    model.img = filtered_img; 
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for (x, y, pixel) in model.img.enumerate_pixels() {
        let brightness = pixel[0] as f32 / 255.0;
        let flipped_y = model.img.height() as f32 - y as f32 - 1.0;
        draw.rect()
            .x_y(x as f32 - model.img.width() as f32 / 2.0, flipped_y - model.img.height() as f32 / 2.0)
            .w_h(1.0, 1.0) 
            .rgba(brightness, brightness, brightness, 1.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
fn convert_to_grayscale(img: &RgbImage) -> GrayImage {
    let mut gray_img = GrayImage::new(img.width(), img.height());
    for (x, y, pixel) in img.enumerate_pixels() {
        let rgb = pixel.to_rgb();
        let luma = (0.299 * rgb[0] as f64 + 0.587 * rgb[1] as f64 + 0.114 * rgb[2] as f64) as u8;
        gray_img.put_pixel(x, y, Luma([luma]));
    }
    gray_img
}