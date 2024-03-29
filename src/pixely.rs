use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    lines: Vec<(Point2, Point2)>,
    speed : f32,
    egui: Egui,
    settings: Settings,
    
}

struct Settings {
    angle: f32,
    line_length: f32,
     r: f32,
    line_count: usize,
    stroke_weight: f32,
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
    Model {
        lines: Vec::new(),
        egui,
        speed: 1.05,
        settings: Settings {
            angle: 0.0,
            line_length: 15.0,
            r: 10.0,
            line_count: 1000,
            stroke_weight: 5.0,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("angle:");
        ui.add(egui::Slider::new(&mut model.settings.angle, 0.0..=1000.0));
        ui.label("r:");
        ui.add(egui::Slider::new(&mut model.settings.r, 0.0..=100.0));
        ui.label("line_length:");
        ui.add(egui::Slider::new(&mut model.settings.line_length, 0.0..=100.0));
        ui.label("r_threshold:");
        ui.add(egui::Slider::new(&mut model.settings.line_count, 0..=5000));
        ui.label("stroke_weight:");
        ui.add(egui::Slider::new(&mut model.settings.stroke_weight, 0.0..=300.0));
        if ui.button("Restart").clicked() {
            model.lines.clear();
            model.settings.angle = 0.0;
        }
    });
    model.settings.angle += model.speed;
    let r = model.settings.angle.sqrt() * model.settings.r;
    let x = r * model.settings.angle.cos();
    let y = r * model.settings.angle.sin();
    let start = pt2(x, y);
    let end = pt2(x + model.settings.line_length, y +model.settings.line_length);
    model.lines.push((start, end));
    if model.lines.len() > model.settings.line_count {
        model.lines.remove(0);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for (i, &(start, end)) in model.lines.iter().enumerate() {
        let hue = i as f32 / model.lines.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.line()
            .start(start)
            .end(end)
            .color(color)
            .stroke_weight(model.settings.stroke_weight);
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