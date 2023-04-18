use nannou::image;
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use std::path::PathBuf;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    
    current_dir.join(relative_path)
}
struct Model {
    img: image::RgbaImage,
    perlin: Perlin,
    egui: Egui,
    settings: Settings,

}
struct Settings {
    SQUARE_SIZE: f32,
    SQUARE_SPACING: f32,
    NOISE_SCALE: f64,
    NOISE_DIMENSIONS: f64,
    NOISE_RESOLUTION: f32,
    a: f32,
    b: f32,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let img_path = get_image_path("images/mona.jpg");
    let img = nannou::image::open(img_path).unwrap().to_rgba8();
    let perlin = Perlin::new();
    Model { img, perlin, egui, settings: Settings {
        SQUARE_SIZE: 5.0,
        SQUARE_SPACING: 0.0,
        NOISE_SCALE: 0.001,
        NOISE_DIMENSIONS: 44.0,
        NOISE_RESOLUTION: 1.0,
        a: 100.0,
        b: 100.0,
    },
}
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("SQUARE_SIZE:");
        ui.add(egui::Slider::new(
            &mut model.settings.SQUARE_SIZE,
            2.0..=100.0,
        ));
        ui.label("SQUARE_SPACING:");
        ui.add(egui::Slider::new(
            &mut model.settings.SQUARE_SPACING,
            0.0..=5.0,
        ));
        ui.label("NOISE_SCALE:");
        ui.add(egui::Slider::new(
            &mut model.settings.NOISE_SCALE,
            0.0..=0.001,
        ));
        ui.label("NOISE_DIMENSIONS:");
        ui.add(egui::Slider::new(
            &mut model.settings.NOISE_DIMENSIONS,
            0.0..=5.0,
        ));
        ui.label("NOISE_RESOLUTION:");
        ui.add(egui::Slider::new(
            &mut model.settings.NOISE_RESOLUTION,
            0.1..=3.0,
        ));
        ui.label("a:");
        ui.add(egui::Slider::new(
            &mut model.settings.a,
            0.0..=1000.0,
        ));
        ui.label("b:");
        ui.add(egui::Slider::new(
            &mut model.settings.b,
            0.0..=1000.0,
        ));

    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);
    let num_columns = (win.w() / (model.settings.SQUARE_SIZE * model.settings.NOISE_RESOLUTION)).ceil() as i32;
    let num_rows = (win.h() / (model.settings.SQUARE_SIZE * model.settings.NOISE_RESOLUTION)).ceil() as i32;
    let square_draw_size = model.settings.SQUARE_SIZE / model.settings.NOISE_RESOLUTION;
    for i in 0..num_columns {
        for j in 0..num_rows {
            let x = win.left() + (model.settings.SQUARE_SIZE + model.settings.SQUARE_SPACING) * i as f32 *  model.settings.NOISE_RESOLUTION;
            let y = win.bottom() + (model.settings.SQUARE_SIZE + model.settings.SQUARE_SPACING) * j as f32 *  model.settings.NOISE_RESOLUTION;
            let noise_val_x = model.perlin.get([
                x as f64 * model.settings.NOISE_SCALE,
                y as f64 * model.settings.NOISE_SCALE,
                app.elapsed_frames() as f64 * model.settings.NOISE_SCALE,
                model.settings.NOISE_DIMENSIONS,
            ]);
            let noise_val_y = model.perlin.get([
                (x as f64 + model.settings.a as f64) * model.settings.NOISE_SCALE,
                (y as f64 + model.settings.b as f64) * model.settings.NOISE_SCALE,
                app.elapsed_frames() as f64 * model.settings.NOISE_SCALE,
                model.settings.NOISE_DIMENSIONS,
            ]);
            let _angle = noise_val_x * std::f64::consts::PI;
            let _scale = noise_val_y;
            let img_x = ((noise_val_x * model.img.width() as f64).floor() as u32) % model.img.width();
            let img_y = ((noise_val_y * model.img.height() as f64).floor() as u32) % model.img.height();
            let pixel = model.img.get_pixel(img_x, img_y);
            draw.rect()
                .x_y(x, y)
                .w_h(square_draw_size, square_draw_size)
                .rgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    pixel[3] as f32 / 255.0,
                );
        }
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
