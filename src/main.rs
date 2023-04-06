use nannou::prelude::*;
use rand::{thread_rng, Rng};
use nannou_egui::{self, egui, Egui};

struct Model {
    egui: Egui,
    settings: Settings,
}

struct Settings {
    number_of_ellipses: usize,
    frequeency: f32,
    angle : f32,
    line_weight: f32,
    angle2: f32,
    line_weight2: f32,
}

fn main() {
    nannou::app(model).update(update).run();
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
        settings: Settings {
            number_of_ellipses: 666,
            frequeency: 0.001,
            angle: 1.0,
            line_weight: 1.0,
            angle2: 0.9,
            line_weight2: 3.0,
        },
    }
}




fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("number_of_ellipses:");
        ui.add(egui::Slider::new(
            &mut model.settings.number_of_ellipses,
            0..=1000,
        ));
        ui.label("frequeency:");
        ui.add(egui::Slider::new(
            &mut model.settings.frequeency,
            0.0..=0.01,
        ));
        ui.label("angle:");
        ui.add(egui::Slider::new(
            &mut model.settings.angle,
            0.0..=10.0,
        ));
        ui.label("line_weight:");
        ui.add(egui::Slider::new(
            &mut model.settings.line_weight,
            0.0..=10.0,
        ));
        ui.label("angle2:");
        ui.add(egui::Slider::new(
            &mut model.settings.angle2,
            0.0..=10.0,
        ));
        ui.label("line_weight2:");
        ui.add(egui::Slider::new(
            &mut model.settings.line_weight2,
            0.0..=30.0,
        ));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    let duration = app.duration.since_start.secs() as f32;
    let window_diagonal = window.top_left().distance(window.bottom_right());
    let mut rng = thread_rng();
    draw.background().color(BLACK);
    let number_of_ellipses = model.settings.number_of_ellipses;
    for i in 0..number_of_ellipses {
        let position = i as f32 / number_of_ellipses as f32;
        let max_line_weight = (model.settings.line_weight / number_of_ellipses as f32) * window.w();
        let x_position = window.x.lerp(position);
        let frequency = model.settings.frequeency;
        let moving_x = (duration * frequency * 2.0 * PI).sin() * window.right();
        let distance = (moving_x - x_position).abs();
        let normalized_distance = distance / window.w();
        let line_weight = max_line_weight * normalized_distance * normalized_distance;
        let hue = rng.gen_range(0.0, 1.0);
        let angle = (duration * model.settings.angle2 + position) * model.settings.angle * PI;
        let magnitude = window_diagonal;
        let first_point = pt2(angle.cos() * magnitude, angle.sin() * magnitude);
        let second_point = pt2(angle.cos() * -magnitude, angle.sin() * -magnitude);
        let color = hsla(
            hue,
            (position * 1.0).min(1.0),
            1.0,
            normalized_distance * (171.0 - (angle / (25.0 * PI)).cos()),
        );
        draw.line()
            .weight(line_weight * model.settings.line_weight2)
            .points(first_point, second_point)
            .color(color);
    }
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();    
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}