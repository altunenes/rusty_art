use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    dot_size: f32,
    animation_paused: bool,
    settings: Settings,
    egui: Egui,
}
struct Settings {
    left_circle_color: Srgb<u8>,
    right_circle_color: Srgb<u8>,
    circle_color: Srgb<u8>,
    clear: bool,
    n_dots: f32,
    r1: f32,
    r2: f32,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model { dot_size: 20.0, animation_paused: false, egui,
        settings: Settings {
            left_circle_color:PURPLE,
            right_circle_color:YELLOW,
            circle_color: RED,
            clear: false,
            n_dots: 120.0,
            r1: 0.45,
            r2: 120.0,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("circle_color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.circle_color = Srgb::new(rand::random(), rand::random(), rand::random());
        }
        ui.label("left_circle_color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.left_circle_color = Srgb::new(rand::random(), rand::random(), rand::random());
        }
        ui.label("right_circle_color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.right_circle_color = Srgb::new(rand::random(), rand::random(), rand::random());
        }
        ui.separator();
        let clicked = ui.button("Clear").clicked();
        if clicked {
            model.settings.clear = !model.settings.clear;
        }
        ui.label("n_dots:");
        ui.add(egui::Slider::new(&mut model.settings.n_dots, 0.1..=240.0).text("n_dots"));
        ui.label("r1:");
        ui.add(egui::Slider::new(&mut model.settings.r1, 0.1..=1.0).text("r1"));
        ui.label("r2:");
        ui.add(egui::Slider::new(&mut model.settings.r2, 0.1..=240.0).text("r2"));

    });
    if !model.animation_paused {
        model.dot_size = 0.35 * (0.5 * _app.time.sin().powi(4) + 0.5);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.rect().x_y(-win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(model.settings.right_circle_color);
    draw.rect().x_y(win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(model.settings.left_circle_color);
    draw.ellipse()
        .x_y(win.w() / 7.0, 0.0)
        .radius(model.settings.r1 * win.h() / 2.0)
        .color(model.settings.circle_color);
    draw.ellipse()
        .x_y(-win.w() / 7.0, 0.0)
        .radius(model.settings.r1 * win.h() / 2.0)
        .color(model.settings.circle_color);
if !model.settings.clear {
    for x in 0..=model.settings.n_dots as usize {
        for y in 0..=model.settings.n_dots as usize {
            let uv = pt2(
                (x as f32) / model.settings.n_dots * win.w() - win.right() / 1.0,
                (y as f32) / model.settings.n_dots * win.h() - win.top() / 1.0,
            );
            let dot_color = if uv.x < 0.0 { model.settings.left_circle_color } else { model.settings.right_circle_color };
            draw.ellipse()
                .xy(uv)
                .radius(model.dot_size * win.w() / model.settings.r2)
                .color(dot_color);
        }
    }
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
    use nannou::winit::event::{ElementState, WindowEvent};
    model.egui.handle_raw_event(event);
    if let WindowEvent::MouseInput { button, state, .. } = event {
        if *button == MouseButton::Left && *state == ElementState::Pressed {
            model.animation_paused = !model.animation_paused;
        }
    }
}