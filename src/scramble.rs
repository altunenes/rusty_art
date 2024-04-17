use nannou::image::{open, DynamicImage, GenericImageView, GenericImage, ImageBuffer, Rgba};
use nannou::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use nannou::wgpu::Texture;
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
struct Model {
    img: DynamicImage,
    original_img: DynamicImage,
    texture: Option<Texture>,
    last_shuffle_time: f32,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    interval: f32,
    n: u32,
    open_file_dialog: bool,
    show_ui: bool,
}
fn model(app: &App) -> Model {
    let w = 800;
    let h = 600;
    let _w_id = app
        .new_window()
        .size(w, h)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        interval: 1.0,
        n: 8,
        open_file_dialog: false,
        show_ui : true,
    };
    Model {
        img: DynamicImage::new_rgba8(w, h),
        original_img: DynamicImage::new_rgba8(w, h),
        texture: None,
        last_shuffle_time: -settings.interval,
        egui,
        settings,
        scale:1.0,
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let egui = &mut model.egui;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    let settings = &mut model.settings;
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.interval, 0.0..=5.0).text("Interval"));
        ui.add(egui::Slider::new(&mut settings.n, 1..=16).text("N"));
        if ui.button("Load Image").clicked() {
            settings.open_file_dialog = true;
        }
    });
    if settings.open_file_dialog {
        if let Some(path) = FileDialog::new().pick_file() {
            if let Ok(new_img) = open(path) {
                model.original_img = new_img;
                model.img = model.original_img.clone();
                model.texture = Some(Texture::from_image(app, &model.img));
            }
            settings.open_file_dialog = false;
        }
    }
    if app.time - model.last_shuffle_time >= settings.interval {
        let n = settings.n;
        let (mut width, mut height) = (model.original_img.width(), model.original_img.height());
        if width % n != 0 {
            width = (width / n) * n;
        }
        if height % n != 0 {
            height = (height / n) * n;
        }
        let img = model.original_img.resize_exact(width, height, nannou::image::imageops::FilterType::Nearest);
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
        let draw = app.draw().scale(model.scale);
        draw.texture(texture);
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
}
fn main() {
    nannou::app(model).update(update).run();
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        if !cursor_over_egui {
            match delta {
                nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    model.scale *= 1.0 + *y * 0.05;
                    model.scale = model.scale.max(0.1).min(10.0);
                }
                _ => (),
            }
        }
    }
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = _app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}
