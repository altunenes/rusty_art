use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    points: Vec<Point2>,
    angle: f32,
    screen_width: f32,
    screen_height: f32,
    settings: Settings,
    egui: Egui,
}
struct Settings{
    energy: f32,
    alpha: f32,
    beta: f32,
    ph: f32,
    x: f32,
    y: f32,
    enes:f32,
    nuro:f32,
    hue_step: f32,
    radius: f32,
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
        points: Vec::new(),
        angle: 45.0,
        screen_width: window.rect().w(),
        screen_height: window.rect().h(),
        egui,
        settings: Settings {
            energy: 0.8,
            alpha: 30.5,
            beta: 3.0,
            ph: 0.1,
            x: 1.0,
            y: 31.0,
            enes: 4.0,
            nuro: 12.0,
            hue_step: 0.01,
            radius: 3.1,

        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("energy:");
        ui.add(egui::Slider::new(&mut model.settings.energy, 0.0..=1.0));
        ui.label("alpha:");
        ui.add(egui::Slider::new(&mut model.settings.alpha, 0.0..=100.0));
        ui.label("beta:");
        ui.add(egui::Slider::new(&mut model.settings.beta, 0.0..=100.0));
        ui.label("ph:");
        ui.add(egui::Slider::new(&mut model.settings.ph, 0.0..=1.0));
        ui.label("x:");
        ui.add(egui::Slider::new(&mut model.settings.x, 0.0..=100.0));
        ui.label("y:");
        ui.add(egui::Slider::new(&mut model.settings.y, 0.0..=100.0));
        ui.label("enes:");
        ui.add(egui::Slider::new(&mut model.settings.enes, 0.0..=100.0));
        ui.label("nuro:");
        ui.add(egui::Slider::new(&mut model.settings.nuro, 0.0..=100.0));
        ui.label("hue_step:");
        ui.add(egui::Slider::new(&mut model.settings.hue_step, 0.0..=1.0));
        ui.label("radius:");
        ui.add(egui::Slider::new(&mut model.settings.radius, 0.0..=20.0));
        if ui.button("Clear").clicked() {
            model.points.clear();
        }
    });
    let golden_angle = (model.settings.beta + 35.0_f32.sqrt()) * std::f32::consts::PI+model.settings.ph;
    let r = model.points.len() as f32 * model.settings.energy;
    let distance_from_center = pt2(model.screen_width / model.settings.nuro, model.screen_height / model.settings.enes).distance(pt2(model.settings.x, model.settings.y));
    let angle_scale = distance_from_center / model.settings.alpha; 
    let angle = model.angle * angle_scale;
    let x = r * angle.cos();
    let y = r * angle.sin();
    let pos = pt2(x, y);
    model.points.push(pos);
    model.angle += golden_angle;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for (i, &pos) in model.points.iter().enumerate() {
        let hue_step = model.settings.hue_step;
        let hue = (i as f32 * hue_step) % 1.0;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        let radius = (i as f32).sqrt() * model.settings.radius;
        draw.ellipse()
            .xy(pos)
            .radius(radius)
            .color(color)
            .stroke_weight(1.3)
            .stroke(WHITE);
        if i > 0 {
            let prev = model.points[i - 1];
            draw.line()
                .start(prev)
                .end(pos)
                .weight(13.0)
                .color(color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}