use nannou::prelude::*;
use nannou::image::{open, RgbaImage};
use rand::{thread_rng, Rng};
use std::path::PathBuf;
fn main() {
    nannou::app(model).update(update).run();
}
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}
struct Particle {
    x: usize,
    y: usize,
    velocity: f32,
    size: f32,
}
impl Particle {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let velocity = rng.gen_range(0.5..3.0);
        let size = rng.gen_range(0.5..1.0);
        Self { x, y, velocity, size }
    }
    fn update(&mut self, brightness_map: &[Vec<f32>]) {
        let speed = brightness_map[self.y][self.x];
        let delta_y = 2.0 - speed + self.velocity;
        self.y = ((self.y as f32) + delta_y) as usize % brightness_map.len();
    }
}
struct Model {
    img: RgbaImage,
    particles: Vec<Particle>,
    brightness_map: Vec<Vec<f32>>,
}
fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    let window_width = 1920;
    let window_height = 1080;
    let mut brightness_map = vec![vec![0.0; width as usize]; height as usize];
    for (x, y, pixel) in img.enumerate_pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let brightness = relative_brightness(r, g, b);
        brightness_map[y as usize][x as usize] = brightness;
    }
    let particles = (0..10000).map(|_| Particle::new(width as usize, height as usize)).collect();
    let _window_id = app
        .new_window()
        .size(window_width, window_height)
        .view(view)
        .build()
        .unwrap();

    Model {
        img,
        particles,
        brightness_map,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    for particle in &mut model.particles {
        particle.update(&model.brightness_map);
    }
}
pub fn relative_brightness(r: f32, g: f32, b: f32) -> f32 {
    ((r * r * 0.229) + (g * g * 0.587) + (b * b * 0.114)).sqrt() / 100.0
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let (img_width, img_height) = model.img.dimensions();
    let scale_factor = win.w().min(win.h()) / img_width.max(img_height) as f32;
    draw.background().color(BLACK);
    for particle in &model.particles {
        let pixel = model.img.get_pixel(particle.x as u32, particle.y as u32);
        let brightness = model.brightness_map[particle.y][particle.x];
        let color = srgba(pixel[0] as f32 / 255.0, pixel[1] as f32 / 255.0, pixel[2] as f32 / 255.0, brightness * 0.3);
        let x = (particle.x as f32 - (img_width / 2) as f32) * scale_factor;
        let y = ((img_height - particle.y as u32) as f32 - (img_height / 2) as f32) * scale_factor;

        draw.ellipse()
            .x_y(x, y)
            .radius(particle.size * scale_factor)
            .color(color);
    }
    draw.to_frame(app, &frame).unwrap();
}