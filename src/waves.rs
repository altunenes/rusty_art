use nannou::noise::{NoiseFn, Perlin}; 
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() { nannou::app(model).update(update).view(view).run(); }
struct Model { 
    points: Vec<Point2>, 
    noise: Perlin,
    points2: Vec<Point2>,
    points3: Vec<Point2>,
    egui: Egui,
    settings: Settings,
    scale:f32,

}
struct Settings{
    a: f64,
    b: f64,
    c:f32,
    d: f32,
    e : f32,
    f: f32,
    show_ui:bool,
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
    let points = Vec::new(); 
    let points2 = Vec::new();
    let points3 = Vec::new();
    let noise = Perlin::new(); Model { scale:1.0, points, noise,points2,points3, egui,  settings: Settings {
        a: 0.01,
        b: 1.11,
        c: 10.0,
        d: 50.0,
        e: 1.0,
        f: 10.0,
        show_ui:true,
    },} }

fn update(app: &App, model: &mut Model, _update: Update) { 
    let egui = &mut model.egui;
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("a:");
        ui.add(egui::Slider::new(
            &mut model.settings.a,
            0.0..=1.0,
        ));
        ui.label("b:");
        ui.add(egui::Slider::new(
            &mut model.settings.b,
            0.0..=0.5,
        ));
        ui.label("c:");
        ui.add(egui::Slider::new(
            &mut model.settings.c,
            0.0..=200.5,
        ));
        ui.label("d:");
        ui.add(egui::Slider::new(
            &mut model.settings.d,
            0.0..=150.5,
        ));
        ui.label("e:");
        ui.add(egui::Slider::new(
            &mut model.settings.e,
            0.0..=200.5,
        ));
        ui.label("f:");
        ui.add(egui::Slider::new(
            &mut model.settings.f,
            0.0..=200.0,
        ));
    });

        let win = app.window_rect(); 
        let t = app.time; 
        model.points.clear(); 
        model.points2.clear();
        model.points3.clear();
        for x in 0..win.w() as i32 { 
            let x = x as f32 - (win.w() / 2.0); 
            let n = model.noise.get([x as f64 * model.settings.a, t as f64 * model.settings.b]) as f32;
            let y = 100.0 * ((x * 0.01 + t) * 2.0 * PI).cos() + n * 50.0; 
            let y2 = model.settings.c * ((x * 0.01 + t) * 2.0 * PI).cos() + n * model.settings.d;
            let y3 = 13.0 * ((x * 0.01 + t) * 4.0 * PI).cos() + n * 1.0;
            model.points.push(pt2(x, y + n * 150.0));
            model.points2.push(pt2(x, y2 + n * model.settings.e));
            model.points3.push(pt2(x, y3 + n * model.settings.f));
        }
    }
fn view(app: &App, model: &Model, frame: Frame) { 
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);
    let t = app.time;
    let r = 0.5 + 0.5 * (t + 0.0).cos();
    let g = 0.5 + 0.5 * (t + 2.0).cos();
    let b = 0.5 + 0.5 * (t + 4.0).cos();
    draw.polyline()
        .weight(10.0)
        .color(rgb(r, g, b))
        .points(model.points.clone());
    draw.polyline()
    .weight(10.0)
    .color(rgb(g, b, r))
    .points(model.points2.clone());
    draw.polyline()
    .weight(10.0)
    .color(rgb(b, r, g))
    .points(model.points3.clone());
    draw.polyline()
    .weight(5.0)
    .color(rgb(r, g, b))
    .points(model.points.clone().iter().map(|p| pt2(p.x, p.y / 2.0)));

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

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        if !cursor_over_egui {
            match delta {
                nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    model.scale *= 1.0 + *y * 0.05;
                    model.scale = model.scale.max(0.1).min(10.0);
                }
                _ => (),
            }
        }
    }
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = _app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}