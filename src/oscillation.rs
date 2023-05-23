use nannou::image::{open, RgbaImage};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: RgbaImage,
    settings: Settings,
    egui: Egui,

}
struct Settings {
    st:f32,
    sw_x: f32,
    sw_y: f32,
    vc: f32,
    v: f32,
    n: usize,
    c: usize,
    use_real_colors: bool,

}
fn model(app: &App) -> Model {
    let img_path = "images/ferris.jpg";
    let img = open(img_path).unwrap().to_rgba8();

    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        st: 10.0,
        sw_x: 5.0,
        sw_y: 0.7,
        vc: 0.01,
        v: 100.0,
        n: 20,
        c: 1,
        use_real_colors: false,
    };
    Model { img, settings, egui}
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Settings");
        ui.add(egui::Slider::new(&mut settings.st, 5.0..=100.0).text("st"));
        ui.add(egui::Slider::new(&mut settings.sw_x, 1.0..=40.0).text("sw_x"));
        ui.add(egui::Slider::new(&mut settings.sw_y, 0.0..=10.0).text("sw_y"));
        ui.add(egui::Slider::new(&mut settings.vc, 0.01..=10.0).text("vc"));
        ui.add(egui::Slider::new(&mut settings.v, 0.0..=1000.0).text("v"));
        ui.add(egui::Slider::new(&mut settings.n, 1..=100).text("n"));
        ui.label(format!("color {}",settings.c));
        if ui.button("next").clicked(){
            settings.c = (settings.c%12)+1;
        }
        ui.add(egui::Checkbox::new(&mut settings.use_real_colors, "Use Real Colors"));

    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let settings = &model.settings;

    let draw = app.draw();
    draw.background().color(GRAY);

    for x_center in (0..model.img.width()).step_by(settings.n) {
        draw_vertical_line(&model.img, &draw, x_center, app.elapsed_frames(), &model, app.time);
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

fn draw_vertical_line(img: &RgbaImage, draw: &Draw, x_center: u32, frame_count: u64, model: &Model,app_time: f32) {
    let settings = &model.settings;

    let mut prev_y = 0;
    let step = settings.st as u32;
    let height = img.height();
    let width = img.width();
    let vibration_scale = settings.vc;

    for y in (step..height).step_by(step as usize) {
        let pixel_color = img.get_pixel(x_center, y);
        let b = pixel_color.0[0] as f32 / 255.0;
        let amp = map_range(b, 0.0, 1.0, 50.0, 0.0) * vibration_scale.sin();
        let sw_scaling = map_range((frame_count as f32 / settings.v).sin(), -1.0, 1.0, settings.sw_y, settings.sw_x);
        let sw = map_range(b, 0.0, 1.0, 10.0, 1.0) * sw_scaling;

        let x_offset = (width / 2) as f32;
        let y_offset = (height / 2) as f32;
        let x = if amp != 0.0 {
            x_center as f32 + random_range(-amp, amp) - x_offset
        } else {
            x_center as f32 - x_offset
        };        let y_pos = y_offset - y as f32;

        let color = if settings.use_real_colors {
            let r = pixel_color.0[0] as f32 / 255.0;
            let g = pixel_color.0[1] as f32 / 255.0;
            let b = pixel_color.0[2] as f32 / 255.0;
        
            let r = 0.5 * (1.0 + (app_time + r * PI).sin());
            let g = 0.5 * (1.0 + (app_time + g * PI).sin());
            let b = 0.5 * (1.0 + (app_time + b * PI).sin());
            rgba(r, g, b, 1.0)
        } else {
            match settings.c {
                1 =>
            {
                let hue = map_range(b, 0.0, 1.0, 0.0, 1.0);
                let saturation = 1.0 - map_range(b, 0.0, 1.0, 0.0, 1.0);
                let lightness =  0.4 * (0.5 + app_time + map_range(b, 0.0, 1.0, 0.0, 1.0) * PI).sin();
                hsla(hue, saturation, lightness, 1.0)
            }
            2 => {
                let progress = y as f32 / height as f32; 
                let hue = progress;
                let saturation = 1.0 - progress;
                let lightness =  0.4 * (0.5 + app_time + progress * PI).sin();
                hsla(hue, saturation, lightness, 1.0)
            }
            3 => { 
                let hue = map_range(b, 0.0, 1.0, 0.0, 0.15);
                let saturation = 1.0;
                let lightness = 0.5 * (1.0 + app_time.sin());
                hsla(hue, saturation, lightness, 1.0)
            }
            4 => { 
                let hue = 0.08;
                let saturation = map_range(b, 0.0, 1.0, 0.5, 1.0);
                let lightness = 0.5 * (1.0 + (app_time + b * PI).sin());
                hsla(hue, saturation, lightness, 1.0)
            }
            5 => {
                let hue = app_time % 1.0;
                let saturation = 1.0;
                let lightness = b;
                hsla(hue, saturation, lightness, 1.0)
            }
            6 => {
                let hue = b;
                let saturation = (app_time % 1.0).abs() * 2.0 - 1.0;
                let lightness = b;
                hsla(hue, saturation, lightness, 1.0)
            }
            7 => {
                let hue = y as f32 / height as f32;
                let saturation = (app_time % 1.0).abs() * 2.0 - 1.0;
                let lightness = 0.5 * (1.0 + (app_time * 2.0 * PI).sin());
                hsla(hue, saturation, lightness, 1.0)
            }
            8 => { 

                hsla(1.0, 1.0, 1.0, 1.0)
            }
            9 => {
                if b > 0.1 {
                    let inverted_b = 1.0 - b;
                    let hue = (inverted_b as f32) % 1.0;
                    let saturation = 1.0;
                    let lightness = 0.5;
                    hsla(hue, saturation, lightness, 1.0)
                } else { 
                    hsla(0.0, 0.0, 0.5, 1.0)
                }
            }
            10 => {
                let progress = y as f32 / height as f32; 
                let hue = progress;
                let saturation = 1.0 - progress;
                let lightness = 0.5 * (0.5 + progress * PI).sin().abs(); 
                hsla(hue, saturation, lightness, 1.0)
            }

            11 => {
                hsla(0.0, 0.0, 0.0, 1.0)
            }
            12 => {
                let inverted_b = 1.0 - b;
                let hue = map_range(inverted_b, 0.0, 1.0, 0.0, 0.15);
                let saturation = 1.0;
                let lightness = 0.5 * (1.0 + app_time.sin());
                hsla(hue, saturation, lightness, 1.0)
            }

            _ => unreachable!(),
        }.into()
    };
        draw.line()
            .points(pt2(x, y_offset - prev_y as f32), pt2(x, y_pos)) 
            .color(color)
            .weight(sw);

        prev_y = y;
    }
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    }