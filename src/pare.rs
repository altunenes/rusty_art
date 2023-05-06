// inspired by Roni Kaufman's work 

use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::math::map_range;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    theta: f32,
    perlin: Perlin,    egui: Egui,
    settings: Settings,
    scale: f32,


}
struct Settings {
r: f32,
s: f32,
f: f32,
sc: f32,
o: f32,
p : f32,

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
        theta: 0.0,
        perlin: Perlin::new(),
        egui,
        scale: 1.0,
        settings: Settings {
            r: 75.0,
            s: 15.0,
            f: 40.0,
            sc: 500.0,
            o: 10.0,
            p: 2.0,
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
        let egui = &mut model.egui;
        let settings = &mut model.settings;
        egui.set_elapsed_time(_update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Settings");
            ui.add(egui::Slider::new(&mut settings.r, 0.0..=100.0).text("r"));
            ui.add(egui::Slider::new(&mut settings.s, 0.0..=100.0).text("s"));
            ui.add(egui::Slider::new(&mut settings.f, 0.0..=100.0).text("f"));
            ui.add(egui::Slider::new(&mut settings.sc, 0.0..=1000.0).text("sc"));
            ui.add(egui::Slider::new(&mut settings.o, 0.0..=100.0).text("o"));
            ui.add(egui::Slider::new(&mut settings.p, 0.0..=100.0).text("p"));
        });


    let t = _app.time;
    model.theta = model.settings.p*PI * (t % model.settings.f) / model.settings.f;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);

    let win = app.window_rect();
    let width = win.w();
    let height = win.h();

    let s = model.settings.s;

    let num_points = (width as usize * height as usize) / (s as usize).pow(2);
    let u = map_range(app.time.sin(), -1.0, 1.0, 0.0, 1.0);
    let z = map_range(app.time.cos(), -1.0, 1.0, 0.0, 1.0);
    let mut i = 0;
    for x in (0..width as i32).step_by(s as usize) {
        for y in (0..height as i32).step_by(s as usize) {
            let x = x as f32;
            let y = y as f32;

            let nse = if x < width / 2.0 {
                loop_noise(x, y, model.theta, &model.perlin, &model.settings)
            } else {
                loop_noise(width - x, y, model.theta, &model.perlin, &model.settings)
            };
            if nse < 0.4 {
                let progress = i as f32 / num_points as f32;
                let hue = progress;
                let saturation = 1.0 + u - progress;
                let lightness = z + u * ((app.time * 2.0).sin() * 0.5 + 0.5) * progress.sin();
                let color = hsla(hue, saturation, lightness, 1.0);
                draw.ellipse()
                    .x_y(x - width / 2.0, y - height / 2.0)
                    .radius(model.settings.r)
                    .color(color);
                i += 1;
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap()
}
fn loop_noise(x: f32, y: f32, theta: f32, perlin: &Perlin, settings: &Settings) -> f64 {
    let offset = settings.o;
    let sc = 1.0 / settings.sc;

    perlin.get([
        (offset + (x * sc) * theta.cos()) as f64,
        (offset + (x * sc) * theta.sin()) as f64,
        (y * sc) as f64,
    ])
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