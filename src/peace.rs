use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    num_circles: usize,
    num_points: usize,
    circle_points: Vec<Vec<Point2>>,
    settings: Settings,
    egui: Egui,
}

struct Settings {
    num_circles: usize,
    const_: f32,
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
    let frequency = 125.0;
    let amplitude = 1.4;
    let phase = 1.0;
    let num_points = 155;
    let num_circles = 255;
    let window_rect = app.window_rect();
    let center = window_rect.xy();
    let radius = window_rect.w().min(window_rect.h()) / 2.0;
    let circle_radius = radius / (num_circles as f32);
    let mut circle_points = Vec::with_capacity(num_circles);

    for i in 0..num_circles {
        let mut points = Vec::with_capacity(num_points);
        for j in 0..num_points {
            let angle = j as f32 * 4.0 * PI / (num_points as f32);
            let x = center.x + angle.sin() * circle_radius * (i as f32 + 2.0);
            let y = center.y + angle.cos() * circle_radius * (i as f32 + 2.0);
            points.push(pt2(x, y));
        }
        circle_points.push(points);
    }
    Model {
        frequency,
        amplitude,
        phase,
        num_circles,
        num_points,
        circle_points,
        egui,
        settings: Settings { num_circles: 255, const_: 25.0}
        ,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Num Circles:");
        ui.add(egui::Slider::new(&mut model.settings.num_circles, 1..=255));
        ui.label("Const:");
        ui.add(egui::Slider::new(&mut model.settings.const_, 1.0..=255.0));
    });
    for i in 0..model.num_circles {
        for j in 0..model.num_points {
            let x = model.circle_points[i][j].x
                + (-model.settings.const_ * PI * model.frequency * j as f32 / model.num_points as f32 + model.phase)
                    .sin()
                    * model.amplitude;
            let y = model.circle_points[i][j].y
                + (model.settings.const_ * PI * model.frequency * j as f32 / model.num_points as f32 + model.phase)
                    .cos()
                    * model.amplitude;
            model.circle_points[i][j] = pt2(x, y);
        }
    }
    model.phase += 0.01;
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let draw = app.draw();
    draw.background().color(WHITE);
    for i in 0..settings.num_circles {
        let hue = i as f32 / settings.num_circles as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.polyline()
            .weight(1.5)
            .points(model.circle_points[i].clone())
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