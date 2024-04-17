use nannou::prelude::*;
use nannou::image::{open, RgbaImage};
use rand::{thread_rng, Rng};
use nannou_egui::{self, egui, Egui};
fn main() {
    nannou::app(model).update(update).run();
}
struct Particle {
    x: usize,
    y: usize,
    velocity: f32,
    size: f32,
}
#[derive(Clone, Copy, PartialEq)]
enum AnimationStyle {
    Normal,
    Matrix,
    Spiral,
    Wave,
    Waterfall,
    InvertedWaterfall,
    Diamond,
    Heart,
    Explosion,
    Altun,
}
impl Particle {
    fn new(width: usize, height: usize) -> Self {
        let mut rng = thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        let velocity = rng.gen_range(0.5..3.0);
        let size = rng.gen_range(0.5..1.0);
        Self { x, y, velocity, size }
    }
    fn update(&mut self, brightness_map: &[Vec<f32>]) {
        let speed = brightness_map[self.y][self.x];
        let delta_y = 2.0 - speed + self.velocity;
        self.y = (((self.y as f32) + delta_y + 1.0) as usize) % brightness_map.len();
    }
}
struct Model {
    img: RgbaImage,
    particles: Vec<Particle>,
    brightness_map: Vec<Vec<f32>>,
    egui: Egui,
    settings: Settings,
    prev_particle_count: usize,
    image_path: Option<String>,
    scale:f32,
}
struct Settings {
    l:f32,
    animation_style: AnimationStyle,
    particle_count: usize,
    u:f32,
    t:f32,
    open_file_dialog: bool,
    show_ui:bool,
}
fn model(app: &App) -> Model {
    let image_path = None;
    let img = RgbaImage::new(1, 1);
    let _window_id = app.new_window().size(800, 600).view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(_window_id).unwrap();
    let egui = Egui::from_window(&window);
    let (width, height) = img.dimensions();
    let mut brightness_map = vec![vec![0.0; width as usize]; height as usize];
    for (x, y, pixel) in img.enumerate_pixels() {
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        let brightness = relative_brightness(r, g, b);
        brightness_map[y as usize][x as usize] = brightness;
    }

    let settings = Settings {
        l: 0.3,
        particle_count: 500,
        animation_style: AnimationStyle::Normal,
        u: 3.0,
        t: 3.0,
        open_file_dialog: false,
        show_ui:true,
    };

    let initial_particle_count = settings.particle_count;
    let particles = (0..initial_particle_count)
        .map(|_| Particle::new(width as usize, height as usize))
        .collect();

    Model {
        img,
        particles,
        brightness_map,
        egui,
        settings,
        prev_particle_count: initial_particle_count,
        image_path,
        scale:1.0,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }  
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("Load Image").clicked() {
            model.settings.open_file_dialog = true;
        }
        ui.add(egui::Slider::new(&mut model.settings.l, 0.0..=10.0).text("L"));
        ui.add(egui::Slider::new(&mut model.settings.u, 0.1..=10.0).text("U"));
        ui.add(egui::Slider::new(&mut model.settings.t, 0.1..=10.0).text("T"));
        if ui.button("Switch Animation Style").clicked() {
            model.settings.animation_style = match model.settings.animation_style {
                AnimationStyle::Normal => AnimationStyle::Matrix,
                AnimationStyle::Matrix => AnimationStyle::Spiral,
                AnimationStyle::Spiral => AnimationStyle::Wave,
                AnimationStyle::Wave => AnimationStyle::Waterfall,
                AnimationStyle::Waterfall => AnimationStyle::InvertedWaterfall,
                AnimationStyle::InvertedWaterfall => AnimationStyle::Diamond,
                AnimationStyle::Diamond => AnimationStyle::Heart,
                AnimationStyle::Heart => AnimationStyle::Explosion,
                AnimationStyle::Explosion => AnimationStyle::Altun,
                AnimationStyle::Altun => AnimationStyle::Normal,
            };
        }
        let animation_style_number = model.settings.animation_style as i32 + 1;
        ui.label(format!("Current Animation Style: {}", animation_style_number));
        ui.add(egui::Slider::new(&mut model.settings.particle_count, 1..=1000).text("Particle Count"));
    });
    if model.prev_particle_count != model.settings.particle_count {
        model.particles = (0..model.settings.particle_count)
            .map(|_| {
                Particle::new(
                    model.img.width() as usize,
                    model.img.height() as usize,
                )
            })
            .collect();
        model.prev_particle_count = model.settings.particle_count;
    }
    for particle in &mut model.particles {
        particle.update(&model.brightness_map);
    }
    if model.settings.open_file_dialog {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            model.image_path = Some(path.display().to_string());
            let image_result = open(&model.image_path.as_ref().unwrap());
            if let Ok(image) = image_result {
                let new_img = image.to_rgba8(); 
                model.img = new_img;
                let (new_width, new_height) = model.img.dimensions();
                model.brightness_map = vec![vec![0.0; new_width as usize]; new_height as usize];
                for (x, y, pixel) in model.img.enumerate_pixels() {
                    let brightness = relative_brightness(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32);
                    model.brightness_map[y as usize][x as usize] = brightness;
                }
                model.particles = (0..model.settings.particle_count)
                    .map(|_| Particle::new(new_width as usize, new_height as usize))
                    .collect();
            } else {
                eprintln!("Failed to open image: {:?}", image_result.err());
            }
            model.settings.open_file_dialog = false;
        }
    }
}
pub fn relative_brightness(r: f32, g: f32, b: f32) -> f32 {
    ((r * r * 0.229) + (g * g * 0.587) + (b * b * 0.114)).sqrt() / 100.0
}
fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;
    let draw = app.draw().scale(model.scale); 
    let win = app.window_rect();
    let (img_width, img_height) = model.img.dimensions();
    let scale_factor = win.w().min(win.h()) / img_width.max(img_height) as f32;
    draw.background().color(BLACK);
    for particle in &model.particles {
        let pixel = model.img.get_pixel(particle.x as u32, particle.y as u32);
        let brightness = model.brightness_map[particle.y][particle.x];
        let x = (particle.x as f32 - (img_width / 2) as f32) * scale_factor;
        let y = ((img_height - particle.y as u32) as f32 - (img_height / 2) as f32) * scale_factor;
        match settings.animation_style {
            AnimationStyle::Normal => {
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );

                // let font_data = include_bytes!("../images/elv.ttf"); // for text rendering. download the font and put it in the images folder
                // let font = nannou::text::FontCollection::from_bytes(font_data as &[u8])
                //     .unwrap()
                //     .into_font()
                //     .unwrap();  

                // let text = "F   S";
                // let text = text.to_string();
                // let text = text.as_str();
                // //draw the text
                // draw.text(&text)
                //     .font(font)
                //     .x_y(x, y)
                //     .color(color);

                draw.ellipse()
                    .x_y(x*settings.t/3.0, y)
                    .radius(particle.size * scale_factor*settings.u)
                    .color(color);
            }
            AnimationStyle::Matrix => {
                let color: nannou::color::Alpha<rgb::Rgb, f32> = srgba(0.0, pixel[1] as f32 / 255.0, 0.0, brightness * settings.l);
                let tail_length = settings.u;
                for i in 0..tail_length as usize {
                    let y_offset = (i as f32) * scale_factor*settings.t/3.0;
                    let alpha = (tail_length - i as f32) / tail_length;
                    let adjusted_color = rgba(color.red, color.green, color.blue, color.alpha * alpha);
                    draw.line()
                        .start(pt2(x, y - y_offset))
                        .end(pt2(x, y - y_offset - scale_factor))
                        .color(adjusted_color)
                        .weight(particle.size * scale_factor);
                }
            }
            AnimationStyle::Spiral => {
                let angle = brightness * std::f32::consts::PI * settings.u;
                let r = particle.size * 11.0*settings.t/11.0 * scale_factor;
                let spiral_x = x + angle.cos() * r;
                let spiral_y = y + angle.sin() * r;
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                draw.line()
                    .start(pt2(x, y))
                    .end(pt2(spiral_x, spiral_y))
                    .color(color)
                    .weight(particle.size * scale_factor);
            }
            AnimationStyle::Wave => {
                let wave_length = settings.t.sin();
                let wave_amplitude = 3.0*settings.u* particle.size * scale_factor;
                let wave_x = x;
                let wave_y = y + (brightness * wave_length * std::f32::consts::PI).sin() * wave_amplitude;
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                draw.line()
                    .start(pt2(x, y))
                    .end(pt2(wave_x, wave_y))
                    .color(color)
                    .weight(particle.size * scale_factor);
            }
            AnimationStyle::Waterfall => {
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                let waterfall_factor = settings.u * particle.size * scale_factor;
                let waterfall_height = (brightness * settings.t * waterfall_factor).max(1.0);
            
                let y_normalized = y / win.h();
                let width_scale = 1.0 - (y_normalized * y_normalized); 
                let waterfall_width = waterfall_factor * width_scale;
            
                draw.rect()
                    .x_y(x * width_scale, y - waterfall_height / 2.0)
                    .w_h(waterfall_width, waterfall_height)
                    .color(color);
            }
            AnimationStyle::InvertedWaterfall => {
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                let waterfall_factor = settings.u * particle.size * scale_factor;
                let waterfall_height = (brightness * 2.0 * waterfall_factor).max(1.0);
            
                let y_normalized = y / win.h(); 
                let width_scale = y_normalized * y_normalized*settings.t; 
                let waterfall_width = waterfall_factor * width_scale*200.0;
            
                draw.rect()
                    .x_y(x * width_scale, y - waterfall_height / 2.0)
                    .w_h(waterfall_width, waterfall_height)
                    .color(color);
            }
            AnimationStyle::Diamond => {
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                let size = particle.size * settings.u * scale_factor;
                let half_size = size / settings.t*50.0;
                draw.polygon()
                    .color(color)
                    .points(vec![
                        pt2(x - half_size, y - half_size),
                        pt2(x, y + half_size),
                        pt2(x + half_size, y - half_size),
                    ]);
                draw.polygon()
                    .color(color)
                    .points(vec![
                        pt2(x - half_size, y + half_size),
                        pt2(x, y - half_size),
                        pt2(x + half_size, y + half_size),
                    ]);
            }
            AnimationStyle::Heart => {
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                let heart_scale = settings.u * particle.size * scale_factor;
                let heart_width = brightness * heart_scale;
                let heart_height = 5.0*settings.t * brightness * heart_scale;
                let x_offset = x * brightness;
            
                draw.polygon()
                    .points(vec![
                        pt2(x_offset, y),
                        pt2(x_offset - heart_width / 2.0, y - heart_height / 2.0),
                        pt2(x_offset + heart_width / 2.0, y - heart_height / 2.0),
                    ])
                    .color(color);
            
                let circle_y_offset = y + heart_height / 3.0;
            
                draw.ellipse()
                    .x_y(x_offset - heart_width / 34.0, circle_y_offset)
                    .radius(heart_width / 34.0)
                    .color(color);
                draw.ellipse()
                    .x_y(x_offset + heart_width / 34.0, circle_y_offset)
                    .radius(heart_width / 34.0)
                    .color(color);
            }
            AnimationStyle::Explosion => {
                let angle = brightness * std::f32::consts::PI * settings.u;
                let r = particle.size * 11.0 * settings.t / 11.0 * scale_factor;
                
                let z_x = 5.0 * angle.cos();
                let z_y = 5.0 * angle.sin();
                let zigzag_x = x + z_x * r;
                let zigzag_y = y + z_y * r;
                
                let z_pattern_x = if z_y.abs() < 0.5 {
                    zigzag_x - 0.5 * r
                } else if z_y.abs() > 0.5 && z_y.abs() < 1.0 {
                    zigzag_x + 0.5 * r
                } else {
                    zigzag_x
                };
                
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                draw.line()
                    .start(pt2(x, y))
                    .end(pt2(z_pattern_x, zigzag_y))
                    .color(color)
                    .weight(particle.size * scale_factor);
            }
            AnimationStyle::Altun => {
                let angle = brightness * std::f32::consts::PI * settings.u;
                let r = particle.size * 45.0 * settings.t / 11.0 * scale_factor;
                let branching_x = x + angle.cos() * r;
                let branching_y = y + angle.sin() * r;
                
                let color = srgba(
                    pixel[0] as f32 / 255.0,
                    pixel[1] as f32 / 255.0,
                    pixel[2] as f32 / 255.0,
                    brightness * settings.l,
                );
                let branch_factor = (particle.size * settings.u).max(1.0);
                for i in 0..branch_factor as i32 {
                    let branch_angle = angle + i as f32 * std::f32::consts::PI / branch_factor;
                    let branch_x = branching_x + r * branch_angle.cos();
                    let branch_y = branching_y + r * branch_angle.sin();
                    draw.ellipse()
                        .xy(pt2(branch_x, branch_y))
                        .radius(particle.size * scale_factor)
                        .color(color);
                }
            }
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