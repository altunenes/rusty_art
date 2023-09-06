//inspiration: https://twitter.com/timClicks
use nannou::prelude::*;
use nannou::image::{open, ImageBuffer, Rgba};
use std::path::PathBuf;
use nannou_egui::{self, egui, Egui};
type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: Image,
    texture: Option<wgpu::Texture>,
    settings: Settings,
    egui: Egui,
}
struct Settings {
    pixelation: f32,
    speed: f32,
    direction: f32, 
}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        pixelation: 50.0,
        speed: 0.0,
        direction: 1.0,
    };
    Model {
        img,
        texture: None,
        settings, 
        egui,
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.speed, 0.0..=1.0).text("speed"));
    });
    let new_dims = (model.settings.pixelation.max(1.0).round() as u32, model.settings.pixelation.max(1.0).round() as u32);
    let img_ = pixelate(&model.img, new_dims);
    let dynamic_img = nannou::image::DynamicImage::ImageRgba8(img_);
    let texture = wgpu::Texture::from_image(app, &dynamic_img);
    model.texture = Some(texture);
    model.settings.pixelation += model.settings.speed * model.settings.direction;
    model.settings.pixelation = model.settings.pixelation.min(50.0).max(1.0);
    if model.settings.pixelation <= 1.0 || model.settings.pixelation >= 50.0 {
        model.settings.direction = -model.settings.direction;
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
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
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

//pixelate and resize funcions from Tim Clicks (2023): https://www.youtube.com/watch?v=t4DmszQfD-Q&feature=youtu.be
fn pixelate(img: &Image, new_dims: (u32, u32)) -> Image {
    let old_dims = img.dimensions();
    let small = resize(img, ((old_dims.0 / new_dims.0), (old_dims.1 / new_dims.1)));
    
    resize(&small, old_dims)
}
fn resize(img: &Image, new_dims: (u32, u32)) -> Image {
    let (old_width, old_height) = img.dimensions();
    let (new_width, new_height) = new_dims;
    let mut resized = ImageBuffer::new(new_width, new_height);
    for (new_x, new_y, pixel) in resized.enumerate_pixels_mut() {
        let old_x = (new_x as f32 * (old_width  as f32 / new_width  as f32)) as u32;
        let old_y = (new_y as f32 * (old_height  as f32 / new_height  as f32)) as u32;
        let old_pixel = img.get_pixel(old_x, old_y).to_owned();
        *pixel = old_pixel;
    }
    resized
}
