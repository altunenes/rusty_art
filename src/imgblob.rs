
// I will contiune this file tomorrow, so don't bother with commeents for now. They are just for me to remember what I was doing.
use nannou::image::{open, DynamicImage, GenericImageView, Rgba};
use nannou::prelude::*;
use nannou::wgpu::Texture;
use std::path::PathBuf;

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    img: DynamicImage,
    texture: Option<Texture>,
    scale: f32,
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let scale = 1.0;
    let _w_id = app
        .new_window()
        .size((img.width() as f32 * scale) as u32, (img.height() as f32 * scale) as u32)
        .view(view)
        .build()
        .unwrap();
    Model { img: DynamicImage::ImageRgba8(img), texture: None, scale }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // Convert the image to a dynamic image so that we can create a texture from it.
    model.texture = Some(Texture::from_image(app, &model.img));
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw();

    // We extract the dimensions of the image for the iteration
    let (img_width, img_height) = model.img.dimensions();
    let step_size = 8;

    for y in (0..img_height).step_by(step_size) {
        for x in (0..img_width).step_by(step_size) {
            // Get the pixel, and calculate its luminance
            let pixel = model.img.get_pixel(x, y);
            let luminance = calculate_luminance(&pixel);
    
            // Convert the pixel's position and luminance into a radius for the ellipse
            let radius: f32 = map_range(luminance, 0.0, 1.0, 1.0, 2.0);
    
            // Convert the image's pixel position to window's position
            let x = (x as f32 - img_width as f32 / 2.0) * model.scale;
            let y = ((img_height - y) as f32 - img_height as f32 / 2.0) * model.scale;
    
            draw.ellipse()
                .x_y(x, y)
                .radius(radius)
                .rgba(0.0, 0.0, 0.0, 1.0);
        }
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

fn calculate_luminance(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
