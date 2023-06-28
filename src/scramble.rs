//under construction


use nannou::image::{open, DynamicImage, GenericImageView, GenericImage, ImageBuffer, Rgba};
use nannou::prelude::*;
use std::path::PathBuf;
use rand::seq::SliceRandom;
use rand::thread_rng;
use nannou::wgpu::Texture;
use nannou_egui::{self, egui, Egui};

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}
struct Model {
    img: DynamicImage,
    texture: Option<Texture>,
    tiles: Vec<DynamicImage>,
    last_shuffle_time: f32,
    egui: Egui,
    settings: Settings,

}

struct Settings{
    interval: f32,
    n: u32,


}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();

    let _w_id = app
        .new_window()
        .size(img.width(), img.height())
        .view(view)
        .raw_event(raw_window_event)

        .build()
        .unwrap();

    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        interval: 1.0,
        n: 8,
    };

    Model {
        img: DynamicImage::ImageRgba8(img),
        texture: None,
        tiles: Vec::new(),
        last_shuffle_time: -settings.interval,  // Set to negative of SHUFFLE_INTERVAL
        egui,
        settings,
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {

    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.interval, 0.0..=5.0).text("interval"));
        ui.add(egui::Slider::new(&mut settings.n, 1..=16).text("n"));
    });


    if app.time - model.last_shuffle_time >= settings.interval {
    let n = settings.n;
    let (mut width, mut height) = (model.img.width(), model.img.height());
    
    // Ensure that the image dimensions are multiples of `n`
    if width % n != 0 {
        width = (width / n) * n;
    }
    if height % n != 0 {
        height = (height / n) * n;
    }
    
    let img = model.img.resize_exact(width, height, nannou::image::imageops::FilterType::Nearest);
    
    let tile_width = width / n;
    let tile_height = height / n;
    let mut tiles: Vec<_> = Vec::new();

    for y in (0..height).step_by(tile_height as usize) {
        for x in (0..width).step_by(tile_width as usize) {
            let tile = img.crop_imm(x, y, tile_width, tile_height);
            tiles.push(tile);
        }
    }

    tiles.shuffle(&mut thread_rng());

    let mut scrambled_img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (i, tile) in tiles.iter().enumerate() {
        let x = (i % n as usize) as u32 * tile_width;
        let y = (i / n as usize) as u32 * tile_height;
        scrambled_img.copy_from(&tile.to_rgba8(), x, y).unwrap();
    }

    model.img = DynamicImage::ImageRgba8(scrambled_img);
    model.texture = Some(Texture::from_image(app, &model.img));


model.last_shuffle_time = app.time;
} 
  
}




fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    if let Some(texture) = &model.texture {
        let draw = app.draw();
        draw.texture(texture);
        draw.to_frame(app, &frame).unwrap();
        model.egui.draw_to_frame(&frame).unwrap();    

    }
}

fn main() {
    nannou::app(model).update(update).run();
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}