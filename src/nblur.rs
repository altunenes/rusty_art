use nannou::prelude::*;
use nannou::image::{open, RgbaImage, DynamicImage};
use nannou::wgpu::Texture;
use std::option::Option;
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
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
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: Option<RgbaImage>,
    texture: Option<Texture>,
    noise_type: NoiseType,
    settings: Settings,
    egui: Egui,
}
struct Settings {
    blur_range: f32,
    noise_amount: f32,
    s: f32,
    blur_strength: f32,
    open_file_dialog: bool,
    show_ui: bool,
}
fn model(app: &App) -> Model {
    let _w_id = app.new_window().size(800, 600).view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(_w_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        blur_range: 20.0,
        noise_amount: 30.0,
        s: 0.001,
        blur_strength: 1.0,
        open_file_dialog: false,
        show_ui: true,
    };
    Model {
        img: None,
        texture: None,
        noise_type: NoiseType::Original,
        settings,
        egui,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let egui = &mut model.egui;
    let ctx = egui.begin_frame();
    let mut open_file_dialog = model.settings.open_file_dialog;
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        if ui.button("Load Image").clicked() {
            open_file_dialog = true;
        }
        ui.add(egui::Slider::new(&mut model.settings.blur_range, 0.0..=50.0).text("blur_range"));
        ui.add(egui::Slider::new(&mut model.settings.noise_amount, 1.0..=50.0).text("noise_amount"));
        ui.add(egui::Slider::new(&mut model.settings.s, 0.0..=100.01).text("s"));
        ui.add(egui::Slider::new(&mut model.settings.blur_strength, 0.0..=7.0).text("blur_strength"));
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
    model.settings.open_file_dialog = open_file_dialog;
    if open_file_dialog {
        if let Some(file_path) = FileDialog::new().pick_file() {
            if let Ok(img) = open(&file_path).map(|i| i.to_rgba8()) {
                let dyn_image = DynamicImage::ImageRgba8(img.clone());
                model.img = Some(img);
                model.texture = Some(Texture::from_image(_app, &dyn_image));
            }
            model.settings.open_file_dialog = false; 
        }
    }
    let noise_amount = model.settings.noise_amount as i32;
    if let Some(img) = model.img.as_mut() {
        for pixel in img.pixels_mut() {
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
    let blur_strength = model.settings.blur_range / 2.0 * (1.0 - model.settings.s.rem_euclid(2.0 * std::f32::consts::PI).sin());
    let blurred_img = nannou::image::imageops::blur(img, blur_strength);
    *img = blurred_img;
    let dyn_image = DynamicImage::ImageRgba8(img.clone());
    model.texture = Some(Texture::from_image(_app, &dyn_image));
}
}
fn view(app: &App, model: &Model, frame: Frame) {
    let _settings = &model.settings;
    let draw = app.draw();
    if let Some(texture) = &model.texture {
        draw.texture(texture);
    }
    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}
