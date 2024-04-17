use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    pacman_rotation: f32,
    egui: Egui,
    settings: Settings,
    scale:f32,
}
struct Settings {
    pacman_radius: f32,
    rotation_speed: f32,
    show_ui: bool,
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
        scale:1.0,
        pacman_rotation: 40.0,
        settings: Settings {
            pacman_radius: 50.0,
            rotation_speed: 0.002,
            show_ui: true,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
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

fn draw_pacmans(draw: &Draw, num_pacmans: usize, start_position: Point2, distance: f32, radius: f32, rotation: f32, angle: f32, color: Srgba<u8>) {
    for i in 0..num_pacmans {
        for j in 0..num_pacmans {
            let position = start_position + vec2(distance * i as f32, distance * j as f32);
            
            let current_rotation = rotation + (std::f32::consts::PI / 2.0) * ((i + j) % 2 + 2 * (i % 2)) as f32;
            
            draw_pacman_with_polyline(draw, position, radius, current_rotation, angle * 2.0, color); 
        }
    }
}
fn draw_pacman_with_polyline(draw: &Draw, position: Point2, radius: f32, rotation: f32, angle: f32, color: Srgba<u8>) {
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
        .color(color);
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    let elapsed_time = app.duration.since_start.as_secs_f32();
    let r = ((elapsed_time.sin() * 0.5 + 0.5) * 255.0) as u8;
    let g = ((elapsed_time.cos() * 0.5 + 0.5) * 255.0) as u8;
    draw.background().color(srgba(r, g, 255 - r, 255));
    let pacman_angle = std::f32::consts::PI / 4.0; 
    let num_pacmans = 10; 
    let window_rect = app.window_rect();
    let distance = window_rect.w() / num_pacmans as f32; 
    let start_position = pt2(window_rect.left(), window_rect.bottom());
    draw_pacmans(&draw, num_pacmans, start_position, distance, model.settings.pacman_radius, model.pacman_rotation, pacman_angle, rgba(255 - r, 255 - g, r, 255));
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
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}