use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
enum Formula {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
}

enum Movie{
    Tru,
    Fal,
}

struct Model {
    egui: Egui,
    movie: Movie,
    formula: Formula,
    t: f32,
    settings: Settings,
    x: f32,
    y: f32,
}
struct Settings {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    trail_length: f32,
    time: f32,
    radius: f32,
    t_factor: f32,
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
        t: 0.0,
        egui,
        x: 0.0,
        y: 0.0,
        movie: Movie::Tru,
        formula: Formula::Third,
        settings: Settings {
            a: -0.45,
            b: -0.80,
            c: -1.60,
            d: 2.0,
            trail_length: 300.0,
            time: 200.0,
            radius: 1.0,
            t_factor: 400.05,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {   
        ui.label("a:");
        ui.add(egui::Slider::new(&mut model.settings.a, -2.5..=2.5));
        ui.label("b:");
        ui.add(egui::Slider::new(&mut model.settings.b, -2.5..=2.5));
        ui.label("c:");
        ui.add(egui::Slider::new(&mut model.settings.c, -2.5..=2.5));
        ui.label("d:");
        ui.add(egui::Slider::new(&mut model.settings.d, -2.5..=2.5));
        ui.label("trail_length:");
        ui.add(egui::Slider::new(&mut model.settings.trail_length, 0.0..=2500.0));
        ui.label("time:");
        ui.add(egui::Slider::new(&mut model.settings.time, 0.0..=1000.0));
        ui.label("radius:");
        ui.add(egui::Slider::new(&mut model.settings.radius, 0.0..=10.0));
        ui.label("pattern:");
        ui.add(egui::Slider::new(&mut model.settings.t_factor, -0.0..=5000.0));        
        ui.label("random:");
        if ui.button("random").clicked() {
            model.settings.a = random_range(-2.0, 2.0);
            model.settings.b = random_range(-2.0, 2.0);
            model.settings.c = random_range(-2.0, 2.0);
            model.settings.d = random_range(-2.0, 2.0);
            model.settings.t_factor = random_range(-0.0, 5000.0);
        };
        ui.label("animation");
        if ui.button("movie").clicked() {
            model.movie = match model.movie {
                Movie::Tru => Movie::Fal,
                Movie::Fal => Movie::Tru,
            };
        }
        ui.label("formula:");
        if ui.button("Change formula").clicked() {
            model.formula = match model.formula {
                Formula::First => Formula::Second,
                Formula::Second => Formula::Third,
                Formula::Third => Formula::Fourth,
                Formula::Fourth => Formula::Fifth,
                Formula::Fifth => Formula::Sixth,
                Formula::Sixth => Formula::Seventh,
                Formula::Seventh => Formula::First,
            };        
        }
    });
        model.t = _app.elapsed_frames() as f32 / model.settings.time;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let trail_length = model.settings.trail_length;
    draw.background().color(BLACK);
    let mut x = model.x;
    let mut y = model.y;
    let color = hsl(model.t * 0.1, 0.5, 0.5);
    for i in 0..trail_length as usize {
         let x_prev = x;
         let y_prev = y;
         let t = match model.movie { 
            Movie::Tru => model.t, 
            Movie::Fal => model.t - (i as f32 * model.settings.t_factor),
         };
        match model.formula {
            Formula::First => {
                x = (model.settings.a * y_prev + t).sin() + model.settings.c * (model.settings.a * x_prev + t).cos() * (model.settings.b * x_prev + t).sin();
                y = (model.settings.b * x_prev + t).sin() + model.settings.d * (model.settings.b * y_prev + t).cos() * (model.settings.a * y_prev + t).sin();
            }
            Formula::Second => {
                x = (model.settings.a * y_prev+t).sin() - model.settings.c * (model.settings.a * x_prev+t).cos();
                y = (model.settings.b * x_prev+t).sin() - model.settings.d * (model.settings.b * y_prev+t).cos();
            }
            Formula::Third => {
                x = x_prev.sin() * model.settings.a + model.settings.c * x_prev.cos() * t.sin();
                y = y_prev.sin() * model.settings.b  +model.settings.d * y_prev.cos() * t.sin();
            }
            Formula::Fourth => {
                x = (model.settings.a * y_prev + t).sin() * (model.settings.a * x_prev + t).cos() + model.settings.c * (model.settings.a * x_prev + t).cos() * (model.settings.b * x_prev + t).sin();
                y = (model.settings.b * x_prev + t).sin() * (model.settings.b * y_prev + t).cos() + model.settings.d * (model.settings.b * y_prev + t).cos() * (model.settings.a * y_prev + t).sin();
            }
            Formula::Fifth => {
                x = x_prev.sin() * model.settings.a + model.settings.c * x_prev.cos() * t.sin();
                y = model.settings.c+y_prev.sin() * model.settings.b  +model.settings.d * y_prev.cos() * t.sin();
            }
            Formula::Sixth => {
                x = (model.settings.a * y_prev + t).sin() * (model.settings.a * x_prev + t).cos() + model.settings.c * (model.settings.a * x_prev + t).cos();
                y = (model.settings.b * x_prev + t).sin() * (model.settings.b * y_prev + t).cos() + model.settings.d * (model.settings.b * y_prev + t).cos();
            }
            Formula::Seventh => {
                x = (model.settings.a * y_prev + t).sin() * (model.settings.a * x_prev + t).sin() + model.settings.c * (model.settings.a * x_prev + t).sin() * (model.settings.b * x_prev + t).sin();
                y = (model.settings.b * x_prev + t).sin() * (model.settings.b * y_prev + t).cos() + model.settings.d * (model.settings.b * y_prev + t).cos() * (model.settings.a * y_prev + t).sin();
            }
        }
        
        let x_mapped = map_range(x, -2.0, 2.0, -300.0, 300.0); 
        let y_mapped = map_range(y, -2.0, 2.0, -300.0, 300.0);
        draw.ellipse()
            .x_y(x_mapped, y_mapped)
            .w_h(4.0, 4.0)
            .radius(model.settings.radius)
            .color(color);
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
    model.egui.handle_raw_event(event);
}