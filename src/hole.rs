use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    phase: f32,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    num_points: usize,
    freq: f32,
    scale2: f32,
    r: f32,
    y: f32,
    x: f32,
    z: f32,
    u: f32,
    sf: f32,
    sr: f32,
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
        num_points: 500,
        freq: 45.0,
        scale2: 600.0,
        r: 40.0,
        y: 0.1,
        x: 0.6,
        z: 0.5,
        u: 1.0,
        sf: 10.0,
        sr: 0.002,
    };    
    Model { phase: 0.0, egui, settings ,scale: 1.0}
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut settings.num_points, 0..=1000).text("num_points"));
        ui.add(egui::Slider::new(&mut settings.freq, 0.0..=100.0).text("freq"));
        ui.add(egui::Slider::new(&mut settings.scale2, 0.0..=1000.0).text("scale"));
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=100.0).text("r"));
        ui.add(egui::Slider::new(&mut settings.y, 0.0..=1.0).text("y"));
        ui.add(egui::Slider::new(&mut settings.x, 0.0..=1.0).text("x"));
        ui.add(egui::Slider::new(&mut settings.z, 0.0..=1.0).text("z"));
        ui.add(egui::Slider::new(&mut settings.u, 0.0..=1.0).text("u"));
        ui.add(egui::Slider::new(&mut settings.sf, 0.0..=100.0).text("sf"));
        ui.add(egui::Slider::new(&mut settings.sr, 0.0..=10.1).text("sr"));
    });    
       model.phase += 0.01;
}
fn gauss(x: f32) -> f32 {
    (-10.0 * x * x).exp()
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    let _win = app.window_rect();
    let phase = model.phase;
    let spiral_factor = model.settings.sf;
    let spiral_radius = model.settings.sr;
    draw.background().color(BLACK);
    let freq = model.settings.freq;
    let num_points = model.settings.num_points;
    let scale = model.settings.scale2;
    let center1 = pt2(-200.0, 0.0);
    let center2 = pt2(200.0, 0.0);

    for i in 0..num_points {
        let t = map_range(i, 0, num_points, 0.0, 1.0);
        let x = (t - model.settings.y) * scale;
        let g = gauss(x / scale);
        let y = g * model.settings.x * (freq * t * 2.0 * PI + phase).sin() * scale;

        let progress = i as f32 / num_points as f32;
        let hue = progress;
        let saturation = model.settings.u - progress;
        let lightness = 0.4 + model.settings.z * (app.time + progress * PI).sin();
        let color1 = hsla(hue, saturation, lightness, g);

        let hue2: f32 = 0.5 + 0.5 * (app.time + progress * PI).sin();
        let saturation2 = progress;
        let lightness2 = model.settings.z + 0.5 * (app.time + progress * PI).cos();
        let color2 = hsla(hue2, saturation2, lightness2, 1.0);

        let spiral_offset1 = spiral_offset(i, num_points, spiral_factor, spiral_radius);
        let point1 = pt2(center1.x + x + spiral_offset1.x, center1.y + y + spiral_offset1.y);
        draw.ellipse()
            .xy(point1)
            .radius(model.settings.r)
            .color(color1);

        let spiral_offset2 = spiral_offset(i, num_points, spiral_factor, spiral_radius);
        let point2 = pt2(-center2.x - x - spiral_offset2.x, center2.y + y + spiral_offset2.y);
        draw.ellipse()
            .xy(point2)
            .radius(model.settings.r)
            .color(color2);
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame);
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
    }
    fn spiral_offset(index: usize, num_points: usize, factor: f32, radius: f32) -> Vec2 {
        let angle = index as f32 * factor * TAU / num_points as f32;
        let r = radius * angle;
        vec2(angle.cos() * r, angle.sin() * r)
    }
    