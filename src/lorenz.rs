use nannou::prelude::*;
use std::collections::VecDeque;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    z: f32,
    trail: VecDeque<Point3>,
    egui: Egui,
    settings: Settings,
}

struct  Settings{
    speed:usize,
    draw_mode: DrawMode, 
}
#[derive(PartialEq)]
enum DrawMode {
    Lines,
    Ellipses,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800,800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Model {
        x: 0.1,
        y: 0.0,
        z: 0.0,
        egui,
        trail: VecDeque::with_capacity(5000),
        settings: Settings {
            speed: 5,
            draw_mode: DrawMode::Ellipses,  // Use the DrawMode enum variant here
        },
    }
}
fn update_lorenz(x: &mut f32, y: &mut f32, z: &mut f32, dt: f32) {
    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0 / 3.0;

    let dx = sigma * (*y - *x);
    let dy = *x * (rho - *z) - *y;
    let dz = *x * *y - beta * *z;

    *x += dx * dt;
    *y += dy * dt;
    *z += dz * dt;
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("SPEED:");
        ui.add(egui::Slider::new(
            &mut model.settings.speed,
            0..=10,
        ));
        ui.group(|ui| {
            ui.label("Draw Mode:");
            ui.radio_value(&mut model.settings.draw_mode, DrawMode::Lines, "Lines");
            ui.radio_value(&mut model.settings.draw_mode, DrawMode::Ellipses, "Ellipses");
        });
        if ui.button("Restart").clicked() {
            model.x = 0.1;
            model.y = 0.0;
            model.z = 0.0;
            model.trail.clear();
        }
    });

    for _ in 0..model.settings.speed {
        update_lorenz(&mut model.x, &mut model.y, &mut model.z, 0.01);
        model.trail.push_back(pt3(model.x, model.y, model.z));
        if model.trail.len() > 5000 {
            model.trail.pop_front();
        }
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let mut iter = model.trail.iter();
    if let Some(mut last_point) = iter.next().cloned() {
        for &point in iter {
            let x1 = map_range(last_point.x, -30.0, 30.0, -400.0, 400.0);
            let y1 = map_range(last_point.y, -30.0, 30.0, -400.0, 400.0);
            
            let x2 = map_range(point.x, -30.0, 30.0, -400.0, 400.0);
            let y2 = map_range(point.y, -30.0, 30.0, -400.0, 400.0);
            
            let hue = map_range(point.z, 0.0, 50.0, 0.0, 1.0);
            if model.settings.draw_mode == DrawMode::Lines {
                draw.line()
                    .start(pt2(x1, y1))
                    .end(pt2(x2, y2))
                    .color(hsla(hue, 0.6, 0.6, 0.5));
            } else {
                draw.ellipse()
                    .x_y(x2, y2)
                    .radius(1.0)
                    .color(hsla(hue, 0.6, 0.6, 0.5));
            }

            last_point = point;
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();

    if app.keys.down.contains(&Key::Space) {
        let filepath = app.project_path().expect("failed to locate").join("frames").join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(filepath);
    }
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}