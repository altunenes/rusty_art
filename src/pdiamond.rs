use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    phase: f32,
    egui: Egui,
    settings: Settings,
}
struct Settings{
    s_phase: f32,
    p_2: f32,
    stripe_width: f32,
    rotation: f32,
    edge_size: f32,
    square_size: f32,
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
    Model { phase: 0.0,egui,settings: Settings {
        s_phase: 4.0,
        p_2: 1.5707963,
        stripe_width: 2.2,
        rotation: 0.7853981,
        edge_size: 0.005,
        square_size: 150.2,
    }, }
    } 
    fn update(_app: &App, model: &mut Model, _update: Update) {
        let egui = &mut model.egui;
        let _settings = &model.settings;
        egui.set_elapsed_time(_update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("s_phase:");
            ui.add(egui::Slider::new( &mut model.settings.s_phase, 0.0..=10.0,));
            ui.label("p_2:");
            ui.add(egui::Slider::new( &mut model.settings.p_2, 0.0..=6.28,));
            ui.label("stripe_width:");
            ui.add(egui::Slider::new( &mut model.settings.stripe_width, 0.0..=10.0,));
            ui.label("rotation:");
            ui.add(egui::Slider::new( &mut model.settings.rotation, 0.0..=9.42,));
            ui.label("edge_size:");
            ui.add(egui::Slider::new( &mut model.settings.edge_size, 0.0..=0.1,));
            ui.label("square_size:");
            ui.add(egui::Slider::new( &mut model.settings.square_size, 0.0..=300.0,));
            if ui.button("Up").clicked() {
                model.settings.p_2 = 4.40;
                model.settings.rotation = 0.78;
            }
            if ui.button("Down").clicked() {
                model.settings.p_2 = 1.5708;
                model.settings.rotation = 0.78;
            }
            if ui.button("Left").clicked() {
                model.settings.p_2 = 1.5708;
                model.settings.rotation = 5.5;
            }
            if ui.button("Right").clicked() {
                model.settings.p_2 = 4.40;
                model.settings.rotation = 5.5;
            }
        });
                model.phase += model.settings.s_phase / 60.0; 
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(sin_phase_color(model.phase));
    let PI_2 = model.settings.p_2;
    let square_size = model.settings.square_size;
    let edge_size = model.settings.edge_size;
    let rotation = model.settings.rotation;
    let diamond_points = vec![
        pt2(-square_size, 0.0),
        pt2(0.0, square_size),
        pt2(square_size, 0.0),
        pt2(0.0, -square_size),
    ];
    draw.polygon()
        .points(diamond_points.clone())
        .color(pink_color());
        for (i, point) in diamond_points.iter().enumerate() {
            let next_point = diamond_points[(i + 1) % 4];
            let edge_start = *point * (1.0 - edge_size);
            let edge_end = next_point * (1.0 - edge_size);
            let edge_phase = if i < 2 {
                model.phase + PI_2
            } else {
                model.phase - PI_2
            };
            draw.line()
                .start(edge_start.rotate(rotation))
                .end(edge_end.rotate(rotation))
                .rotate(8.65)
                .weight(model.settings.stripe_width)
                .color(sin_phase_color(edge_phase));
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
fn sin_phase_color(phase: f32) -> Srgb<u8> {
    let value = (phase.sin() * 0.2 + 0.4) * 255.0;
    srgb(value as u8, value as u8, value as u8)
}
fn pink_color() -> Srgb<u8> {
    srgb(136, 105, 121)
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}