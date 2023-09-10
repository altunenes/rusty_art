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
    restart: bool,

}
#[allow(dead_code)]
struct Settings {
    use_real_colors: bool,
    colors: usize,
    r: f32,
    shape: usize,
    s: usize,
    speed: f32,

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
        r: 1.0,
        shape: 2,
        s:50,
        speed: 1.0,

    };
    let mut rng = rand::thread_rng();
    let points: Vec<Point> = (0..100)
        .map(|_| Point {
            x: rng.gen_range(0.0..800.0),
            y: rng.gen_range(0.0..800.0),
        })
        .collect();
    let img_path = get_image_path("images/ferris2.jpg");
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
        restart: false,

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
            settings.colors = (settings.colors % 6) + 1;
        }
        ui.label(format!("shape {}", settings.shape));
        if ui.button("Next shape mode").clicked() {
            settings.shape = (settings.shape % 5) + 1;
        }
        ui.label(format!("sampling {}", settings.s));
        ui.add(egui::Slider::new(&mut settings.s, 1..=200).text("sampling"));
        if ui.button("Restart").clicked() {
            model.restart = true;
        }
        ui.add(egui::Slider::new(&mut settings.r, 0.01f32..=10.0f32).text("r"));
        ui.add(egui::Slider::new(&mut settings.speed, 0.0f32..=1.0f32).text("s"));
    });
    if model.counter % 20 == 0 { 
        if rand::random::<f32>() < settings.speed {
            model.counter += 1;
            let mut rng = rand::thread_rng();
            let new_points: Vec<Point> = (0..model.settings.s)
                .map(|_| Point {
                    x: rng.gen_range(0.0..800.0),
                    y: rng.gen_range(0.0..800.0),
                })
                .collect();
            model.points.extend(new_points);
        }
    } else {
        model.counter += 1;
    }      
    for (current, target) in model.current_points.iter_mut().zip(&model.target_points) {
        current.x += (target.x - current.x) * model.lerp_factor;
        current.y += (target.y - current.y) * model.lerp_factor;
    }
    if model.restart {
        model.points.clear();
        model.restart = false; 
    }

}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    let draw = app.draw().scale(model.zoom);

    let points_as_tuples: Vec<(f64, f64)> = model.points.iter().map(|p| (p.x, p.y)).collect();
    let diagram = VoronoiDiagram::<Point>::from_tuple(&(0.0, 0.0), &(800.0, 800.0), &points_as_tuples).unwrap();

    for cell in diagram.cells() {
        let mut cell_points: Vec<Point2> = cell.points().iter()
            .map(|p| {
                let x: f32 = (p.x as f32 - model.img_width as f32 / 2.0) * model.scale;
                let y: f32 = ((model.img_height - p.y as u32) as f32 - model.img_height as f32 / 2.0) * model.scale;
                pt2(x, y)
            })
            .collect();
        cell_points = cell_points.into_iter()
            .filter(|pt| pt.x.abs() <= 400.0 && pt.y.abs() <= 400.0)
            .collect();
        let centroid = calculate_centroid(cell);
        let img_x = centroid.x as u32;
        let img_y = centroid.y as u32;

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
                2 => rgba(r, g, b, 1.0),
                3 => {
                    let intensity = (r + g + b) / 3.0;
                    rgba(intensity, 0.0, 1.0 - intensity, 1.0)
                }
                4 => {
                    let intensity = (r + g + b) / 3.0;
                    let phase = 2.0 * std::f32::consts::PI * intensity;
                    let r = (phase.sin() * 0.5 + 0.5) as f32;
                    let g = ((phase + 2.0 * std::f32::consts::PI / 3.0).sin() * 0.5 + 0.5) as f32;
                    let b = ((phase + 4.0 * std::f32::consts::PI / 3.0).sin() * 0.5 + 0.5) as f32;
                    rgba(r, g, b, 1.0)
                }
                5 => {
                    let r_comp = 1.0 - r;
                    let g_comp = 1.0 - g;
                    let b_comp = 1.0 - b;
                    rgba(r_comp, g_comp, b_comp, 1.0)
                }
                6 => {
                    let intensity = 0.2126 * r + 0.7152 * g + 0.0722 * b;
                    let hue = intensity * 1.0;
                    let hsv_color = hsv(hue, 1.0, 1.0);
                    let rgb_color = Rgb::from(hsv_color);
                    rgba(rgb_color.red, rgb_color.green, rgb_color.blue, 1.0)
                }
                _ => unreachable!(),
            };

            let x: f32 = (centroid.x as f32 - model.img_width as f32 / 2.0) * model.scale;
            let y: f32 = ((model.img_height - centroid.y as u32) as f32 - model.img_height as f32 / 2.0) * model.scale;

            let radius = model.settings.r;
            match model.settings.shape {
                1 => {
                    draw.ellipse()
                        .x_y(x, y)
                        .radius(radius)
                        .color(color);
                },
                2 => {
                    draw.polyline()
                        .stroke_weight(radius)
                        .points(cell_points)
                        .color(color);
                },
                3 => {
                    if let Some(first_point) = cell_points.first() {
                        draw.line()
                            .start(pt2(x, y))
                            .end(*first_point)
                            .color(color)
                            .weight(radius);
                    }
                },
                4 => {
                    if cell_points.len() >= 3 {
                        draw.tri()
                        .stroke_weight(radius)
                            .points(cell_points[0], cell_points[1], cell_points[2])
                            .color(color);
                    }
                },
                5 => {

                    draw.polygon()
                        .stroke_weight(radius)
                        .points(cell_points)
                        .color(color);
                },
                _ => {},
            }
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