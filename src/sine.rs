use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    time: f32,
    egui: Egui,
    settings: Settings,
}
struct Settings {
    num_waves: usize,
    wave_spacing: f32,
    wave_speed: f32,
    wave_offset: f32,
    x: usize,
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
    Model { time: 0.0 , egui, settings: Settings {
        num_waves: 37,
        wave_spacing: 58.0,
        wave_speed: 0.41,
        wave_offset: 20.0,
        x:100
    }}
    }
    fn update(_app: &App, model: &mut Model, _update: Update) {
        let egui = &mut model.egui;
        let _settings = &mut model.settings;
        egui.set_elapsed_time(_update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("Num W:");
            ui.add(egui::Slider::new(&mut model.settings.num_waves, 1..=255));
            ui.label("Wave S:");
            ui.add(egui::Slider::new(&mut model.settings.wave_spacing, 1.0..=255.0));
            ui.label("Wave S2:");
            ui.add(egui::Slider::new(&mut model.settings.wave_speed, 0.0..=1.1));
            ui.label("Wave O:");
            ui.add(egui::Slider::new(&mut model.settings.wave_offset, 1.0..=255.0));
            ui.label("X:");
            ui.add(egui::Slider::new(&mut model.settings.x, 1..=255));
        });
                model.time +=model.settings.wave_speed;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let _settings = &model.settings;    let draw = app.draw();
    let width = app.window_rect().w();
    let height = app.window_rect().h();
    let num_waves = model.settings.num_waves;
    let wave_spacing = model.settings.wave_spacing;

    for i in 0..model.settings.x {
        let wave_color = hsla(
            map_range(i, 0, num_waves, 0.0, 360.0),
            0.8,
            0.5,
            1.0,
        );
        let wave_thickness = map_range(i, 0, num_waves, 2.0, 10.0);
        let wave_scale = map_range(i, 0, num_waves, 1.0, 3.0);
        let wave_speed = map_range(i, 0, num_waves, 0.01, 0.05);
        let wave_offset = map_range(i, 0, num_waves, -1.0, 1.0);
        let wave_points = (0..=width as i32)
            .step_by(3)
            .map(|x| {
                let x = x as f32;
                let y = wave_scale * (x / wave_spacing + wave_offset + model.time * wave_speed).sin()
                    * height / model.settings.wave_offset;
                pt2(x - width / 2.0, y)
            });

        draw.polyline()
            .weight(wave_thickness)
            .points_colored(wave_points.map(|pt| (pt, wave_color)));
    }
    draw.background().color(BLACK);
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
