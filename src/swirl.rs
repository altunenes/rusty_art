//swirl inspired from the https://github.com/willdady/swirlr project but with many modifications to work with nannou, and with a different algorithm for getting the average color between two points
//I wanted to use some animations to make the effect more interesting but I belive I have to use WGPU for accelerated animation but I don't know WGPU yet
use nannou::prelude::*;
use nannou::image::{Rgba, RgbaImage, open};
use std::path::PathBuf;
use std::f64::consts::PI;
use nannou::image::Pixel;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    color: Rgba<u8>,
}

pub fn swirl(source: &RgbaImage, scale: f64) -> Vec<Point> {
    let (img_width, img_height) = (source.width() as f64, source.height() as f64);
    let size = img_width.max(img_height);

    let max_radius = img_width.min(img_height) / 2.0;
    let origin_x = img_width / 2.0;  
    let origin_y = img_height / 2.0; 
    // other code remains the same
    let mut r;
    let turns = 1000.0;
    let mut theta = 0.0;
    let max_angle = turns * 2.0 * PI;
    let a = 0.0;
    let b = 1.2;
    let sample_length = 3.0;
    let mut inner = vec!();
    let mut outer = vec!();
    while theta < max_angle {
        theta += 0.003;
        r = a + b * theta;
        if r >= max_radius {
            break;
        }

        let p0 = Point{
            x: origin_x + r * theta.cos(),
            y: origin_y + r * theta.sin(), 
            color: Rgba([0, 0, 0, 255]), 
        };
        let p1 = Point{
            x: p0.x - (sample_length * 0.5) * theta.cos(),
            y: p0.y - (sample_length * 0.5) * theta.sin(),
            color: Rgba([0, 0, 0, 255]), 
        };
        let p2 = Point{
            x: p0.x + (sample_length * 0.5) * theta.cos(),
            y: p0.y + (sample_length * 0.5) * theta.sin(),
            color: Rgba([0, 0, 0, 255]), 
        };
        let average_rgba = get_average_rgba_between_points(&source, &p1, &p2);
        let luma = average_rgba.to_luma();
        let mut length = ((255.0 - (luma[0] as f64)) / 255.0) * sample_length;
        if length < 1.0 {
            length = 1.0;
        }

        inner.push(Point{
            x: p1.x * img_width / size * scale, // scaled by the factor
            y: p1.y * img_height / size * scale, // scaled by the factor
            color: average_rgba
        });
        outer.push(Point{
            x: p2.x * img_width / size * scale, // scaled by the factor
            y: p2.y * img_height / size * scale, // scaled by the factor
            color: average_rgba
        });
    }
    inner.append(&mut outer);
    inner
}
fn get_average_rgba(pixels: &Vec<&Rgba<u8>>) -> Rgba<u8> {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 0.0;
    for pixel in pixels {
        r += pixel[0] as f64;
        g += pixel[1] as f64;
        b += pixel[2] as f64;
        a += pixel[3] as f64;
    }
    let length = pixels.len() as f64;
    Rgba([
        (r / length).round() as u8,
        (g / length).round() as u8,
        (b / length).round() as u8,
        (a / length).round() as u8
    ])
}
fn get_average_rgba_between_points(image: &RgbaImage, p1: &Point, p2: &Point) -> Rgba<u8> {
    let (x1, y1, x2, y2) = (p1.x as i32, p1.y as i32, p2.x as i32, p2.y as i32);

    // Create an empty vector to store pixels
    let mut pixels = Vec::new();

    // Bresenham's Line Algorithm
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    loop {
        if x >= 0 && x < image.width() as i32 && y >= 0 && y < image.height() as i32 {
            pixels.push(image.get_pixel(x as u32, (image.height() as i32 - 1 - y) as u32));
        }

        if x == x2 && y == y2 { break; }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }

    get_average_rgba(&pixels)
}
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win_rect = app.window_rect();
    let (win_w, win_h) = (win_rect.w() / 4.0, win_rect.h() / 2.0);

    draw.background().color(WHITE);

    for i in 0..model.path.len()-1 {
        let intensity = ((model.path[i].color[0] as f32 
                       + model.path[i].color[1] as f32
                       + model.path[i].color[2] as f32) / 3.0) / 255.0;

        draw.line()
            .start(pt2(model.path[i].x as f32 - win_w, model.path[i].y as f32 - win_h))
            .end(pt2(model.path[i+1].x as f32 - win_w, model.path[i+1].y as f32 - win_h))
            .weight(lerp(0.1, 3.0, intensity))
            .rgb(
                model.path[i].color[0] as f32 / 255.0,
                model.path[i].color[1] as f32 / 255.0,
                model.path[i].color[2] as f32 / 255.0,
            );
    }

    draw.to_frame(app, &frame).unwrap();
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}
struct Model {
    path: Vec<Point>,
    index: usize,  // added index to track progress in path
    last_update: std::time::Instant,  // time of the last index increment


}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let path = swirl(&img, 1.6);

    app.new_window()
        .size((img.width() as f64 * 1.5) as u32, (img.height() as f64 * 1.5) as u32)
        .view(view)
        .build()
        .unwrap();

    Model {
        path,
        index: 0,
        last_update: std::time::Instant::now(),
    }
}

fn main() {
    nannou::app(model)
        .view(view)
        .run();
}