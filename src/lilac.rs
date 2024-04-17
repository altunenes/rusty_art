use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

struct Model {
    timer: f32,
    egui: Egui,
    settings: Settings,
    noise_circles: Vec<Point2>,
    scale: f32,
}

struct Settings {
    num_circles: usize,
    c_radius: f32,
    angle: f32,
    c_pos: f32,
    timer: f32,
    color: egui::Color32,
    bgcolor: egui::Color32,
    clockwise: bool,
    noise: bool,
    noise_radius: f32,
    show_ui: bool,
}
fn main() {
    nannou::app(model).update(update).run();
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
    
    let num_noise_circles = 100;
    let win = app.window_rect();
    let noise_circles = (0..num_noise_circles).map(|_| {
        let x = random_range(-win.w() / 2.0, win.w() / 2.0);
        let y = random_range(-win.h() / 2.0, win.h() / 2.0);
        pt2(x, y)
    }).collect();

    Model { 
        timer: 0.0, 
        scale:1.0,
        egui, 
        settings: Settings {
            num_circles: 12,
            c_radius: 0.02,
            angle: 1.0,
            c_pos: 0.25,
            timer: 10.0,
            clockwise: true,
            noise: false,
            noise_radius: 4.0,
            color: egui::Color32::from_rgb(217, 151, 217),
            bgcolor: egui::Color32::from_rgb(128,128,128),
            show_ui:true,
        },
        noise_circles
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
            egui::Window::new("Settings").show(&ctx, |ui| {
                ui.label("num_circles:");
                ui.add(egui::Slider::new(&mut settings.num_circles, 1..=100).text("num_circles"));
                ui.label("c_radius:");
                ui.add(egui::Slider::new(&mut settings.c_radius, 0.01..=1.0).text("c_radius"));
                ui.label("angle:");
                ui.add(egui::Slider::new(&mut settings.angle, 0.01..=10.0).text("angle"));
                ui.label("c_pos:");
                ui.add(egui::Slider::new(&mut settings.c_pos, 0.01..=1.0).text("c_pos"));
                ui.label("timer:");
                ui.add(egui::Slider::new(&mut settings.timer, 0.01..=50.0).text("timer"));
                ui.label("Clockwise rotation:");
                ui.checkbox(&mut settings.clockwise, "Enable");
                ui.label("Noise:");
                ui.checkbox(&mut settings.noise, "Enable");
                ui.label("Noise circle radius:");
                ui.add(egui::Slider::new(&mut settings.noise_radius, 0.01..=20.0).text("noise_radius"));

                ui.horizontal(|ui| {
                    ui.label("bgcolor:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut settings.bgcolor,
                        egui::color_picker::Alpha::Opaque,
                    );
                });
                ui.horizontal(|ui| {
                    ui.label("crcolor:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut settings.color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });
            });

            if model.settings.noise {
                let speed = 2.1;
                for pt in &mut model.noise_circles {
                    pt.y += speed;
                    if pt.y > _app.window_rect().h() / 2.0 {
                        pt.y = -_app.window_rect().h() / 2.0;
                        pt.x = random_range(-_app.window_rect().w() / 2.0, _app.window_rect().w() / 2.0);
                    }
                }
            }

            model.timer = _app.time;
        }
        fn view(app: &App, model: &Model, frame: Frame) {
            let draw = app.draw().scale(model.scale);
        
            draw.background().color(srgba(
                model.settings.bgcolor.r(),
                model.settings.bgcolor.g(),
                model.settings.bgcolor.b(),
                model.settings.bgcolor.a(),
            ));
        
        
            let timer = model.timer;
            let win = app.window_rect();
        
            let num_circles = model.settings.num_circles;
            let circle_radius = win.w().min(win.h()) * model.settings.c_radius;
        
            let aspect_ratio = (win.w() / win.h())*model.settings.angle;
            if model.settings.noise {
                let noise_circle_radius = model.settings.noise_radius;
                for &pt in &model.noise_circles {
                    draw.ellipse()
                        .xy(pt)
                        .radius(noise_circle_radius)
                        //make the color a bit transparent
                        .color(srgba(
                            model.settings.color.r(),
                            model.settings.color.g(),
                            model.settings.color.b(),
                            40,

                        ));
                }
            }

for i in 0..num_circles {
    let angle = if model.settings.clockwise {
        i as f32 / num_circles as f32 * 2.0 * PI + 4.25
    } else {
        -(i as f32) / num_circles as f32 * 2.0 * PI + 4.25
    };
                let circle_pos = pt2(
                    (win.w() * model.settings.c_pos * angle.sin()) / aspect_ratio,
                    win.h() * model.settings.c_pos * angle.cos(),
                );
                let current_time = timer * model.settings.timer;
                if i as i32 != (current_time as i32 % num_circles as i32) {
                    draw.ellipse()
                        .xy(circle_pos)
                        .radius(circle_radius)
                        .color(srgba(
                            model.settings.color.r(),
                            model.settings.color.g(),
                            model.settings.color.b(),
                            model.settings.color.a(),
                        ));
                }
            }
            draw.line()
                .points(pt2(-20.0, 0.0), pt2(20.0, 0.0))
                .color(RED);
            draw.line()
                .points(pt2(0.0, -20.0), pt2(0.0, 20.0))
                .color(RED);
        

    draw.to_frame(app, &frame).unwrap();
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();}
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