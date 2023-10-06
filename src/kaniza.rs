use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    pacman_rotation: f32,
    egui: Egui,
    settings: Settings,
}

struct Settings {
    pacman_radius: f32,
    rotation_speed: f32,
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
        pacman_rotation: 40.0,
        settings: Settings {
            pacman_radius: 50.0,
            rotation_speed: 0.002,
        },
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.separator();
        ui.add(egui::Slider::new(&mut model.settings.pacman_radius, 10.0..=300.0).text("Pacman Radius"));
        ui.add(egui::Slider::new(&mut model.settings.rotation_speed, 0.0..=0.1).text("Rotation Speed"));
    });
    model.pacman_rotation += model.settings.rotation_speed;
}



fn draw_pacmans(draw: &Draw, num_pacmans: usize, start_position: Point2, distance: f32, radius: f32, rotation: f32, angle: f32) {
    for i in 0..num_pacmans {
        for j in 0..num_pacmans {
            let position = start_position + vec2(distance * i as f32, distance * j as f32);
            
            let current_rotation = rotation + (std::f32::consts::PI / 2.0) * ((i + j) % 2 + 2 * (i % 2)) as f32;
            
            draw_pacman_with_polyline(draw, position, radius, current_rotation, angle * 2.0); 
        }
    }
}
fn draw_pacman_with_polyline(draw: &Draw, position: Point2, radius: f32, rotation: f32, angle: f32) {
    const NUM_POINTS: usize = 100;
    let half_angle = angle / 2.0;
    let start_angle = rotation - half_angle;
    let end_angle = rotation + half_angle;
    let mut points = Vec::with_capacity(NUM_POINTS + 1);
    points.push(position);
    for i in 0..=NUM_POINTS {
        let t = i as f32 / NUM_POINTS as f32;
        let current_angle = lerp(end_angle, start_angle + 2.0 * std::f32::consts::PI, t);
        let x = position.x + radius * current_angle.cos();
        let y = position.y + radius * current_angle.sin();
        points.push(pt2(x, y));
    }
    draw.polygon()
        .points(points)
        .color(WHITE);
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let pacman_angle = std::f32::consts::PI / 4.0; 

    let num_pacmans = 10; 
    let window_rect = app.window_rect();
    let distance = window_rect.w() / num_pacmans as f32; 
    let start_position = pt2(window_rect.left(), window_rect.bottom());
    draw_pacmans(&draw, num_pacmans, start_position, distance, model.settings.pacman_radius, model.pacman_rotation, pacman_angle);

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}