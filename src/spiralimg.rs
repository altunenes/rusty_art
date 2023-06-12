//code is still a work in progress, so there are some comment sections for future improvements and changes
//major revision will be done in order to make it more efficient

use nannou::prelude::*;
use nannou::image::{open, RgbaImage};
use std::path::PathBuf;
use nannou::image::Pixel;

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

enum ColorOption {
    Rainbow,
    Real,
}

struct Model {
    img: RgbaImage,
    time: f32,
    delay_time: f32,
    cycle_completed: bool,
    color_option: ColorOption,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/ferris2.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let _w_id = app.new_window().size(img.width(), img.height()).view(view).build().unwrap();
    
    Model {
        img,
        time: 0.0,
        delay_time: 0.0,
        cycle_completed: false,
        color_option: ColorOption::Rainbow, // Use Rainbow or Real
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    if model.delay_time > 0.0 {
        model.delay_time -= update.since_last.as_secs_f32();
        if model.delay_time <= 0.0 {
            model.time = 0.0;
            model.cycle_completed = false;
        }
    } else {
        model.time += update.since_last.as_secs_f32();
        if model.time >= 50.0 {
            model.cycle_completed = true;
            model.delay_time = 3.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    let win_rect = app.window_rect();
    let image_aspect_ratio = model.img.width() as f32 / model.img.height() as f32;

    let rect_w = win_rect.w() / model.img.width() as f32;
    let rect_h = win_rect.h() / model.img.height() as f32;

    for (x, y, pixel) in model.img.enumerate_pixels() {
        let x = x as f32;
        let y = y as f32;

        // Normalize and shift coordinates so that (0,0) is at the center and y axis is -1 to 1 while keeping the aspect ratio
        let uv = vec2(
            (x / model.img.width() as f32 - 0.5) * image_aspect_ratio,
            y / model.img.height() as f32 - 0.5,
        );

        // Cartesian to polar conversion
        let angle = uv.y.atan2(uv.x);
        let radius = uv.length();

        // Generate the spiral coordinates
        let spiral = vec2(
            angle / 6.2831 + model.time * 1.0 - radius * 40.0, 
            radius,
        );
        // Calculate the color intensity
        let color_intensity = pixel.channels().iter().map(|&c| c as f32 / 255.0).sum::<f32>() * 1.6;

        // Use the fractional part of the spiral's x coordinate for the mask
        let mask = spiral.x.fract() - color_intensity * 0.3;

        // Only draw the pixel if it meets the mask threshold
        if mask.abs() < 0.2 {
            // The color to be used
            let color = match model.color_option {
                ColorOption::Rainbow => nannou::color::hsv(angle / 6.2831, 1.0, 1.0),
                ColorOption::Real => nannou::color::rgb(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                ).into(),
            };

            // Draw the pixel
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
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    }
}