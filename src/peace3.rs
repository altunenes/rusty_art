use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    circle_points: Vec<Vec<Point2>>,
    settings: Settings,
    egui: Egui,
}

struct Settings {
    num_circles: usize,
    const_: f32,
    frequency: f32,
    amplitude: f32,
    phase: f32,
    num_points: usize,
    radius: f32,
    r: f32,
    x: f32,
    y: f32,
    z: f32,
    t:f32,
    c: usize,
    ani: bool,
    noise: Perlin,
    p:f32,
    fourier_terms: usize,

    use_perlin_noise: bool,

}

fn fourier_coefficients(n: usize) -> (f32, f32) {
    match n {
        0 => (0.0, 0.0),
        _ => {
            let a_n = ((-1.0_f32).powi(n as i32) * 2.0) / (n as f32 * PI);
            let b_n = 0.0;
            (a_n, b_n)
        }
    }
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
    let settings = Settings {
        num_circles: 255,
        const_: 25.0,
        frequency: 125.0,
        amplitude: 1.4,
        phase: 1.0,
        num_points: 155,
        radius: 1.5,
        r:4.0,
        x: 2.0,
        y: 2.0,
        z: 2.0,
        t:0.5,
        c: 0,
        ani: true,
        noise: Perlin::new(),
        use_perlin_noise: false,
        p: 1.0,
        fourier_terms: 100,


    };
    let circle_points = generate_circle_points(&settings, &window.rect());

    Model {
        circle_points,
        settings,
        egui,
        
    }
}

