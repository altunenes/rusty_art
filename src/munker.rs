use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    dot_size: f32,
    animation_paused: bool,
    a_p_s: std::time::Instant,
    a_p_d: std::time::Duration,
    
    settings: Settings,
    egui: Egui,
}
struct Settings {
    left_circle_color: egui::Color32,
    right_circle_color: egui::Color32,
    circle_color: egui::Color32,
    clear: bool,
    n_dots: f32,
    r1: f32,
    r2: f32,
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model { dot_size: 20.0, animation_paused: false, egui,
        a_p_s: std::time::Instant::now(),
        a_p_d: std::time::Duration::new(0, 0),
        settings: Settings {
            left_circle_color:egui::Color32::from_rgb(128, 0, 128),
            right_circle_color:egui::Color32::from_rgb(255, 255, 0),
            circle_color: egui::Color32::from_rgb(255, 0, 0),
            clear: false,
            n_dots: 120.0,
            r1: 0.45,
            r2: 120.0,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("left_stripe_color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut settings.left_circle_color,
                egui::color_picker::Alpha::Opaque,
            );
        });
        ui.horizontal(|ui| {
            ui.label("right_stripe_color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut settings.right_circle_color,
                egui::color_picker::Alpha::Opaque,
            );
        });

        ui.horizontal(|ui| {
            ui.label("circle_color:");
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut settings.circle_color,
                egui::color_picker::Alpha::Opaque,
            );
        });

        ui.separator();
        let clicked = ui.button("Clear").clicked();
        if clicked {
            settings.clear = !settings.clear;
        }
        ui.separator();

        let clicked = ui.button("Stop/Resume Animation").clicked();
        if clicked {
            model.animation_paused = !model.animation_paused;
            if model.animation_paused {
                model.a_p_s = std::time::Instant::now();
            } else {
                model.a_p_d += std::time::Instant::now().duration_since(model.a_p_s);
            }
        }


        ui.label("n_dots:");
        ui.add(egui::Slider::new(&mut settings.n_dots, 0.1..=240.0).text("n_dots"));
        ui.label("r1:");
        ui.add(egui::Slider::new(&mut settings.r1, 0.1..=1.0).text("r1"));
        ui.label("r2:");
        ui.add(egui::Slider::new(&mut settings.r2, 0.1..=240.0).text("r2"));

    });


    let time = (_app.time - model.a_p_d.as_secs_f32()) as f32;
    if !model.animation_paused {
        model.dot_size = 0.35 * (0.5 * time.sin().powi(4) + 0.5);
    }

}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.rect().x_y(-win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(srgba(model.settings.right_circle_color.r(), model.settings.right_circle_color.g(), model.settings.right_circle_color.b(),model.settings.right_circle_color.a(),       
));
    draw.rect().x_y(win.w() / 4.0, 0.0).w_h(win.w() / 2.0, win.h()).color(srgba(model.settings.left_circle_color.r(), model.settings.left_circle_color.g(), model.settings.left_circle_color.b(),model.settings.left_circle_color.a(),
));
    draw.ellipse()
        .x_y(win.w() / 7.0, 0.0)
        .radius(model.settings.r1 * win.h() / 2.0)
        .color(srgba(model.settings.circle_color.r(), model.settings.circle_color.g(), model.settings.circle_color.b(),model.settings.circle_color.a(),
));
    draw.ellipse()
        .x_y(-win.w() / 7.0, 0.0)
        .radius(model.settings.r1 * win.h() / 2.0)
        .color(srgba(model.settings.circle_color.r(), model.settings.circle_color.g(), model.settings.circle_color.b(),model.settings.circle_color.a(),
));
if !model.settings.clear {
    for x in 0..=model.settings.n_dots as usize {
        for y in 0..=model.settings.n_dots as usize {
            let uv = pt2(
                (x as f32) / model.settings.n_dots * win.w() - win.right() / 1.0,
                (y as f32) / model.settings.n_dots * win.h() - win.top() / 1.0,
            );
            let dot_color = if uv.x < 0.0 { model.settings.left_circle_color } else { model.settings.right_circle_color };
            draw.ellipse()
                .xy(uv)
                .radius(model.dot_size * win.w() / model.settings.r2)
                .color(srgba(dot_color.r(), dot_color.g(), dot_color.b(), dot_color.a()));
        }
    }
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
