use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    egui: Egui,
    t: f32,
    settings: Settings,
    x: f32,
    y: f32,
}
struct Settings {

    a: f32,
    b: f32,
    c: f32,
    d: f32,
    trail_length: f32,
    time: f32,
    radius: f32,
    t_factor: f32,
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
        t: 0.0,
        egui,
        x: 0.0,
        y: 0.0,
        settings: Settings {
            a: -0.45,
            b: -0.80,
            c: -1.60,
            d: 2.0,
            trail_length: 300.0,
            time: 200.0,
            radius: 1.0,
            t_factor: 400.05,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {   
        ui.label("a:");
        ui.add(egui::Slider::new(&mut model.settings.a, -5.0..=5.0));
        ui.label("b:");
        ui.add(egui::Slider::new(&mut model.settings.b, -5.0..=5.0));
        ui.label("c:");
        ui.add(egui::Slider::new(&mut model.settings.c, -5.0..=5.0));
        ui.label("d:");
        ui.add(egui::Slider::new(&mut model.settings.d, -5.0..=5.0));
        ui.label("trail_length:");
        ui.add(egui::Slider::new(&mut model.settings.trail_length, 0.0..=1000.0));
        ui.label("time:");
        ui.add(egui::Slider::new(&mut model.settings.time, 0.0..=1000.0));
        ui.label("radius:");
        ui.add(egui::Slider::new(&mut model.settings.radius, 0.0..=40.0));
        ui.label("pattern:");
        ui.add(egui::Slider::new(&mut model.settings.t_factor, 0.0..=5000.0));

    });
        model.t = _app.elapsed_frames() as f32 / model.settings.time;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let trail_length = model.settings.trail_length;
    let mut x = model.x;
    let mut y = model.y;
    for i in 0..trail_length as usize {
        let t = model.t - (i as f32 * model.settings.t_factor); 
        let x_prev = x;
        let y_prev = y;
        x = x_prev.sin() * model.settings.a + model.settings.c * x_prev.cos() * t.sin();
        y = y_prev.sin() * model.settings.b  +model.settings.d * y_prev.cos() * t.sin();
        let x_mapped = map_range(x, -2.0, 2.0, -300.0, 300.0); 
        let y_mapped = map_range(y, -2.0, 2.0, -300.0, 300.0);
        draw.ellipse()
            .x_y(x_mapped, y_mapped)
            .w_h(4.0, 4.0)
            .color(WHITE)
            .radius(model.settings.radius);
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