fn generate_circle_points(settings: &Settings, window_rect: &Rect) -> Vec<Vec<Point2>> {
    let center = window_rect.xy();
    let radius = window_rect.w().min(window_rect.h()) / settings.z;
    let circle_radius = radius / (settings.num_circles as f32);
    let mut circle_points = Vec::with_capacity(settings.num_circles);

    for i in 0..settings.num_circles {
        let mut points = Vec::with_capacity(settings.num_points);
        for j in 0..settings.num_points {
            let angle = j as f32 * 2.0 * PI / (settings.num_points as f32);
            let mut x_fourier = 0.0;
            let mut y_fourier = 0.0;

            for n in 0..=settings.fourier_terms {
                let (a_n, b_n) = fourier_coefficients(n);
                x_fourier += a_n * (n as f32 * angle).cos() + b_n * (n as f32 * angle).sin();
                y_fourier += a_n * (n as f32 * angle).sin() - b_n * (n as f32 * angle).cos();
            }

            let x = center.x + x_fourier * circle_radius * (i as f32 + settings.x);
            let y = center.y + y_fourier * circle_radius * (i as f32 + settings.y);
            points.push(pt2(x, y));
        }
        circle_points.push(points);
    }

    circle_points
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Num Circles:");
        ui.add(egui::Slider::new(&mut settings.num_circles, 1..=255));
        ui.label("Const:");
        ui.add(egui::Slider::new(&mut settings.const_, 1.0..=255.0));
        ui.label("Frequency:");
        ui.add(egui::Slider::new(&mut settings.frequency, 1.0..=255.0));
        ui.label("Amplitude:");
        ui.add(egui::Slider::new(&mut settings.amplitude, 1.0..=255.0));
        ui.label("Phase:");
        ui.label("Num Points:");
        ui.add(egui::Slider::new(&mut settings.num_points, 1..=500));
        ui.label("Radius:");
        ui.add(egui::Slider::new(&mut settings.radius, 0.1..=10.0));
        ui.label("R:");
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=10.0));
        ui.label("X:");
        ui.add(egui::Slider::new(&mut settings.x, 0.1..=10.0));
        ui.label("Y:");
        ui.add(egui::Slider::new(&mut settings.y, 0.1..=10.0));
        ui.label("Z:");
        ui.add(egui::Slider::new(&mut settings.z, 0.1..=10.0));
        ui.label("T:");
        ui.add(egui::Slider::new(&mut settings.t, 0.1..=10.0));
        ui.label("P:");
        ui.add(egui::Slider::new(&mut settings.p, 0.1..=10.0));
        ui.checkbox(&mut settings.use_perlin_noise, "Use Perlin Noise");
        ui.label(format!("Current Color Pattern: {}", settings.c));
        if ui.button("Switch Color Pattern").clicked() {
            settings.c = (settings.c + 1) % 14;
        }
        if ui.button("Toggle Background Animation").clicked() {
            settings.ani = !settings.ani;
        }
    });
    model.circle_points = generate_circle_points(&model.settings, &app.window_rect());

    for i in 0..model.settings.num_circles {
        for j in 0..model.settings.num_points {
            let angle = j as f32 * 2.0 * PI / (model.settings.num_points as f32);
            let mut x_fourier = 0.0;
            let mut y_fourier = 0.0;
    
            for n in 0..=model.settings.fourier_terms {
                let (a_n, b_n) = fourier_coefficients(n);
                x_fourier += a_n * (n as f32 * angle).cos() + b_n * (n as f32 * angle).sin();
                y_fourier += a_n * (n as f32 * angle).sin() - b_n * (n as f32 * angle).cos();
            }
    
            let x = model.circle_points[i][j].x + x_fourier * model.settings.amplitude * (model.settings.phase + j as f32 * PI).cos();
            let y = model.circle_points[i][j].y + y_fourier * model.settings.amplitude * (model.settings.phase + j as f32 * PI).sin();
    
            model.circle_points[i][j] = pt2(x, y);
    
            if model.settings.use_perlin_noise {
                let noise_value = model.settings.noise.get([model.circle_points[i][j].x as f64, model.circle_points[i][j].y as f64, app.time as f64]) as f32;
                let x_offset = noise_value * model.settings.amplitude * (app.time + j as f32 * PI).cos() * (1.0 + 0.5 * (app.time + i as f32 * PI / 2.0).sin());
                let y_offset = noise_value * model.settings.amplitude * (app.time + j as f32 * PI).sin() * (1.0 + 0.5 * (app.time + i as f32 * PI / 2.0).sin());
                model.circle_points[i][j] = pt2(x + x_offset, y + y_offset);
            }
        }
    }
    model.settings.phase += 0.01;
}
            
            fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
            model.egui.handle_raw_event(event);
            }
            
            fn view(app: &App, model: &Model, frame: Frame) {
            let settings = &model.settings;
            let draw = app.draw();
            if settings.ani {
                draw.background().color(hsla(app.time.sin() as f32 / 2.0, 0.5, 0.5, 1.0));
            } else {
                draw.background().color(BLACK);
            }           
            for i in 0..settings.num_circles {
                let progress = i as f32 / settings.num_circles as f32;
                let color = match settings.c {
                    0 => {
                        let hue: f32 = settings.t * ((progress * 2.0 * PI) + app.time).sin() + 0.5;
                        hsla(hue, 0.6, 0.5, 1.0)
                    }
                    1 => {
                        let t = app.time;
                        let hue = progress + settings.t  * (t + i as f32 * 0.1).cos();
                        let saturation = progress +settings.t  * (t + i as f32 * 0.2).cos();
                        let lightness = progress + settings.t  * (t + i as f32 * 0.3).cos();
                        hsla(hue, saturation, lightness, 1.0)

                    }
                    2 => {
                        let hue: f32 = settings.t * ((progress * 2.0 * PI) + app.time).sin() + 0.5;
                        hsla(hue, 1.0, 0.5, 1.0)
                    }

                    3 => {
                        let hue: f32 = 1.5 * app.time.cos() * progress;
                        let saturation = 1.5 + 1.5 * settings.t;
                        let lightness = 1.5 + 0.5 * progress;
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    4 => {
                        let hue: f32 = 0.5 + 0.5 * (settings.t+app.time + progress * PI).sin();
                        let saturation = progress;
                        let lightness = 0.4 + 0.4 * (settings.t+app.time + progress * PI).cos();
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    5 => {
                        let hue = progress;
                        let saturation = 1.0 - progress;
                        let lightness = 0.4 + 0.4 * (settings.t+app.time + progress * PI).sin();
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    6 => {
                        let t = app.time;
                        let hue = progress + settings.t * (t + i as f32 * 0.15).sin();
                        let saturation = 0.5 + settings.t * 0.5 * (t + i as f32 * 0.25).cos();
                        let lightness = 0.5 + settings.t * 0.25 * (t + i as f32 * 0.1).sin() + settings.t * 0.25 * (t + i as f32 * 0.2).cos();
                        hsla(hue, saturation, lightness, 1.0)
                    }

                    7 => {
                        let t = app.time;
                        let hue = progress + settings.t * (t + i as f32 * 0.12).cos();
                        let saturation = 0.5 + settings.t * 0.3 * (t + i as f32 * 0.2).sin();
                        let lightness = 0.4 + settings.t * 0.4 * (t + i as f32 * 0.25).cos() + settings.t * 0.2 * (t + i as f32 * 0.15).sin();
                        hsla(hue, saturation, lightness, 1.0)
                    }

                    8 => {
                        let t = app.time;
                        let hue = progress + settings.t * (t + i as f32 * 0.22).sin();
                        let saturation = 1.5 + settings.t * 1.5 * (t + i as f32 * 0.18).cos();
                        let lightness = 0.5 + settings.t * 0.35 * (t + i as f32 * 0.3).sin() + settings.t * 0.15 * (t + i as f32 * 0.28).cos();
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    9 => {
                        let golden_ratio: f32 = (1.0 + 5.0f32.sqrt()).sin() / 2.0;
                        let t = app.time.sin();
                        let hue = progress.sin() * 1.0 + settings.t * 30.0;
                        let saturation = 0.5 + 0.5 * ((golden_ratio * t).sin() * 0.5 + 0.5);
                        let lightness = 0.5 + 0.5 * ((golden_ratio * t * 2.0).sin() * 0.5 + 0.5);
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    10 => {
                        let hue = progress;
                        let saturation = 1.0 - progress;
                        let lightness = 0.5 + 0.5 * (settings.t + app.time + progress * PI).sin();
                        let adjusted_lightness = if lightness > 0.8 { 0.3 } else { lightness };
                        hsla(hue, saturation, adjusted_lightness, 1.0)
                    }
                    11 => {
                        let hue = progress;
                        let saturation = settings.t - progress;
                        let lightness = settings.z + 0.5 * progress;
                        let adjusted_lightness = if lightness > settings.y { 0.3 } else { lightness };
                        hsla(hue, saturation, adjusted_lightness, 1.0)
                    }
                    12 => {
                        let perlin = Perlin::new();
                        let hue = perlin.get([progress as f64, app.time as f64]) as f32;
                        let saturation = 1.0 - progress;
                        let lightness = 0.5 + 0.5 * (settings.t + app.time + progress * PI).sin();
                        hsla(hue, saturation, lightness, 1.0)
                    }

                    
                    13 => {
                        let perlin = Perlin::new();
                        let hue = 0.5 + 0.5 * perlin.get([progress as f64, app.time as f64]) as f32;
                        let saturation = progress;
                        let lightness = 0.5 + 0.5 * (settings.t + app.time + progress * PI).cos();
                        hsla(hue, saturation, lightness, 1.0)
                    }
                    _ => unreachable!(),


                };
            
                draw.polyline()
                    .weight(settings.radius)
                    .points(model.circle_points[i].clone())
                    .color(color);
                draw.polyline()
                    .weight(settings.radius)
                    .points(model.circle_points[i].iter().map(|p| pt2(-p.x, p.y)).collect::<Vec<Point2>>())
                    .color(color);
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