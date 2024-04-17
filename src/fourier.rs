use nannou::prelude::*;
use std::cell::RefCell;
use std::f32::consts::PI;
use nannou_egui::{self, egui, Egui};
use nannou::noise::Perlin;
use nannou::noise::NoiseFn;

fn main() {
    nannou::app(model).update(update).view(view).run();
}
#[allow(dead_code)]
struct Model {
    wave: RefCell<Vec<Point2>>,
    egui: Egui,
    settings: Settings,
    time : f32,
    perlin_noise: Perlin,
    scale: f32,

}
struct Settings {
    num_harmonics: usize,
    speed: f32,
    n1: usize,
    n2: usize,
    r1: f32,
    x: f32,
    y: f32,
    s_size: f32,
    use_noise: bool,
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
    Model {
        wave: RefCell::new(Vec::new()),
        egui,
        time: 0.0,
        perlin_noise: Perlin::new(),
        scale:1.0,
        settings: Settings {
            num_harmonics: 4,
            speed: 4.05,
            n1: 2,
            n2: 1,
            r1: 75.0,
            x: 800.0,
            y: 300.0,
            s_size: 1.0,
            use_noise: false,
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
        ui.label("num_harmonics:");
        ui.add(egui::Slider::new(
            &mut model.settings.num_harmonics,
            1..=100,
        ));
        ui.label("n1:");
        ui.add(egui::Slider::new(&mut model.settings.n1, 1..=100));
        ui.label("n2:");
        ui.add(egui::Slider::new(&mut model.settings.n2, 1..=10));
        ui.label("r1:");
        ui.add(egui::Slider::new(&mut model.settings.r1, 1.0..=100.0));
        ui.label("x:");
        ui.add(egui::Slider::new(&mut model.settings.x, 1.0..=2000.0));
        ui.label("y:");
        ui.add(egui::Slider::new(&mut model.settings.y, 1.0..=1000.0));
        ui.label("s_size:");
        ui.add(egui::Slider::new(&mut model.settings.s_size, 1.0..=10.0));
        ui.label("speed:");
        ui.add(egui::Slider::new(&mut model.settings.speed, 0.0..=4.05));
        ui.checkbox(&mut model.settings.use_noise, "Use noise");

    });
            model.time += 0.01 * model.settings.speed;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    let win = app.window_rect();
    let win_center = pt2(win.w() / 2.0, win.h() / 2.0);
    let center = win_center - pt2(model.settings.x, model.settings.y);

    draw.background().color(BLACK);

    let mut x = 0.0;
    let mut y: f32 = 0.0;
    let perlin = Perlin::new();
    
    for i in 0..model.settings.num_harmonics {
        let prev_x = x;
        let prev_y = y;
    
        let n = i * model.settings.n1 + model.settings.n2;
        let radius = model.settings.r1 * (4.0 / (n as f32 * PI));
    
        if model.settings.use_noise {
            let noise_value = perlin.get([model.time as f64 * 0.01, i as f64 * 0.1]) as f32;
            x += radius * (n as f32 * model.time * noise_value).cos();
            y += radius * (n as f32 * model.time * noise_value).sin();
        } else {
            x += radius * (n as f32 * model.time).cos();
            y += radius * (n as f32 * model.time).sin();
        }
    
        let color = hsla(
            i as f32 / model.settings.num_harmonics as f32,
            1.0,
            0.5,
            1.0,
        );
    
        draw.ellipse()
        .xy(center + pt2(prev_x, prev_y))
        .radius(radius)
        .no_fill()
        .stroke_color(color)
        .stroke_weight(4.0);
        draw.line()
            .start(center + pt2(prev_x, prev_y))
            .end(center + pt2(x, y))
            .color(WHITE);
    }
    model.wave.borrow_mut().insert(0, pt2(x, y));
    if model.wave.borrow().len() > 1000 {
        model.wave.borrow_mut().pop();
    }
    let wave_start = center + pt2(model.settings.x /4.0, 0.0);
    draw.line()
        .start(center + pt2(x, y))
        .end(wave_start + pt2(0.0, model.wave.borrow()[0].y))
        .color(WHITE);
    let points: Vec<_> = model
        .wave
        .borrow()
        .iter()
        .enumerate()
        .map(|(i, point)| wave_start + pt2(i as f32, point.y))
        .collect();
    draw.polyline().stroke_weight(model.settings.s_size).points(points).color(WHITE);
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