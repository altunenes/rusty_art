use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use voronator::VoronoiDiagram;
use voronator::delaunator::Point;
use rand::prelude::*;
use voronator::polygon::Polygon;
use nannou::image::{open, DynamicImage, GenericImageView};
use std::path::PathBuf;
struct Model {
    egui: Egui,
    zoom: f32,
    scale: f32,
    settings: Settings,
    points: Vec<Point>,
    img: DynamicImage,
    img_width: u32,
    img_height: u32,
    counter: usize,
    current_points: Vec<Point>,
    target_points: Vec<Point>,
    lerp_factor: f64,
}
#[allow(dead_code)]
struct Settings {
    use_real_colors: bool,
    colors: usize,
    min_radius: f32,
    max_radius: f32,
}
fn main() {
    nannou::app(model).update(update).run();
}
fn model(app: &App) -> Model {
    let _w_id = app
        .new_window()
        .size(800, 800) 
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        colors: 1,
        use_real_colors: false,
        min_radius: 0.05,
        max_radius: 2.0,
    };
    let mut rng = rand::thread_rng();
    let points: Vec<Point> = (0..100)
        .map(|_| Point {
            x: rng.gen_range(0.0..800.0),
            y: rng.gen_range(0.0..800.0),
        })
        .collect();
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgba8();
    let img_width = img.width();
    let img_height = img.height();
    Model {
        scale: 1.0,
        egui,
        settings,
        zoom: 1.0,
        points,
        img: DynamicImage::ImageRgba8(img),
        img_width,
        img_height,
        counter: 0,
        current_points: Vec::new(),
        target_points: Vec::new(),
        lerp_factor: 0.0,
    }
}
fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label(format!("color {}", settings.colors));
        if ui.button("next").clicked() {
            settings.colors = (settings.colors % 3) + 1;
        }
        ui.add(egui::Slider::new(&mut settings.min_radius, 0.01f32..=10.0f32).text("min radius"));
        ui.add(egui::Slider::new(&mut settings.max_radius, 0.01f32..=10.0f32).text("max radius"));
    });
    model.counter += 100;
        let mut rng = rand::thread_rng();
        let new_points: Vec<Point> = (0..50)
            .map(|_| Point {
                x: rng.gen_range(0.0..800.0),
                y: rng.gen_range(0.0..800.0),
            })
            .collect();
        model.points.extend(new_points);
    for (current, target) in model.current_points.iter_mut().zip(&model.target_points) {
        current.x += (target.x - current.x) * model.lerp_factor;
        current.y += (target.y - current.y) * model.lerp_factor;
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw().scale(model.zoom);
let points_as_tuples: Vec<(f64, f64)> = model.points.iter().map(|p| (p.x, p.y)).collect();
let diagram = VoronoiDiagram::<Point>::from_tuple(&(0.0, 0.0), &(800.0, 800.0), &points_as_tuples).unwrap();
let min_radius = model.settings.min_radius;
let max_radius = model.settings.max_radius;
for cell in diagram.cells() {
    let centroid = calculate_centroid(cell);
    let img_x = centroid.x as u32;
    let img_y = centroid.y as u32;
    let distance_to_center = ((centroid.x - 400.0).powi(2) + (centroid.y - 400.0).powi(2)).sqrt();
    let normalized_distance = distance_to_center / (800.0 * f64::sqrt(2.0) / 2.0);
    let radius = map_range(normalized_distance as f32, 0.0, 1.0, min_radius, max_radius);
    if img_x < model.img_width && img_y < model.img_height {
        let pixel = model.img.get_pixel(img_x, img_y);
        let r = pixel.0[0] as f32 / 255.0;
        let g = pixel.0[1] as f32 / 255.0;
        let b = pixel.0[2] as f32 / 255.0;

        let color = match model.settings.colors {
            1 => {
                let grayscale = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                rgba(grayscale, grayscale, grayscale, 1.0)
            }
            2 => { 
                rgba(r, g, b, 1.0)
            }
            3 => {  
                let intensity = (r + g + b) / 3.0;
                rgba(intensity, 0.0, 1.0 - intensity, 1.0) 
            }
            _ => unreachable!(),
        };
        let x: f32 = (centroid.x as f32 - model.img_width as f32 / 2.0) * model.scale;
        let y: f32 = ((model.img_height - centroid.y as u32) as f32 - model.img_height as f32 / 2.0) * model.scale;
        draw.ellipse()
            .x_y(x, y)
            .radius(radius) 
            .color(color);
    }
}
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
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
fn calculate_centroid(cell: &Polygon<Point>) -> Point {
    let points = cell.points();
    let mut area = 0.0;
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..points.len() {
        let xi = points[i].x;
        let yi = points[i].y;
        let xi1 = points[(i+1) % points.len()].x;
        let yi1 = points[(i+1) % points.len()].y;
        let a = xi * yi1 - xi1 * yi;
        area += a;
        cx += (xi + xi1) * a;
        cy += (yi + yi1) * a;
    }
    area *= 0.5;
    cx /= 6.0 * area;
    cy /= 6.0 * area;
    Point { x: cx, y: cy }
}