use nannou::image::{open, DynamicImage, GenericImageView, Rgba};
use nannou::prelude::*;
use nannou::wgpu::Texture;
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: Option<DynamicImage>,
    egui: Egui,
    zoom: f32,
    texture: Option<Texture>,
    scale: f32,
    settings: Settings,
}

#[allow(dead_code)]
struct Settings {
    use_real_colors: bool,
    colors: usize,
    sampling: usize,
    min_radius: f32,
    max_radius: f32,
    open_file_dialog: bool,
    show_ui: bool,
}
fn model(app: &App) -> Model {
    let _w_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(_w_id).unwrap();
    let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        colors: 1,
        use_real_colors: false,
        sampling: 8,
        min_radius: 0.05,
        max_radius: 2.0,
        open_file_dialog: false,
        show_ui: true,
    };
    Model {
        img: None,
        texture: None,
        scale: 1.0,
        egui,
        settings,
        zoom: 1.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let egui = &mut model.egui;

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("Load Image").clicked() {
            model.settings.open_file_dialog = true;
        }
        ui.label(format!("color {}", model.settings.colors));
        if ui.button("next").clicked() {
            model.settings.colors = (model.settings.colors % 3) + 1;
        }
        ui.add(egui::Slider::new(&mut model.settings.sampling, 4..=10).text("sampling"));
        ui.add(egui::Slider::new(&mut model.settings.min_radius, 0.01f32..=10.0f32).text("min radius"));
        ui.add(egui::Slider::new(&mut model.settings.max_radius, 0.01f32..=10.0f32).text("max radius"));
        ui.add(egui::Checkbox::new(&mut model.settings.use_real_colors, "Use Real Colors"));
    });
    if model.settings.open_file_dialog {
        if let Some(file_path) = FileDialog::new().pick_file() {
            if let Ok(img) = open(&file_path) {
                model.img = Some(img);
                model.texture = Some(Texture::from_image(app, model.img.as_ref().unwrap()));
            }
            model.settings.open_file_dialog = false;
        }
    }
    if model.img.is_some() {
        model.texture = model.img.as_ref().map(|img| Texture::from_image(app, img));
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);
    if let Some(ref img) = model.img {
        let draw = app.draw().scale(model.zoom);
        let (img_width, img_height) = img.dimensions();
        let step_size = model.settings.sampling;
        let time = app.time;
        for y in (0..img_height).step_by(step_size) {
            for x in (0..img_width).step_by(step_size) {
                let pixel = img.get_pixel(x, y);
                let luminance = calculate_luminance(&pixel);
                let oscillation = (time.sin() * 0.5 + 0.5).abs();
                let radius_state1 = map_range(luminance, 0.0, 1.0, model.settings.min_radius, model.settings.max_radius);
                let radius_state2 = map_range(luminance, 0.0, 1.0, model.settings.max_radius, model.settings.min_radius);
                let radius = radius_state1 * oscillation + radius_state2 * (1.0 - oscillation);
                let x: f32 = (x as f32 - img_width as f32 / 2.0) * model.scale;
                let y = ((img_height - y) as f32 - img_height as f32 / 2.0) * model.scale;
                let color = if model.settings.use_real_colors {
                    rgba(pixel.0[0] as f32 / 255.0, pixel.0[1] as f32 / 255.0, pixel.0[2] as f32 / 255.0, 1.0)
                } else {
                    match model.settings.colors {
                        1 => {
                            let progress = y / img_height as f32; 
                            let hue = progress;
                            let saturation = 1.0 - progress;
                            let lightness =  0.4 * (0.5 + time + progress * PI).sin();
                            hsla(hue, saturation, lightness, 1.0)
                        }
                        2 => hsla(0.0, 0.0, 0.0, 1.0),
                        3 => { 
                            let hue = (y / img_height as f32).fract();
                            let saturation = 1.0;
                            let lightness = 0.5;
                            hsla(hue, saturation, lightness, 1.0)
                        },
                        _ => unreachable!(),
                    }.into()
                };

                draw.ellipse()
                    .x_y(x, y)
                    .radius(radius)
                    .color(color);
            }
        }

        draw.to_frame(app, &frame).unwrap();
    }

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
fn calculate_luminance(pixel: &Rgba<u8>) -> f32 {
    let r = pixel[0] as f32 / 255.0;
    let g = pixel[1] as f32 / 255.0;
    let b = pixel[2] as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
        let cursor_over_egui = model.egui.ctx().wants_pointer_input();
        if !cursor_over_egui {
            match delta {
                nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                    model.zoom *= 1.0 + *y * 0.05;
                    model.zoom = model.zoom.max(0.1).min(10.0);
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

