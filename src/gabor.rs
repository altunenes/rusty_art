use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    time: f32,
    settings: Settings,
    egui: Egui,
    scale: f32,
    pause: bool, 

}

struct Settings {
    sigma: f32,
    t: f32,
    u: f32,
    v: f32,
    kx: f32,
    ky: f32,
    c: usize,
    shape: usize,



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
}


fn main() {
    nannou::app(model)
        .update(update)
        .run();
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
    let settings = Settings {
        sigma: 128.0,
        t:30.0,
        u: 128.0,
        v: 128.0,
        kx: 12.0,
        ky: 5.0,
        c: 1,
        shape: 1,

    };

    Model { time: 0.0, settings, egui, scale: 1.0, pause: false }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.sigma, 0.0..=256.0).text("sigma"));
        ui.add(egui::Slider::new(&mut settings.t, 0.0..=360.0).text("theta"));
        ui.add(egui::Slider::new(&mut settings.u, 50.0..=256.0).text("u"));
        ui.add(egui::Slider::new(&mut settings.v, 50.0..=256.0).text("v"));
        ui.add(egui::Slider::new(&mut settings.kx, 0.0..=256.0).text("kx"));
        ui.add(egui::Slider::new(&mut settings.ky, 0.0..=256.0).text("ky"));
        ui.label(format!("color {}", settings.c));
        if ui.button("Next color mode").clicked() {
            settings.c = (settings.c % 13) + 1;
        }
        ui.label(format!("shape {}", settings.shape));
        if ui.button("Next shape mode").clicked() {
            settings.shape = (settings.shape % 3) + 1;
        }
        if ui.button("Pause/Resume Animation").clicked() {
            model.pause = !model.pause;
        }
        
    });    
    if !model.pause {
        model.time = app.time;
    }
}
fn gabor(x: f32, y: f32, kx_ratio: f32, ky_ratio: f32, theta: f32, sigma: f32, width: f32, height: f32) -> f32 {
    let kx = kx_ratio * std::f32::consts::PI / width;
    let ky = ky_ratio * std::f32::consts::PI / height;
    let x_theta = theta.cos() * x + theta.sin() * y;
    let y_theta = -theta.sin() * x + theta.cos() * y;
    (-0.5 * (x_theta.powi(2) / sigma.powi(2) + y_theta.powi(2) / sigma.powi(2))).exp()
        * (2.0 * std::f32::consts::PI * (kx * x_theta + ky * y_theta)).cos()
}
fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let draw = app.draw().scale(model.scale);
    let win = app.window_rect();
    draw.background().color(GRAY);
    let kx_ratio = settings.kx;
    let ky_ratio = settings.ky;
    let theta = (settings.t + model.time).to_radians();
    let sigma = settings.sigma;
    let width = settings.u;
    let height = settings.v;
    let step_x = win.w() / width;
    let step_y = win.h() / height;
    for i in 0..width as i32 {
        for j in 0..height as i32 {
            let x = step_x * i as f32 - win.w() / 2.0;
            let y = step_y * j as f32 - win.h() / 2.0;
            let value = gabor(x, y, kx_ratio, ky_ratio, theta, sigma, width, height);
            let color = match settings.c {
                1 => nannou::color::gray(value.abs()),
                2 => nannou::color::rgb((x + win.w() / 2.0) / win.w(), (y + win.h() / 2.0) / win.h(), value.abs()), 
                3 => nannou::color::hsv((value + 1.0) / 2.0, 1.0, 1.0).into(),
                4 => nannou::color::hsv((value.cos() + 1.0) / 2.0, (x + win.w() / 2.0) / win.w(), (y + win.h() / 2.0) / win.h()).into(),
                5 => nannou::color::hsv((model.time % 1.0 + value + 1.0) / 2.0, 1.0, 1.0).into(), 
                6 => nannou::color::gray(((value.sin() + 1.0) / 2.0).abs()).into(),
                7 => nannou::color::gray(((model.time % 3.0 + value + 1.0) / 2.0).abs()).into(), 
                8 => nannou::color::hsva((value.cos() + 1.0) / 2.0, ((x + win.w() / 2.0) / win.w()).abs(), ((y + win.h() / 2.0) / win.h()).abs(), (model.time % 1.0 + 1.0) / 2.0).into(),
                9 => nannou::color::hsva((value.sin() + 1.0) / 2.0, (model.time % 1.0 + 1.0) / 2.0, 1.0, ((x + win.w() / 2.0) / win.w()).abs()).into(),
                10 => nannou::color::rgb(((x + win.w() / 2.0) / win.w()).abs(), ((y + win.h() / 2.0) / win.h()).abs(), ((value + 1.0) / 2.0).abs()),
                11 => nannou::color::rgb(((model.time % 1.0 + value + 1.0) / 2.0).abs(), ((x + win.w() / 2.0) / win.w()).abs(), ((y + win.h() / 2.0) / win.h()).abs()),
                12 => nannou::color::rgb((model.time % 1.0 + 1.0) / 2.0, ((x + win.w() / 2.0) / win.w()).abs(), ((value + 1.0) / 2.0).abs()),
                13 => nannou::color::rgb(((y + win.h() / 2.0) / win.h()).abs(), ((value.sin() + 1.0) / 2.0).abs(), ((x + win.w() / 2.0) / win.w()).abs()),
                _ => nannou::color::rgb(0.0, 0.0, 0.0),
            };            
            match settings.shape {
                1 => {
                    draw.rect().x_y(x, y).w_h(step_x, step_y).color(color);
                }
                2 => {
                    draw.ellipse().x_y(x, y).w_h(step_x, step_y).color(color);
                }
                3 => {
                    draw.line().start(pt2(x, y)).end(pt2(x + step_x * value, y + step_y * value)).color(color).weight(10.0);
                }
                _ => (),
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