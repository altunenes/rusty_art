//Hilbert curve function based from: https://www.youtube.com/watch?v=dSK-MW-zuAc by Daniel Shiffman

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    counter_start: usize,
    counter_end: usize,
    path: Vec<Point2>,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    r:f32,
    s: f32,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        r: 1.0,
        s: 150.0,
    };
    let order = 8;
    let n = 2usize.pow(order as u32);
    let total =  n*n;
    let window_rect = app.window_rect();
    let len = window_rect.w().min(window_rect.h()) / n as f32;

    let mut path = Vec::with_capacity(total);
    for i in 0..total {
        let mut v = hilbert(i, order);
        v *= len;
        v -= vec2(len * n as f32 / 2.0, len * n as f32 / 2.0); 
        path.push(v);
    }
    Model {path, egui, settings, scale: 1.0, counter_start: 0, counter_end: 0}
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=40.0).text("r"));
        ui.add(egui::Slider::new(&mut settings.s, 0.1..=300.0).text("s"));
    });
    if model.counter_start < model.counter_end {
        model.counter_start += model.settings.s as usize;
        model.counter_end -= model.settings.s as usize;
    } else {
        model.counter_start = 0;
        model.counter_end = model.path.len();
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);
    for i in 1..model.counter_start {
        let hue = map_range(i, 0, model.path.len(), 0.0, 10.0);
        let color = hsl(hue, 1.0, 0.5);
        draw.line()
            .start(model.path[i - 1])
            .end(model.path[i])
            .color(color)
            .weight(model.settings.r);
    }
    for i in model.counter_end..model.path.len() {
        let hue = map_range(i, 0, model.path.len(), 0.0, 10.0);
        let color = hsl(hue, 1.0, 0.5);
        draw.line()
            .start(model.path[i - 1])
            .end(model.path[i])
            .color(color)
            .weight(model.settings.r);
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
fn hilbert(i: usize, order: u8) -> Point2 {
    let points = [
        pt2(0.0, 0.0),
        pt2(0.0, 1.0),
        pt2(1.0, 1.0),
        pt2(1.0, 0.0),
    ];

    let mut index = i & 3;
    let mut v = points[index];

    for j in 1..order {
        index = (i >> (2 * j as usize)) & 3;
        let len = 2f32.powi(j as i32);
        v = match index {
            0 => {
                let temp = v.x;
                v.x = v.y;
                v.y = temp;
                v
            }
            1 => {
                v.y += len;
                v
            }
            2 => {
                v.x += len;
                v.y += len;
                v
            }
            3 => {
                let temp = len - 1.0 - v.x;
                v.x = len - 1.0 - v.y;
                v.y = temp;
                v.x += len;
                v
            }
            _ => unreachable!(),
        };
    }
    v
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