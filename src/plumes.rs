use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    t: f32,
    egui: Egui,
    settings: Settings,
}

struct Settings {
    num_plumes: usize,
    plume_length: f32,
    plume_width: f32,
    plume_speed: f32,
    p_width: f32,
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
    Model {t: 0.0,
    egui,
    settings: Settings {
        num_plumes: 100,
        plume_length: 333.0,
        plume_width: 125.0,
        plume_speed: 1.8,
        p_width: 1.0,
    },
    }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    model.t = app.time;
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("num_plumes:");
        ui.add(egui::Slider::new(&mut model.settings.num_plumes, 0..=1000));
        ui.label("plume_length:");
        ui.add(egui::Slider::new(&mut model.settings.plume_length, 0.0..=1000.0));
        ui.label("plume_width:");
        ui.add(egui::Slider::new(&mut model.settings.plume_width, 0.0..=1000.0));
        ui.label("plume_speed:");
        ui.add(egui::Slider::new(&mut model.settings.plume_speed, 0.0..=5.0));
        ui.label("p_width:");
        ui.add(egui::Slider::new(&mut model.settings.p_width, 0.0..=10.0));
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let num_plumes = model.settings.num_plumes;
    let plume_length = model.settings.plume_length;
    let plume_width = model.settings.plume_width;
    let plume_speed = model.settings.plume_speed;
    for i in 0..num_plumes {
        let angle = map_range(i, 0, num_plumes, 0.0, 360.0);
        let plume_x = (angle.to_radians()).cos() * (app.time * plume_speed).cos() * plume_length;
        let plume_y = (angle.to_radians()).sin() * (app.time * plume_speed).sin() * plume_length;
        let plume_points = vec![
            pt2(plume_x - plume_width, plume_y + plume_width),
            pt2(plume_x + plume_width, plume_y + plume_width),
            pt2(plume_x + plume_width, plume_y - plume_width),
            pt2(plume_x - plume_width, plume_y - plume_width),
        ];
        let plume_color = Hsl::new(map_range(i, 0, num_plumes, 0.0, 360.0), 1.0, 0.5);
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(plume_x, plume_y))
            .weight(3.0)
            .color(plume_color);
        draw.ellipse()
            .x_y(plume_x, plume_y)
            .w_h(plume_width * 2.0, plume_width * 2.0)
            .color(plume_color)
            .radius(model.settings.p_width);
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