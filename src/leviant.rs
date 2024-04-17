use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    settings: Settings,
    egui: Egui,
    rotation: f32,
    scale: f32,

}

struct Settings {
    ring_colors: Vec<egui::Color32>,
    animate: bool,
    sectors: usize,
    resolution: usize,
    v: f32,
    show_ui: bool,
    polygon_colors: [egui::Color32; 2],
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

    Model {
        egui,
        rotation: 0.0,
        scale: 1.0,


        settings: Settings {
            animate: false,
            sectors: 140,
            resolution: 100,
            show_ui:true,
            v: 0.01,
            polygon_colors: [
                egui::Color32::from_rgb(0, 0, 0),    // BLACK
                egui::Color32::from_rgb(255, 255, 255),],  // WHITE

            ring_colors: vec![
                egui::Color32::from_rgb(176, 113, 171),
                egui::Color32::from_rgb(160, 104, 163),
                egui::Color32::from_rgb(160, 104, 163),
                egui::Color32::from_rgb(128, 111, 171),
                egui::Color32::from_rgb(160, 104, 163),
                egui::Color32::from_rgb(128, 111, 171),
                egui::Color32::from_rgb(160, 104, 163),
                egui::Color32::from_rgb(128, 111, 171),
                egui::Color32::from_rgb(160, 104, 163),
                egui::Color32::from_rgb(128, 111, 171),
            ],
            
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
        for (i, color) in model.settings.ring_colors.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Ring {} color:", i + 1));
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    color,
                    egui::color_picker::Alpha::Opaque,
                );
            });

        }
        ui.separator();
        ui.label("Animate:");
        let animate_state = model.settings.animate;
        let clicked = ui.button(if animate_state { "Stop" } else { "Animate" }).clicked();
        if clicked {
            model.settings.animate = !animate_state;
        }
        ui.separator();
        ui.label("Number of sectors:");
        ui.add(egui::Slider::new(&mut model.settings.sectors, 1..=2000));

        ui.separator();
        ui.label("shape:");
        ui.add(egui::Slider::new(&mut model.settings.resolution, 1..=100));

        ui.label("speed:");
        ui.add(egui::Slider::new(&mut model.settings.v, 0.001..=0.1));

        ui.separator();

        ui.label("Polygon colors:");
        for (i, color) in model.settings.polygon_colors.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(format!("Polygon color {}:", i + 1));
                egui::color_picker::color_edit_button_srgba(
                    ui,
                    color,
                    egui::color_picker::Alpha::Opaque,
                );
            });
        }
    });

    if model.settings.animate {
        model.rotation += model.settings.v;
    }
}
fn egui_to_nannou_color(color: egui::Color32) -> nannou::color::Srgba {
    let (r, g, b, a) = color.to_tuple();
    nannou::color::srgba(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}
fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw().scale(_model.scale);
    let window_width = _app.window_rect().w();
    let window_height = _app.window_rect().h();
    draw.background().color(WHITE);
    let n_circles = 3;
    let circle_radius = 100.0;
    let n_sectors = _model.settings.sectors;
    for i in 0..n_circles {
        let radius = circle_radius * (i + 1) as f32 * window_width.min(window_height) / 2.0;
        for j in 0..n_sectors {
            let start_angle = (j as f32 / n_sectors as f32 * TAU) + _model.rotation;
            let end_angle = ((j + 1) as f32 / n_sectors as f32 * TAU) + _model.rotation;
            let color = egui_to_nannou_color(_model.settings.polygon_colors[j % 2]);
            let points = vec![
                pt2(0.0, 0.0),
                pt2(start_angle.cos() * radius, start_angle.sin() * radius),
                pt2(end_angle.cos() * radius, end_angle.sin() * radius),
            ];
            draw.polygon().color(color).points(points);
        }
    } 
    draw.ellipse().color(rgb(160.0/255.0, 104.0/255.0, 163.0/255.0)).w_h(0.1 * window_width.min(window_height), 0.1 * window_width.min(window_height));
    let ring_inner_radius = [0.18, 0.21, 0.40, 0.45, 0.7, 0.75, 1.25, 1.30, 1.0, 1.05];
    let ring_outer_radius = [0.21, 0.24, 0.45, 0.50, 0.75, 0.80, 1.30, 1.35, 1.05, 1.10];
    let ring_resolution = _model.settings.resolution;
    let ring_radius_scale = window_width.min(window_height) / 2.0;
    for r in 0..10 {
        let mut ring_points = Vec::new();

        for i in 0..=ring_resolution {
            let angle = i as f32 / ring_resolution as f32 * TAU;
            ring_points.push(pt2(angle.cos() * ring_outer_radius[r] * ring_radius_scale, angle.sin() * ring_outer_radius[r] * ring_radius_scale));
        }
        for i in (0..=ring_resolution).rev() {
            let angle = i as f32 / ring_resolution as f32 * TAU;
            ring_points.push(pt2(angle.cos() * ring_inner_radius[r] * ring_radius_scale, angle.sin() * ring_inner_radius[r] * ring_radius_scale));
        }

        draw.polygon().color(srgba(_model.settings.ring_colors[r].r(), _model.settings.ring_colors[r].g(), _model.settings.ring_colors[r].b(), _model.settings.ring_colors[r].a())).points(ring_points);
    }

    draw.to_frame(_app, &frame).unwrap();
    if _model.settings.show_ui {
        _model.egui.draw_to_frame(&frame).unwrap();
    }        if _app.keys.down.contains(&Key::Space) {
        let file_path = _app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", _app.elapsed_frames()));
        _app.main_window().capture_frame(file_path);
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