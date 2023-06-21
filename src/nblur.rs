use nannou::prelude::*;
use nannou::image::{open, RgbaImage, DynamicImage};
use nannou::wgpu::Texture;
use std::path::PathBuf;
use std::option::Option;
use nannou_egui::{self, egui, Egui};

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
    noise_type: NoiseType,
    settings: Settings,
    egui: Egui,
    restart: bool,

}

struct Settings {
    blur_range: f32,
    noise_amount: f32,
    s: f32,
    blur_strength: f32,

}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).raw_event(raw_window_event).build().unwrap();

    let window = app.window(_w_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        blur_range: 20.0,
        noise_amount: 30.0,
        s: 0.001,
        blur_strength: 1.0,

    };

    Model {
        img,
        texture: None,
        noise_type: NoiseType::Original, 
        settings,
        egui,
        restart: false, 

    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let egui = &mut model.egui;
    let settings = &mut model.settings;
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.add(egui::Slider::new(&mut settings.blur_range, 0.0..=50.0).text("blur_range"));
        ui.add(egui::Slider::new(&mut settings.noise_amount, 1.0..=50.0).text("noise_amount"));
        ui.add(egui::Slider::new(&mut settings.s, 0.0..=100.01).text("s"));
        ui.add(egui::Slider::new(&mut settings.blur_strength, 0.0..=7.0).text("blur_strength"));
        

        ui.horizontal(|ui| {
            if ui.button("Original").clicked() {
                model.noise_type = NoiseType::Original;
            }
            if ui.button("Vibrant").clicked() {
                model.noise_type = NoiseType::Vibrant;
            }
            if ui.button("Random").clicked() {
                model.noise_type = NoiseType::Random;
            }
            if ui.button("Restart").clicked() {
                model.restart = true;  // set to true when button is clicked
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Per Channel").clicked() {
                model.noise_type = NoiseType::PerChannel;
            }
            if ui.button("Darker").clicked() {
                model.noise_type = NoiseType::Darker;
            }
            if ui.button("Lighter").clicked() {
                model.noise_type = NoiseType::Lighter;
            }
            if ui.button("Middle").clicked() {
                model.noise_type = NoiseType::Middle;
            }
        });
    });

    let noise_amount = settings.noise_amount as i32;

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
                let noise_probability = 1.1;
                if rand::random::<f32>() < noise_probability {
                    let r = rand::random::<u8>();
                    let g = rand::random::<u8>();
                    let b = rand::random::<u8>();
                    let a = pixel[3];
                    *pixel = nannou::image::Rgba([r, g, b, a]);
                }
            }
            NoiseType::Random => {
                let r = (pixel[0] as i32
                    + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount))
                .clamp(0, 255) as u8;
                let g = (pixel[1] as i32
                    + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount))
                .clamp(0, 255) as u8;
                let b = (pixel[2] as i32
                    + (rand::random::<i32>().rem_euclid(2 * noise_amount) - noise_amount))
                .clamp(0, 255) as u8;
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
            }
            NoiseType::Darker => {

                let noise_ratio = 0.1; 
                let r_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
                let g_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
                let b_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
            
                let r = (pixel[0] as i32 - r_noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 - g_noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 - b_noise).clamp(0, 255) as u8;
                let a = pixel[3];
                *pixel = nannou::image::Rgba([r, g, b, a]);
            }
            NoiseType::Lighter => {
                let noise_ratio = 0.1; 
                let r_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
                let g_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
                let b_noise = (rand::random::<i32>().rem_euclid(noise_amount) as f32 * noise_ratio) as i32;
                let r = (pixel[0] as i32 + r_noise).clamp(0, 255) as u8;
                let g = (pixel[1] as i32 + g_noise).clamp(0, 255) as u8;
                let b = (pixel[2] as i32 + b_noise).clamp(0, 255) as u8;
                let a = pixel[3];
            
                *pixel = nannou::image::Rgba([r, g, b, a]);
            }
            NoiseType::Middle => {
                let r = (pixel[0] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let g = (pixel[1] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let b = (pixel[2] as i32 + rand::random::<i32>().rem_euclid(noise_amount)) as u8;
                let a = pixel[3];

                *pixel = nannou::image::Rgba([r, g, b, a]);
            }
        }
    }

    let blur_range = settings.blur_range;
    let oscillation_speed = settings.s;
    let sg = (settings.blur_strength + oscillation_speed).rem_euclid(2.0 * std::f32::consts::PI);
    let blur_strength = (blur_range / 2.0) * (1.0 - sg.sin());

    model.img = nannou::image::imageops::blur(&model.img, blur_strength);

    let dyn_image = DynamicImage::ImageRgba8(model.img.clone());
    model.texture = Some(Texture::from_image(_app, &dyn_image));

    if model.restart {
        model.img = open(get_image_path("images/mona.jpg")).unwrap().to_rgba8();
    
        model.restart = false;  
    }


}



fn view(app: &App, model: &Model, frame: Frame) {
    let _settings = &model.settings;
    let draw = app.draw();
    
    if let Some(texture) = &model.texture {
        draw.texture(texture);
    }
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();

    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    }