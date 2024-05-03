use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    time: f32,
    egui: Egui,
    settings: Settings,
    scale: f32,
}

struct Settings {
    y_frequency: f32,
    x_amplitude: f32,
    y_amplitude: f32,
    phase_shift: f32,
    point_count: usize,
    hue_range: f32,
    hue_offset: f32,
    weight: f32,
    time : f32,
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


        Model { time: 0.0 ,scale: 1.0,

        egui,
        settings: Settings {
            y_frequency: 150.0,
            x_amplitude: 200.0,
            y_amplitude: 100.0,
            phase_shift: 0.0,
            point_count: 45,
            hue_range: 0.9,
            hue_offset: 0.1,
            weight: 4.0,
            time : 0.01,
            show_ui: true,
        },
}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.separator();
        ui.add(egui::Slider::new(&mut model.settings.y_frequency, 0.0..=1000.0).text("y_frequency"));
        ui.add(egui::Slider::new(&mut model.settings.x_amplitude, 0.0..=1000.0).text("x_amplitude"));
        ui.add(egui::Slider::new(&mut model.settings.y_amplitude, 0.0..=1000.0).text("y_amplitude"));
        ui.add(egui::Slider::new(&mut model.settings.phase_shift, 0.0..=1000.0).text("phase_shift"));
        ui.add(egui::Slider::new(&mut model.settings.point_count, 1..=1000).text("point_count"));
        ui.add(egui::Slider::new(&mut model.settings.hue_range, 0.0..=1000.0).text("hue_range"));
        ui.add(egui::Slider::new(&mut model.settings.hue_offset, 0.0..=1000.0).text("hue_offset"));
        ui.add(egui::Slider::new(&mut model.settings.weight, 0.0..=10.0).text("weight"));
        ui.add(egui::Slider::new(&mut model.settings.time, 0.0..=0.1).text("time"));
    });
    model.time += model.settings.time;

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    draw.background().color(BLACK);
    let y_frequency = model.settings.y_frequency;
    let x_amplitude = model.settings.x_amplitude;
    let y_amplitude = model.settings.y_amplitude;
    let phase_shift = model.time;
    let point_count = model.settings.point_count;
    let hue_range = model.settings.hue_range;
    let hue_offset = model.settings.hue_offset;

    let hue_values: Vec<f32> = (0..point_count)
        .map(|i| {
            let hue = i as f32 / point_count as f32 * hue_range + hue_offset;
            hue % 1.0
        })
        .collect();

    let points: Vec<Point2> = (0..point_count)
        .map(|i| {
            let x = (i as f32 / point_count as f32 * 12.0 * PI).sin() * x_amplitude;
            let y = ((i as f32 / point_count as f32 * 1.0 * PI * y_frequency) + phase_shift).sin()
                * y_amplitude;
            pt2(x, y)
        })
        .collect();

    for i in 0..point_count - 1 {
        let start = points[i];
        let end = points[i + 1];
        let hue_start = hue_values[i];
        let hue_end = hue_values[i + 1];
        let color_start = hsla(hue_start, 1.0, 0.5, 1.0);
        let color_end = hsla(hue_end, 1.0, 0.5, 1.0);
        draw.line()
            .start(start)
            .end(end)
            .weight(model.settings.weight)
            .color(color_start)
            .color(color_end);
    }

    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
    if app.keys.down.contains(&Key::Space) {
        match app.project_path() {
            Ok(project_path) => {
                let frames_path = project_path.join("frames");
                if let Err(e) = std::fs::create_dir_all(&frames_path) {
                    eprintln!("Failed to create frames directory: {:?}", e);
                    return;
                }
                let file_path = frames_path.join(format!("{:0}.png", app.elapsed_frames()));
                app.main_window().capture_frame(file_path);
            },
            Err(e) => {
                eprintln!("Failed to locate project directory: {:?}", e);
            }
        }
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