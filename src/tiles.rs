//reproduction of the work: https://www.shadertoy.com/view/ddVSDV by @sleeping 


use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    _window: window::Id,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    t: f32,
    s: f32,
    r: f32,
    w: f32,
    f: f32,
    show_ui:bool,
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .raw_event(raw_window_event)
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(_window).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        t: 0.8,
        s: 120.0,
        r: 8.0,
        w: 400.0,
        f:60.0,
        show_ui:true,
    };

    Model { _window,
        egui,
        settings,
        scale:1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.t, 0.1..=10.0).text("t"));
        ui.add(egui::Slider::new(&mut settings.s, 0.1..=200.0).text("s"));
        ui.add(egui::Slider::new(&mut settings.r, 5.0..=40.0).text("r"));
        ui.add(egui::Slider::new(&mut settings.w, 0.1..=800.0).text("w"));
        ui.add(egui::Slider::new(&mut settings.f, 0.1..=60.0).text("f"));
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let draw = app.draw().scale(model.scale);
    let time = app.elapsed_frames() as f32 / settings.f;
    draw.background().color(BLACK);
    let scale = settings.s;
    let time_factor = settings.t * time;
    let rect_size = settings.r;
    let window_size = settings.w;
    for i in (0..window_size as usize).step_by(rect_size as usize) {
        for j in (0..window_size as usize).step_by(rect_size as usize) {
            let u = pt2(i as f32 / scale, j as f32 / scale);
            let t = time_factor;
            let r = (u.x + t).ceil() + (u.y + t).ceil();
            let v = if r % 4.0 > 1.0 { u.x } else { u.y };
            let b = if (v + 0.2).fract() < 0.5 { 0.0 } else { 1.0 };
            let color = srgba(1.0 * b, 0.0, 1.0 - b, 1.0);
            draw.rect()
                .x_y(i as f32 - window_size / 2.0 + rect_size / 2.0, j as f32 - window_size / 2.0 + rect_size / 2.0)
                .w_h(rect_size, rect_size)
                .color(color);
        }
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