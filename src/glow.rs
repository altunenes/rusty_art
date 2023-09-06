use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).view(view).run();
}
struct Model {
    egui: Egui,
    settings: Settings,
}
struct Settings {
    ray_width: f32,
    ray_color: [f32; 4],
    sun_color: [f32; 4],
    sun_radius: f32,
    num_rays: usize,
    glow_color: [f32; 4],
    glow_radius: f32,
    glow_steps: usize,
    animate_rays: bool,
    animate_glow: bool,
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model {
        egui,
        settings: Settings {
            ray_width: 3.0,
            ray_color: [0.0, 0.0, 0.0, 1.0],
            sun_color: [1.0, 1.0, 1.0, 1.0],
            sun_radius: 80.0,
            num_rays: 45,
            glow_color: [1.0, 1.0, 0.0, 0.3],
            glow_radius: 85.0,
            glow_steps: 12,
            animate_rays: false,
            animate_glow: false,
        },
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Ray width:");
        ui.add(egui::Slider::new(&mut model.settings.ray_width, 1.0..=25.0));

        ui.label("Sun radius:");
        ui.add(egui::Slider::new(&mut model.settings.sun_radius, 10.0..=250.0));

        ui.label("Number of rays:");
        ui.add(egui::Slider::new(&mut model.settings.num_rays, 8..=250));

        ui.label("Glow radius:");
        ui.add(egui::Slider::new(&mut model.settings.glow_radius, 0.0..=200.0));

        ui.label("Glow steps:");
        ui.add(egui::Slider::new(&mut model.settings.glow_steps, 0..=500));
        ui.separator();
        ui.label("Animate rays:");
        let clicked = ui.button("Animate rays").clicked();
        if clicked {
            model.settings.animate_rays = !model.settings.animate_rays;
        }
        ui.separator();
        ui.label("Animate glow:");
        let clicked = ui.button("Animate glow").clicked();
        if clicked {
            model.settings.animate_glow = !model.settings.animate_glow;
        }
        ui.separator();
        ui.label("Sun rays color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.ray_color = [rand::random(), rand::random(), rand::random(), 1.0];
        }
        ui.separator();
        ui.label("Sun color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.sun_color = [rand::random(), rand::random(), rand::random(), 1.0];
        }

        ui.separator();
        ui.label("Glow color:");
        let clicked = ui.button("Random color").clicked();
        if clicked {
            model.settings.glow_color = [rand::random(), rand::random(), rand::random(), 0.3];
        }
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let sun_center = pt2(app.window_rect().w() / 24.0, app.window_rect().h() / 24.0);
    let num_rays = model.settings.num_rays;
    let angle_step = 2.0 * PI / num_rays as f32;
    let window_width = app.window_rect().w();
    let window_height = app.window_rect().h();
    let ray_length = ((window_width / 2.0).powi(2) + (window_height / 2.0).powi(2)).sqrt();

    let ray_rotation = if model.settings.animate_rays {
        app.time
    } else {
        0.0
    };

    for i in 0..num_rays {
        let angle = i as f32 * angle_step + ray_rotation;
        let x = sun_center.x + ray_length * angle.cos();
        let y = sun_center.y + ray_length * angle.sin();
        draw.line()
        .start(sun_center)
        .end(pt2(x, y))
        .color(srgba(model.settings.ray_color[0], model.settings.ray_color[1], model.settings.ray_color[2], model.settings.ray_color[3]))
        .weight(model.settings.ray_width);
    }
    let glow_radius = model.settings.sun_radius + model.settings.glow_radius;
    let num_steps = model.settings.glow_steps;

    let max_glow_radius_offset = 100.0; 
    let glow_radius_offset = if model.settings.animate_glow {
        max_glow_radius_offset * (app.time * 2.0).sin()
    } else {
        0.0
    };
    for i in 0..=num_steps {
        let t = i as f32 / num_steps as f32;
        let radius = model.settings.sun_radius + t * (glow_radius - model.settings.sun_radius + glow_radius_offset);
        let alpha = (1.0 - t) * model.settings.glow_color[3];
        draw.ellipse()
            .xy(sun_center)
            .radius(radius)
            .color(srgba(
                model.settings.glow_color[0],
                model.settings.glow_color[1],
                model.settings.glow_color[2],
                alpha,
            ));
    }
        draw.ellipse()
            .xy(sun_center)
            .radius(model.settings.sun_radius)
            .color(srgba(model.settings.sun_color[0], model.settings.sun_color[1], model.settings.sun_color[2], model.settings.sun_color[3]));
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