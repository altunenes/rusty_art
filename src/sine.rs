use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    time: f32,
    egui: Egui,
    settings: Settings,
    perlin: Perlin,
    scale:f32,
}
struct Settings {
    num_waves: usize,
    wave_spacing: f32,
    wave_speed: f32,
    wave_offset: f32,
    x: usize,
    p_scale: f64,
    show_ui:bool,
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
    Model { scale:1.0,time: 0.0 ,        perlin: Perlin::new(),
        egui, settings: Settings {
        num_waves: 37,
        wave_spacing: 58.0,
        wave_speed: 0.41,
        wave_offset: 20.0,
        x:100,
        p_scale: 1.0,
        show_ui:true,

    }}
    }
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
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
        ui.label("P Scale:");
        ui.add(egui::Slider::new(&mut model.settings.p_scale, 0.0..=10.1));
    });
    model.time +=model.settings.wave_speed;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let draw = app.draw().scale(model.scale);
    let width = app.window_rect().w();
    let height = app.window_rect().h();
    let num_waves = settings.num_waves;
    let wave_spacing = settings.wave_spacing;
    let perlin_scale = model.settings.p_scale;
    for i in 0..settings.x {
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
                let perlin_value = model.perlin.get([
                    perlin_scale * x as f64,
                    perlin_scale * (model.time * wave_speed) as f64,
                ]);
                let y = wave_scale
                    * (x / wave_spacing + wave_offset + model.time * wave_speed).sin()
                    * height
                    / settings.wave_offset
                    * (1.0 + perlin_value as f32);
                pt2(x - width / 2.0, y)
            });

        draw.polyline()
            .weight(wave_thickness)
            .points_colored(wave_points.map(|pt| (pt, wave_color)));
    }
    draw.background().color(BLACK);
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
