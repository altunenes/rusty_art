use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    layers: Vec<Vec<Point2>>,
    colors: Vec<Rgb>,
    balls: Vec<Ball>,
    travel_time: f32,
    egui: Egui,
    settings: Settings,
    scale:f32,
}
struct Settings {
    speed: f32,
    num_balls: usize,
    show_ui:bool,
}
struct Ball {
    position: Point2,
    source: Point2,
    target: Point2,
    progress: f32,
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

    let layers = vec![
        create_layer(3, -400.0),
        create_layer(12, -250.0),
        create_layer(6, -100.0),
        create_layer(8, 50.0),
        create_layer(4, 200.0),
        create_layer(3, 350.0),
    ];
    let colors = vec![
        Rgb::new(0.5, 0.3, 0.7),
        Rgb::new(0.3, 0.6, 0.9),
        Rgb::new(0.8, 0.2, 0.4),
        Rgb::new(0.9, 0.7, 0.1),
    ];
    let settings = Settings {
        num_balls : 5,
        speed : 1.0,
        show_ui:true,
    };
        
    let balls = (0..settings.num_balls)
        .map(|_| {
            let source = layers[0][rand::random::<usize>() % layers[0].len()];
            let target = layers[1][rand::random::<usize>() % layers[1].len()];
            Ball {
                position: source,
                source,
                target,
                progress: 0.0,
            }
        })
        .collect();
    Model {
        scale:1.0,
        layers,
        colors,
        balls,
        travel_time: 2.0,
        settings,
        egui,

    }
}
fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let settings = &mut model.settings;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Number of balls");
        ui.add(
            egui::Slider::new(&mut settings.num_balls, 1..=10)
                .text("Number of balls"),
        );
        ui.label("Speed");
        ui.add(
            egui::Slider::new(&mut settings.speed, 0.0..=10.0)
                .text("Speed"),
        );
    });
    if model.balls.len() != settings.num_balls {
        model.balls = (0..settings.num_balls)
            .map(|_| {
                let source = model.layers[0][rand::random::<usize>() % model.layers[0].len()];
                let target = model.layers[1][rand::random::<usize>() % model.layers[1].len()];
                Ball {
                    position: source,
                    source,
                    target,
                    progress: 0.0,
                }
            })
            .collect();
    }
    let delta_time = update.since_last.secs() as f32;
    let speed = settings.speed / model.travel_time;
    for ball in &mut model.balls {
        ball.progress += delta_time * speed;
        if ball.progress >= 1.0 {
            let current_layer = model
                .layers
                .iter()
                .position(|layer| layer.contains(&ball.target));
            if let Some(layer_idx) = current_layer {
                if layer_idx < model.layers.len() - 1 {
                    let next_layer = &model.layers[layer_idx + 1];
                    let next_node = rand::random::<usize>() % next_layer.len();
                    ball.source = ball.target;
                    ball.target = next_layer[next_node];
                    ball.progress = 0.0;
                } else {
                    let first_layer = &model.layers[0];
                    let first_node = rand::random::<usize>() % first_layer.len();
                    ball.source = first_layer[first_node];
                    ball.target = model.layers[1][0];
                    ball.progress = 0.0;
                }
            }
        }
        ball.position = ball.source.lerp(ball.target, ball.progress);
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
        draw.background().color(BLACK);
    for (i, layer) in model.layers.iter().enumerate() {
        for point in layer {
            let distance = model.balls[0].position.distance(*point);
            let brightness = 1.0 - distance / 800.0;
            let rgb_color = model.colors[i % model.colors.len()];
            let hsla_color = Hsl::from(rgb_color);
            let color = Hsla::new(
                hsla_color.hue,
                hsla_color.saturation,
                hsla_color.lightness * brightness,
                1.0,
            );
            draw.ellipse().xy(*point).radius(4.0).color(color);
        }
    }
    for i in 0..(model.layers.len() - 1) {
        for a in &model.layers[i] {
            for b in &model.layers[i + 1] {
                let distance_a = model.balls[0].position.distance(*a);
                let distance_b = model.balls[0].position.distance(*b);
                let distance = distance_a.min(distance_b);
                let brightness = 1.0 - distance / 600.0;
                let rgb_color = model.colors[i % model.colors.len()];
                let hsla_color = Hsl::from(rgb_color);
                let color = Hsla::new(
                    hsla_color.hue,
                    hsla_color.saturation,
                    hsla_color.lightness * brightness,
                    1.0,
                );
                draw.line()
                    .points(*a, *b)
                    .color(color);
            }
        }
    }
    for ball in &model.balls {
        draw.ellipse()
            .xy(ball.position)
            .radius(5.0)
            .color(WHITE);
    }
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
fn create_layer(num_nodes: usize, x: f32) -> Vec<Point2> {
    let mut layer = Vec::with_capacity(num_nodes);
    let step = 800.0 / (num_nodes + 1) as f32;
    for i in 1..=num_nodes {
        let y = i as f32 * step - 400.0;
        layer.push(pt2(x, y));
    }
    layer
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