use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    circles: Vec<(Point2, f32)>,
    egui: Egui,
    settings: Settings,
}

struct Settings{
    angle: f32,
    speed: f32,
    r:f32,
    radius: f32,
    step : f32,
    cstep: usize,
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
        circles: Vec::new(),
        egui,
        settings: Settings {
            angle: 0.0,
            speed: 1.11,
            r: 10.0,
            radius: 20.0,
            step: 1.0,
            cstep: 1000,
            show_ui:true,
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("angle:");
        ui.add(egui::Slider::new(
            &mut model.settings.angle,
            0.0..=1.0,
        ));
        ui.label("speed:");
        ui.add(egui::Slider::new(
            &mut model.settings.speed,
            0.0..=2.5,
        ));
        ui.label("r:");
        ui.add(egui::Slider::new(
            &mut model.settings.r,
            0.0..=100.0,
        ));
        ui.label("radius:");
        ui.add(egui::Slider::new(
            &mut model.settings.radius,
            0.0..=50.0,
        ));
        ui.label("step:");
        ui.add(egui::Slider::new(
            &mut model.settings.step,
            0.0..=5.0,
        ));
        ui.label("cstep:");
        ui.add(egui::Slider::new(
            &mut model.settings.cstep,
            0..=1000,
        ));

        if ui.button("Reset").clicked() {
            model.circles.clear();
            model.settings.angle = 0.0;           
        }
    });
   
    model.settings.angle += model.settings.speed;
    let r = model.settings.angle.sqrt() * model.settings.r;
    let x = r * model.settings.angle.cos();
    let y = r * model.settings.angle.sin();
    let center = pt2(x, y);
    let radius = 1.0 + model.settings.radius * (model.settings.angle % model.settings.step);
    model.circles.push((center, radius));
    if model.circles.len() > model.settings.cstep {
        model.circles.remove(0);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for (i, &(center, radius)) in model.circles.iter().enumerate() {
        let hue = i as f32 / model.circles.len() as f32;
        let color = hsla(hue, 1.0, 0.5, 1.0);
        draw.ellipse()
            .xy(center)
            .radius(radius)
            .color(color)
            .stroke_weight(2.0)
            .stroke(BLACK);
    }
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
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