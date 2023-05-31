    // mandelbrot set math based from https://github.com/plotters-rs/plotters/blob/master/plotters/examples/mandelbrot.rs

    use nannou::prelude::*;
    use std::f64::consts::PI;
    use nannou_egui::{self, egui, Egui};
    
    const WIDTH: u32 = 1366;
    const HEIGHT: u32 = 768;
    const MAX_ITER_START: usize = 1;
    const MAX_ITER_MIN: usize = 1;
    const MAX_ITER_LIMIT: usize = 200;
    const RESOLUTION: u32 = 2;
    const DELTA_ITER: usize = 1;
    
    enum IterDirection {
        Increasing,
        Decreasing,
    }
    
    struct Model {
        max_iter: usize,
        egui: Egui,
        scale: f32,
        settings: Settings,
        iter_direction: IterDirection,
    }
    
    struct Settings {
        a: f32,
    }
    
    fn main() {
        nannou::app(model).update(update).run();
    }
    
    fn model(app: &App) -> Model {
        let window_id = app
            .new_window()
            .size(WIDTH, HEIGHT)
            .view(view)
            .raw_event(raw_window_event)
            .build()
            .unwrap();
            let window = app.window(window_id).unwrap();
    
            let egui: Egui = Egui::from_window(&window);
            let settings = Settings {
                a: 1.0,
            };
    
        Model {
            max_iter: MAX_ITER_START,
            egui,
            scale: 1.0,
            settings,
            iter_direction: IterDirection::Increasing,
        }
    }
    
    
    fn update(_app: &App, model: &mut Model, _update: Update) {
        let egui = &mut model.egui;
        let settings = &mut model.settings;
        egui.set_elapsed_time(_update.since_start);
        let ctx = egui.begin_frame();
        egui::Window::new("Settings").show(&ctx, |ui| {
            ui.label("r:");
            ui.add(egui::Slider::new(&mut settings.a, 0.0..=1.0));
        });
    
        match model.iter_direction {
            IterDirection::Increasing => {
                if model.max_iter < MAX_ITER_LIMIT {
                    model.max_iter += DELTA_ITER;
                } else {
                    model.iter_direction = IterDirection::Decreasing;
                }
            },
            IterDirection::Decreasing => {
                if model.max_iter > MAX_ITER_MIN {
                    model.max_iter -= DELTA_ITER;
                } else {
                    model.iter_direction = IterDirection::Increasing;
                }
            },
        }
    }
    
    fn view(app: &App, model: &Model, frame: Frame) {
        let draw = app.draw().scale(model.scale);
    
        draw.background().color(BLACK);
    
        for y in (0..HEIGHT).step_by(RESOLUTION as usize) {
            for x in (0..WIDTH).step_by(RESOLUTION as usize) {
                let (scaled_x, scaled_y) = scale_coords(x, y);
                let c = (scaled_x, scaled_y);
                let mut z = (0.0, 0.0);
                let mut cnt = 0;
                while cnt < model.max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
                    z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                    cnt += 1;
                }
    
                let hue = 0.5 + 0.5 * ((app.time * 0.1) as f32 + 0.6 + 2.0 * PI as f32 * (cnt as f32 / model.max_iter as f32)).cos();
                let saturation = 0.5 + 0.5 * ((app.time * 0.1) as f32 + 0.8 + 2.0 * PI as f32 * (cnt as f32 / model.max_iter as f32)).cos();
                let value = 0.5 + 0.5 * ((app.time * 0.1) as f32 + 1.0 + 2.0 * PI as f32 * (cnt as f32 / model.max_iter as f32)).cos();
    
                draw.rect()
                    .w_h(RESOLUTION as f32, RESOLUTION as f32)
                    .x_y(
                        x as f32 - WIDTH as f32 / 2.0,
                        y as f32 - HEIGHT as f32 / 2.0,
                    )
                    .color(hsla(hue, saturation, value,model.settings.a));
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
    fn scale_coords(x: u32, y: u32) -> (f64, f64) {
        let scaled_x = (x as f64 / WIDTH as f64) * 2.6 - 2.1;
        let scaled_y = (y as f64 / HEIGHT as f64) * 2.4 - 1.2;
        (scaled_x, scaled_y)
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