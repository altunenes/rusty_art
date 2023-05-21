// all appreciation is going to Paul Bourke for his math. All I did was implement it in Rust.
// math: cos(n pi x / L) cos(m pi y / L) - cos(m pi x / L) cos(n pi y / L) = 0
// http://paulbourke.net/geometry/chladni/

use nannou::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use nannou_egui::{self, egui, Egui};

struct Model {
    m: f32,
    n: f32,
    particles: Vec<Point2>,
    egui: Egui,
    settings: Settings,
    hue: f32,

}

struct Settings {
    r: f32,
}

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        r: 1.0,
    };
    let mut rng = rand::thread_rng();
    let m = rng.gen_range(1.0..10.0);
    let n = rng.gen_range(1.0..10.0);
    let particles = vec![pt2(0.0, 0.0); 10000];
    let hue = 0.0; 


    Model { m, n, particles, egui, settings, hue }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("r");
        ui.add(egui::Slider::new(
            &mut settings.r,
            0.1..=2.1,
        ));

        if ui.button("Next").clicked() {
            let mut rng = rand::thread_rng();
            model.m = rng.gen_range(1.0..10.0);
            model.n = rng.gen_range(1.0..10.0);
        }
    });

    let win = app.window_rect();
    let l = win.w().min(win.h());
    for particle in model.particles.iter_mut() {
        let x = random_range(-l / 2.0, l / 2.0);
        let y = random_range(-l / 2.0, l / 2.0);
        let pattern = (model.m * PI * x / l).cos() * (model.n * PI * y / l).cos() 
                    - (model.n * PI * x / l).cos() * (model.m * PI * y / l).cos();
        if pattern.abs() < 0.1 {
            *particle = pt2(x, y);
        }
    }
    model.hue += 0.001;
    if model.hue > 1.0 {
        model.hue -= 1.0;
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings: &Settings = &model.settings;
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(BLACK);
    for particle in model.particles.iter() {
        let hue = map_range(particle.x, -win.w() / 2.0, win.w() / 2.0, 0.0, 1.0);
        let color = hsl(hue, 1.0, 0.5);
    
        draw.ellipse()
            .x_y(particle.x, particle.y)
            .radius(settings.r)
            .color(color);
        draw.ellipse()
            .x_y(-particle.x/4.0, -particle.y/4.0)
            .radius(settings.r)
            .color(color);
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
}
