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
#[derive(Debug)]
enum ColorOption {
    Rainbow,
    Real,
    Black,
    Test,
    Ranim,
}
#[derive(Debug)]
enum AnimationOption {
    Vortex,
    Vortex2,
    Default,
    Luminance,
    Luminance2,
    Luminance3,
    Spiral,
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
    frequency: f32,
    t: f32,
    u:f32,
    v:f32,
    mask:f32,
    th:f32,
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
        frequency:1.0,
        mask:0.3,
        th:1.0,
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
                ColorOption::Rainbow => ColorOption::Black,
                ColorOption::Black => ColorOption::Real,
                ColorOption::Real => ColorOption::Test,
                ColorOption::Test => ColorOption::Ranim,
                ColorOption::Ranim => ColorOption::Rainbow,

            };
        }
        ui.label(format!("Color {:?}",model.settings.color_option));

        if ui.button("animation").clicked() {
            model.settings.animation_option = match model.settings.animation_option {
                AnimationOption::Vortex => AnimationOption::Vortex2,
                AnimationOption::Vortex2 => AnimationOption::Luminance,
                AnimationOption::Luminance => AnimationOption::Luminance2,
                AnimationOption::Luminance2 => AnimationOption::Luminance3,
                AnimationOption::Luminance3 => AnimationOption::Default,
                AnimationOption::Default => AnimationOption::Spiral,
                AnimationOption::Spiral => AnimationOption::Vortex,
            };
        }
        ui.label(format!("Animation {:?}",model.settings.animation_option));
        if ui.button("Restart").clicked() {
            model.time = 0.0;
        }

        ui.add(egui::Slider::new(&mut model.settings.t, 0.0..=100.0).text("t"));
        ui.add(egui::Slider::new(&mut model.settings.u, 0.0..=3.0).text("u"));
        ui.add(egui::Slider::new(&mut model.settings.v, 0.0..=10.0).text("v"));
        ui.add(egui::Slider::new(&mut model.settings.frequency, 0.1..=10.0).text("frequency"));
        ui.add(egui::Slider::new(&mut model.settings.mask, 0.0..=1.0).text("mask"));
        ui.add(egui::Slider::new(&mut model.settings.th, 0.1..=20.0).text("th"));
    });


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
        let luminance = 0.2126* pixel[0] as f32 /2551.0 + 0.7152*pixel[1] as f32 /255.0 + 0.07220*pixel[2] as f32 / 255.0;
        let uv = vec2(
            (x / model.img.width() as f32 - 0.5) * image_aspect_ratio,
            y / model.img.height() as f32 - 0.5,
        );
        let angle = uv.y.atan2(uv.x);
        let radius: f32 = uv.length();

        let spiral = vec2(
            angle / PI + model.time * model.settings.v - radius * model.settings.t, 
            radius,
        );

        let color_intensity: f32;
        let mask: f32;
        match model.settings.animation_option {
            AnimationOption::Vortex => {
                let rotation_angle = PI * (model.time + spiral.x) * (0.5 - radius).max(0.0);
                let adjusted_angle = angle + rotation_angle;
                color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * model.settings.u;
                mask = (spiral.x + adjusted_angle).fract() - color_intensity * model.settings.mask;
            },
            AnimationOption::Vortex2 => {
                let rotation_angle = PI* (model.time + spiral.x) * (0.5- radius).max(0.0);
                let adjusted_angle = angle + rotation_angle;
                color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * model.settings.u;
                mask = (spiral.x + model.settings.frequency * adjusted_angle.cos()).fract() - color_intensity * model.settings.mask; 
            },

            AnimationOption::Luminance => {
                color_intensity = 1.0-luminance;

                mask = 2.0*spiral.x.fract() - color_intensity* model.settings.u*2.0;
            },

            AnimationOption::Luminance2 => {
                
                mask = spiral.x.fract() - luminance * model.settings.u*2.0;
            }
            AnimationOption::Luminance3 => {
                color_intensity = 1.0-luminance;
                mask = spiral.x.fract().sin() - color_intensity * model.settings.u*2.0;
            }

            AnimationOption::Spiral => {
                let a = 1.0;
                let b = 1.0;
                let spiral_angle = a + b * angle;
                let spiral_radius = radius * model.settings.u;
                mask = (model.time + spiral_angle).fract() - spiral_radius * model.settings.mask;
            }


            AnimationOption::Default => {
                color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * 1.6;
                mask = spiral.x.fract() - color_intensity * model.settings.mask;
            },
        }

        if mask.abs() < 0.2 {
            let color = match model.settings.color_option {
                ColorOption::Rainbow => nannou::color::hsv(angle / 6.2831, 1.0, 1.0),
                ColorOption::Black => nannou::color::rgb(0.0,0.0,0.0).into(),
                ColorOption::Real => nannou::color::rgb(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                ).into(),
                ColorOption::Ranim => {
                    let hue = (model.time/10.0+ angle/6.2831)%1.0;
                    let saturation = 0.5 + 0.5 * (radius * 2.0).max(0.0).min(1.0);
                    let lightness = 0.4 + 0.4 * (radius * 2.0).max(0.0).min(1.0);
                    nannou::color::hsv(hue, saturation, lightness)
                }
                ColorOption::Test => {
                    let random_value = (x * y) / (model.img.width() as f32 * model.img.height() as f32);                  

                    let progress= (model.time/10.0).sin();                    
                    let hue = 0.5 + 0.5 * ((model.settings.t + progress + random_value) % 1.0).sin();
                    let saturation = progress.fract();
                    let lightness = 0.4 + 0.4 * ((model.settings.t + progress + random_value) % 1.0).cos();
                    nannou::color::hsv(hue, saturation, lightness)
                }};
            
            // draw.rect()
            //     .x_y(
            //         win_rect.left() + x * rect_w,
            //         win_rect.top() - y * rect_h,
            //     )
            //     .w_h(rect_w, rect_h)
            //     .color(color);

            draw.line()
                .start(pt2(
                    win_rect.left() + x * rect_w,
                    win_rect.top() - y * rect_h,
                ))
                .end(pt2(
                    win_rect.left() + x * rect_w + rect_w,
                    win_rect.top() - y * rect_h + rect_h,
                ))
                .weight(model.settings.th)
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

