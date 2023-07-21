
//inspired by Roni Kaufman, https://ronikaufman.github.io/
use nannou::image::{open, DynamicImage, GenericImageView, Rgba};
use nannou::prelude::*;
use nannou::wgpu::Texture;
use std::path::PathBuf;
use nannou_egui::{self, egui, Egui};

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    img: DynamicImage,
    egui: Egui,
    zoom: f32,
    texture: Option<Texture>,
    scale: f32,
    settings: Settings,
}

#[allow(dead_code)]
struct Settings {
    colors: usize,
    use_real_colors: bool,
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let scale = 1.0;
    let _w_id = app
        .new_window()
        .size((img.width() as f32 * scale) as u32, (img.height() as f32 * scale) as u32)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        colors: 1,
        use_real_colors: false,
    };
    Model {
        img: DynamicImage::ImageRgba8(img),
        texture: None,
        scale,
        egui,
        settings,
        zoom: 1.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label(format!("color {}", settings.colors));
        if ui.button("next").clicked() {
            settings.colors = (settings.colors % 3) + 1;
        }

        ui.add(egui::Checkbox::new(&mut settings.use_real_colors, "Use Real Colors"));
    });

    model.texture = Some(Texture::from_image(app, &model.img));
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw().scale(model.zoom);

    let (img_width, img_height) = model.img.dimensions();
    let center_x = img_width as f32 / 2.0;
    let center_y = img_height as f32 / 2.0;
    let max_d = dist_squared(0.0, 0.0, center_x, center_y);

    for (x, y, pixel) in model.img.pixels() {
        let gray = calculate_luminance(&pixel);
        let d = dist_squared(x as f32, y as f32, center_x, center_y);
        let hue = map_range(d, 0.0, max_d, gray, gray+1.0) % 1.0;

        let x: f32 = (x as f32 - center_x) * model.scale;
        let y = (center_y - y as f32) * model.scale;

let color = if model.settings.use_real_colors {
    let r = pixel.0[0] as f32 / 255.0;
    let g = pixel.0[1] as f32 / 255.0;
    let b = pixel.0[2] as f32 / 255.0;
    rgba(r, g, b, 1.0)
} else {
    let hsv_color = hsv(hue, 1.0, 1.0);
    let rgb_color = Rgb::from(hsv_color);
    rgba(rgb_color.red, rgb_color.green, rgb_color.blue, 1.0)
};

        draw.rect()
            .x_y(x, y)
            .w_h(model.scale, model.scale)
            .color(color);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}

fn calculate_luminance(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}

fn dist_squared(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx * dx + dy * dy
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        if !cursor_over_egui {
            match delta {
                nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    model.zoom *= 1.0 + *y * 0.05;
                    model.zoom = model.zoom.max(0.1).min(10.0);
                }
                _ => (),
            }
        }
    }
}