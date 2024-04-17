use nannou::prelude::*;
use image::{GrayImage,open};
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
use butter2d::butterworth;
use nannou::wgpu::Texture;
struct Model {
    img: Option<GrayImage>,
    texture: Option<Texture>,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    cutoff_frequency_ratio: f64,
    order:f64,
    high_pass: bool,
    squared_butterworth: bool,
    open_file_dialog: bool,
    show_ui: bool,
}
fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}
fn model(app: &App) -> Model {
    let _w_id = app.new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(_w_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {
        img: None,
        texture: None,
        scale: 1.0,
        egui,
        settings: Settings {
            cutoff_frequency_ratio: 0.25,
            order:2.0,
            high_pass: true,
            squared_butterworth: false,
            open_file_dialog: false,
            show_ui: true,
        },
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let mut apply_filter = false;
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    {
        let egui = &mut model.egui;
        egui.set_elapsed_time(_update.since_start);
        let ctx = egui.begin_frame();
        let settings = &mut model.settings;
        egui::Window::new("Filter Settings").show(&ctx, |ui| {
            if ui.button("Load Image").clicked() {
                settings.open_file_dialog = true;
            }
            if ui.add(egui::Slider::new(&mut settings.cutoff_frequency_ratio, 0.0001..=0.49999).text("Cutoff Frequency Ratio")).changed() ||
                ui.add(egui::Slider::new(&mut settings.order, 0.0..=75.0).text("Order")).changed() ||
               ui.checkbox(&mut settings.high_pass, "High Pass Filter").changed() ||
               ui.checkbox(&mut settings.squared_butterworth, "Squared Butterworth Filter").changed() {
                apply_filter = true;
            }
            ui.checkbox(&mut settings.show_ui, "Show UI");
            ui.label("Non-square images may crash.Use mouse wheel to zoom in/out");

        });
        if settings.open_file_dialog {
            if let Some(path) = FileDialog::new().pick_file() {
                if let Ok(img) = open(path) {
                    let gray_image = img.to_luma8();
                    model.img = Some(gray_image);
                    apply_filter = true;
                    settings.open_file_dialog = false;
                }
            }
        }
    } 
    if apply_filter && model.img.is_some() {
        let (filtered_img, _) = butterworth(
            model.img.as_ref().unwrap(),
            model.settings.cutoff_frequency_ratio,
            model.settings.high_pass,
            model.settings.order,
            model.settings.squared_butterworth,
            0
        );
        model.img = Some(filtered_img);
    }
    if model.img.is_some() {
        update_texture(app, model);
    }
}
fn update_texture(app: &App, model: &mut Model) {
    if let Some(ref img) = model.img {
        let width = img.width();
        let height = img.height();
        let rgb_image: image::RgbImage = image::ImageBuffer::from_fn(width, height, |x, y| {
            let luma = img.get_pixel(x, y);
            image::Rgb([luma[0], luma[0], luma[0]])
        });
        let raw_data = rgb_image.to_vec();
        let buffer = nannou::image::ImageBuffer::<nannou::image::Rgb<u8>, Vec<u8>>::from_raw(width, height, raw_data).unwrap();
        let dyn_img = nannou::image::DynamicImage::ImageRgb8(buffer);
        model.texture = Some(Texture::from_image(app, &dyn_img));
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(GRAY);
    if let Some(ref texture) = model.texture {
        draw.texture(texture);
    }
    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
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