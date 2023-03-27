use nannou::prelude::*;
use image::{ImageBuffer, Rgb};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .fullscreen()
        .run();
}

struct Model {
    angle: f32,
    scale: f32,
    img: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

fn model(app: &App) -> Model {
    let window_size = app.window_rect().w();
    let img = ImageBuffer::from_pixel(window_size as u32, window_size as u32, Rgb([0, 0, 0]));
    Model {
        angle: 0.0,
        scale: 5.0,
        img,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let golden_angle = (1.0 + 5.0_f32.sqrt()) * std::f32::consts::PI;
    let r = model.angle * model.scale;
    let x = r * model.angle.cos();
    let y = r * model.angle.sin();
    let pos = pt2(x, y);
    let color = hsla(model.angle % 1.0, 1.0, 0.5, 1.0);
    draw_circle(&mut model.img, pos.x as u32, pos.y as u32, 5, color);
    model.angle += golden_angle;
}

fn draw_circle(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, x: u32, y: u32, radius: u32, color: Hsla) {
    for i in (x - radius)..(x + radius) {
        for j in (y - radius)..(y + radius) {
            let dx = (i as i32 - x as i32) as f32;
            let dy = (j as i32 - y as i32) as f32;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance <= radius as f32 {
                let pixel = img.get_pixel_mut(i, j);
                let rgb = color.to_rgb();
                pixel[0] = (rgb.0[0] * 255.0) as u8;
                pixel[1] = (rgb.0[1] * 255.0) as u8;
                pixel[2] = (rgb.0[2] * 255.0) as u8;
            }
        }
    }
}


fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let texture = app
        .new_texture(model.img.clone())
        .unwrap();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}

