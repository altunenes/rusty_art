use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    points: Vec<Point2>,
    angle: f32,
    scale: f32,
    egui: Egui,
    settings: Settings,
}
struct Settings{
    ratio : f32,
    radius : f32,
    alpha : f32,
    stroke_weight : f32,
    show_ui: bool,
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
        egui,
        points: Vec::new(),
        angle: 30.0,
        scale: 1.0,
        settings: Settings {
            ratio: 31.0,
            radius: 5.0,
            alpha: 5.0,
            stroke_weight: 1.0,
            show_ui: true,

        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.separator();
        ui.add(egui::Slider::new(&mut model.settings.ratio, 0.0..=100.0).text("scale"));
        ui.add(egui::Slider::new(&mut model.settings.radius, 0.0..=20.0).text("radius"));
        ui.add(egui::Slider::new(&mut model.settings.alpha, 0.0..=10.0).text("alpha"));
        ui.add(egui::Slider::new(&mut model.settings.stroke_weight, 0.0..=100.0).text("stroke_weight"));
    
        if ui.button("Clear").clicked() {
            model.points.clear();
        }
    });
    let golden_angle = (model.settings.ratio +model.settings.alpha.sqrt()) * std::f32::consts::PI;
    let r = model.points.len() as f32 * model.scale;
    let x = r * model.angle.cos();
    let y = r * model.angle.sin();
    let pos = pt2(x, y);
    model.points.push(pos);
    model.angle += golden_angle;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for (i, &pos) in model.points.iter().enumerate() {
        let hue = i as f32 / model.points.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        let radius = (i as f32).sqrt() *model.settings.radius;
        draw.ellipse()
            .xy(pos)
            .radius(radius)
            .color(color)
            .stroke_weight(model.settings.stroke_weight)
            .stroke(color);
    }
    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    }
} 
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}