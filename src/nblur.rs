use nannou::prelude::*;
use nannou::image::{open, RgbaImage, DynamicImage};
use nannou::wgpu::Texture;
use std::path::PathBuf;
use std::option::Option;

#[allow(dead_code)]
enum NoiseType {
    Original,
    Vibrant,
    Random,
    PerChannel,
    Darker,
    Lighter,
    Middle,
}

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    img: RgbaImage,
    texture: Option<Texture>,
    blur_strength: f32,
    noise_type: NoiseType,
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).build().unwrap();
    Model {
        img,
        texture: None,
        blur_strength: 1.0, 
        noise_type: NoiseType::Random, // Set the default noise type, choice here
    }
}


fn update(_app: &App, model: &mut Model, _update: Update) {
    let noise_amount = 30;

    for pixel in model.img.pixels_mut() {
        match model.noise_type {
            NoiseType::Original => {
                let noise = rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount;
                let r = (pixel[0] as i32 + noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 + noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 + noise).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            }
            NoiseType::Vibrant => {
                let noise_probability = 0.1;
                if rand::random::<f32>() < noise_probability {
                    let r = rand::random::<u8>();
                    let g = rand::random::<u8>();
                    let b = rand::random::<u8>();
                    let a = pixel[3];
                    *pixel = nannou::image::Rgba([r, g, b, a]);
                }
            }
            NoiseType::Random => {
                let r = (pixel[0] as i32 + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount)).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount)).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount)).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            }
            NoiseType::PerChannel => {
                let r_noise = rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount;
                let g_noise = rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount;
                let b_noise = rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount;
                let r = (pixel[0] as i32 + r_noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 + g_noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 + b_noise).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            },
            NoiseType::Darker => {
                let noise = rand::random::<i32>().rem_euclid(noise_amount);
                let r = (pixel[0] as i32 - noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 - noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 - noise).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            },
            NoiseType::Lighter => {
                let noise = rand::random::<i32>().rem_euclid(noise_amount);
                let r = (pixel[0] as i32 + noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 + noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 + noise).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            },

            NoiseType::Middle => {
                let r = (pixel[0] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let g = (pixel[1] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let b = (pixel[2] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let a = pixel[3];
        
                *pixel = nannou::image::Rgba([r, g, b, a]);
            },
        }
    }
    let blur_range = 20.0;
    let oscillation_speed = 0.005; 
    model.blur_strength = (model.blur_strength + oscillation_speed).rem_euclid(2.0 * std::f32::consts::PI);
    let blur_strength = (blur_range / 2.0) * (1.0 - model.blur_strength.sin());



    model.img = nannou::image::imageops::blur(&model.img, blur_strength);

    let dyn_image = DynamicImage::ImageRgba8(model.img.clone());
    model.texture = Some(Texture::from_image(_app, &dyn_image));

}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    
    if let Some(texture) = &model.texture {
        draw.texture(texture);
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