//code is still a work in progress, so there are some comment sections for future improvements and changes
//major revision will be done in order to make it more efficient

use nannou::prelude::*;
use nannou::image::{open, RgbaImage};
use std::path::PathBuf;
use nannou::image::Pixel;
use nannou_egui::{self, egui, Egui};

const PI : f32 = 3.1415_f32;

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

enum ColorOption {
    Rainbow,
    Real,
}

enum AnimationOption {
    Vortex,
    Default,
}

struct Model {
    img: RgbaImage,
    time: f32,
    egui: Egui,
    settings: Settings,
    scale: f32,
}

struct Settings{
    color_option: ColorOption,
    animation_option: AnimationOption,
    t: f32,
    u:f32,
    v:f32,
}

fn main() {
    nannou::app(model).update(update).
    run();
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).raw_event(raw_window_event).build().unwrap();
    
    let window = app.window(_w_id).unwrap();
    let egui=Egui::from_window(&window);

    let settings = Settings {

        color_option: ColorOption::Real,
        animation_option: AnimationOption::Default,
        t: 20.0,
        v:1.0,
        u:1.6,
    };
    
    Model {
        img,
        time: 0.0,
        scale:1.0,
        egui,
        settings,
        }
}

fn update(_app: &App, model: &mut Model, _update: Update) {

    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("colors").clicked() {
            model.settings.color_option = match model.settings.color_option {
                ColorOption::Rainbow => ColorOption::Real,
                ColorOption::Real => ColorOption::Rainbow,
            };
        }

        if ui.button("animation").clicked() {
            model.settings.animation_option = match model.settings.animation_option {
                AnimationOption::Vortex => AnimationOption::Default,
                AnimationOption::Default => AnimationOption::Vortex,
            };
        }
        if ui.button("Restart").clicked() {
            model.time = 0.0;
        }

        ui.add(egui::Slider::new(&mut model.settings.t, 0.0..=100.0).text("t"));
        ui.add(egui::Slider::new(&mut model.settings.u, 0.0..=3.0).text("u"));
        ui.add(egui::Slider::new(&mut model.settings.v, 0.0..=10.0).text("v"));
    });

    // handle the output and paint_cmds as required

    model.time += _update.since_last.as_secs_f32();

}
    


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(GRAY);

    let win_rect = app.window_rect();
    let image_aspect_ratio = model.img.width() as f32 / model.img.height() as f32;

    let rect_w = win_rect.w() / model.img.width() as f32;
    let rect_h = win_rect.h() / model.img.height() as f32;

    for (x, y, pixel) in model.img.enumerate_pixels() {
        let x = x as f32;
        let y = y as f32;

        let uv = vec2(
            (x / model.img.width() as f32 - 0.5) * image_aspect_ratio,
            y / model.img.height() as f32 - 0.5,
        );

        let angle = uv.y.atan2(uv.x);
        let radius = uv.length();

        let spiral = vec2(
            angle / 2.0*PI + model.time * model.settings.v - radius * model.settings.t, 
            radius,
        );

        let color_intensity: f32;
        let mask: f32;

        match model.settings.animation_option {
            AnimationOption::Vortex => {
                let rotation_angle = 6.2 * (model.time + spiral.x) * (0.5 - radius).max(0.0);
                let adjusted_angle = angle + rotation_angle;
                color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * model.settings.u;
                mask = (spiral.x + adjusted_angle).fract() - color_intensity * 0.3;
            },
            AnimationOption::Default => {
                color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * 1.6;
                mask = spiral.x.fract() - color_intensity * 0.3;
            },
        }

        if mask.abs() < 0.2 {
            let color = match model.settings.color_option {
                ColorOption::Rainbow => nannou::color::hsv(angle / 6.2831, 1.0, 1.0),
                ColorOption::Real => nannou::color::rgb(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                ).into(),
            };
            
            draw.rect()
                .x_y(
                    win_rect.left() + x * rect_w,
                    win_rect.top() - y * rect_h,
                )
                .w_h(rect_w, rect_h)
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
}