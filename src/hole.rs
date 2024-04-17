use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    phase: f32,
    egui: Egui,
    settings: Settings,
    scale: f32,
}
struct Settings {
    num_points: usize,
    freq: f32,
    scale2: f32,
    r: f32,
    y: f32,
    r2: f32,
    x: f32,
    z: f32,
    u: f32,
    sf: f32,
    sr: f32,
    l:f32,
    g_a: f32,
    g_v: f32,
    g_b: f32,
    g_v2: f32,
    v: f32,
    use_stroke_color: bool,
    show_ui:bool,

}

fn gabor_noise(u: Vec2, a: f32, v: f32) -> f32 {
    let sin_cos = (a + std::f32::consts::FRAC_PI_2).to_radians().sin_cos();
    
    (-0.5 * u.dot(u) * 1e3).exp() * (40.0 * u.dot(Vec2::from(sin_cos))).sin() - v
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        num_points: 500,
        freq: 45.0,
        scale2: 600.0,
        r: 40.0,
        r2: 2.0,
        y: 0.1,
        x: 0.6,
        z: 0.5,
        u: 1.0,
        sf: 10.0,
        sr: 0.002,
        l: 0.4,
        g_a:0.8,
        g_b:0.8,
        g_v:10.0,
        g_v2:10.0,
        v: 0.01,
show_ui:true,
        use_stroke_color: false,
    };    
    Model { phase: 0.0, egui, settings ,scale: 1.0}
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
        ui.add(egui::Slider::new(&mut settings.num_points, 0..=1000).text("num_points"));
        ui.add(egui::Slider::new(&mut settings.freq, 0.0..=100.0).text("freq"));
        ui.add(egui::Slider::new(&mut settings.scale2, 0.0..=1000.0).text("scale"));
        ui.add(egui::Slider::new(&mut settings.r, 0.1..=200.0).text("r"));
        ui.add(egui::Slider::new(&mut settings.r2, 0.1..=200.0).text("r2"));
        ui.add(egui::Slider::new(&mut settings.y, 0.0..=1.0).text("y"));
        ui.add(egui::Slider::new(&mut settings.x, 0.0..=1.0).text("x"));
        ui.add(egui::Slider::new(&mut settings.z, 0.0..=1.0).text("z"));
        ui.add(egui::Slider::new(&mut settings.u, 0.0..=1.0).text("u"));
        ui.add(egui::Slider::new(&mut settings.sf, 0.0..=100.0).text("sf"));
        ui.add(egui::Slider::new(&mut settings.sr, 0.0..=10.1).text("sr"));
        ui.add(egui::Slider::new(&mut settings.l, 0.0..=10.0).text("l"));
        ui.add(egui::Slider::new(&mut settings.g_a, 0.0..=10.0).text("g_a"));
        ui.add(egui::Slider::new(&mut settings.g_b, 0.0..=10.0).text("g_b"));
        ui.add(egui::Slider::new(&mut settings.g_v, 0.0..=100.0).text("g_v"));
        ui.add(egui::Slider::new(&mut settings.g_v2, 0.0..=100.0).text("g_v2"));
        ui.add(egui::Checkbox::new(&mut settings.use_stroke_color, "Use Stroke Color"));
        ui.add(egui::Slider::new(&mut settings.v, 0.0..=100.0).text("v"));




    });    
       model.phase += model.settings.v;
}
fn gauss(x: f32) -> f32 {
    (-10.0 * x * x).exp()
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    let _win = app.window_rect();


    let phase = model.phase;
    let spiral_factor = model.settings.sf;
    let spiral_radius = model.settings.sr;
    draw.background().color(BLACK);
    let freq = model.settings.freq;
    let num_points = model.settings.num_points;
    let scale = model.settings.scale2;
    let center1 = pt2(0.0, 0.0);
    let center2 = pt2(0.0, 0.0);
    for i in 0..num_points {
        let t = map_range(i, 0, num_points, 0.0, 1.0);
        let x = (t - model.settings.y) * scale;
        let g = gauss(x / scale);
        let y1 = g * model.settings.x * (freq * t * model.settings.freq* PI + phase).sin() * scale;
        let y2 = g * model.settings.x * (freq * t * model.settings.freq * PI - phase).sin() * scale;

        let progress = i as f32 / num_points as f32;
        let hue = progress;
        let saturation = 1.0+model.settings.u - progress;
        let lightness = model.settings.z+ model.settings.z * (app.time + progress * PI).sin();
        let color1 = hsla(hue, saturation, lightness, g);
        let spiral_offset1 = spiral_offset(i, num_points, spiral_factor, spiral_radius);
        let gabor1 = gabor_noise(vec2(x, y1), model.settings.g_a, model.settings.g_v * (0.5 * (i as f32) + 0.2).sin() * (6.0 * app.time + 0.3 * (i as f32)).sin());
        let gabor2 = gabor_noise(vec2(-x, y2), model.settings.g_b, model.settings.g_v2 * (0.5 * (i as f32) + 0.2).sin() * (6.0 * app.time + 0.3 * (i as f32)).sin());

        let hue2: f32 = model.settings.z + model.settings.z * (app.time + progress * PI).sin();
        let saturation2 = progress;
        let lightness2: f32 = model.settings.z + 0.5 * (app.time + progress * PI).cos();
        let color2 = hsla(hue2, saturation2, lightness2, 1.0);

        let point1 = pt2(center1.x + x + spiral_offset1.x + gabor1, center1.y + y1 + spiral_offset1.y + gabor1);
        let spiral_offset2 = spiral_offset(i, num_points, spiral_factor, spiral_radius);
        let point2 = pt2(-center2.x - x - spiral_offset2.x + gabor2, center2.y + y2 + spiral_offset2.y + gabor2);

        if model.settings.use_stroke_color {
            draw.ellipse()
                .xy(point1)
                .radius(model.settings.r)
                .color(color1)
                .stroke_weight(model.settings.r2)
                .stroke_color(color2);

            
            draw.ellipse()
                .xy(point2)
                .radius(model.settings.r)
                .color(color1)
                .stroke_weight(model.settings.r2)
                .stroke_color(color2);
        } else {
            draw.ellipse()
                .xy(point1)
                .radius(model.settings.r)
                .color(color1);
            
            
            draw.ellipse()
                .xy(point2)
                .radius(model.settings.r)
                .color(color1);


        }
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
    fn spiral_offset(index: usize, num_points: usize, factor: f32, radius: f32) -> Vec2 {
        let angle = index as f32 * factor * TAU / num_points as f32;
        let r = radius * angle;
        vec2(angle.cos() * r, angle.sin() * r)
    }
    

