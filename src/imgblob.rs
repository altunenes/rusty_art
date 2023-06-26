
use nannou::image::{open, DynamicImage, GenericImageView, Rgba};
use nannou::prelude::*;
use nannou::wgpu::Texture;
use std::path::PathBuf;
use nannou_egui::{self, egui, Egui};

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)}
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
struct Settings
{
    use_real_colors: bool,
    colors:usize,
    sampling:usize,

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
    let settings = Settings {colors:1 ,use_real_colors: false,sampling:8
    };
    Model { img: DynamicImage::ImageRgba8(img), texture: None, scale, egui, settings,zoom: 1.0}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label(format!("color {}",settings.colors));
        if ui.button("next").clicked(){
            settings.colors = (settings.colors%3)+1;
        }

        ui.add(egui::Slider::new(&mut settings.sampling, 4..=10).text("sampling"));
        

        ui.add(egui::Checkbox::new(&mut settings.use_real_colors, "Use Real Colors"));
    });

    model.texture = Some(Texture::from_image(app, &model.img));
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw().scale(model.zoom);

    let (img_width, img_height) = model.img.dimensions();
    let step_size = model.settings.sampling;
    let time = app.time;
    
    for y in (0..img_height).step_by(step_size) {
        for x in (0..img_width).step_by(step_size) {
            let pixel = model.img.get_pixel(x, y);
            let luminance = calculate_luminance(&pixel);
            let oscillation = (time.sin() * 0.5 + 0.5).abs();
            let radius_state1 = map_range(luminance, 0.0, 1.0, 0.05, 2.0);
            let radius_state2 = map_range(luminance, 0.0, 1.0, 2.0, 0.05);
            let radius = radius_state1 * oscillation + radius_state2 * (1.0 - oscillation);
            let x: f32 = (x as f32 - img_width as f32 / 2.0) * model.scale;
            let y = ((img_height - y) as f32 - img_height as f32 / 2.0) * model.scale;
            let color = if model.settings.use_real_colors {
                let r = pixel.0[0] as f32 / 255.0;
                let g = pixel.0[1] as f32 / 255.0;
                let b = pixel.0[2] as f32 / 255.0;
                rgba(r, g, b, 1.0)
            } else{
            match model.settings.colors {
                1 =>
            {
                let progress = y as f32 / img_height as f32; 
                let hue = progress;
                let saturation = 1.0 - progress;
                let lightness =  0.4 * (0.5 + time + progress * PI).sin();
                hsla(hue, saturation, lightness, 1.0)
            }

            2 => {
                hsla(0.0, 0.0, 0.0, 1.0)
            }

            3 => { 
                let hue = (y as f32 / img_height as f32).fract();
                let saturation = 1.0;
                let lightness = 0.5;
                hsla(hue, saturation, lightness, 1.0)
            },

            _ => unreachable!(),
        }.into()
    };

            draw.ellipse()
                .x_y(x, y)
                .radius(radius)
                .color(color);

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

fn calculate_luminance(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
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

