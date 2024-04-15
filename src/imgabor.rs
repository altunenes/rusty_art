// Note: It's not intended to be used as traditional GABOR filter.
// it's just aimed to create some interesting patterns.
// But of course, if you want to use it as Gabor filter, after modifying the code, you can do it.
use nannou::prelude::*;
use nannou::wgpu::Texture;
use fft2d::nalgebra::{fft_2d, fftshift, ifft_2d, ifftshift};
use nannou::image::{open, DynamicImage};
use nalgebra::DMatrix;
use rustfft::num_complex::Complex;
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
fn main() {
    nannou::app(model).update(update).run();
}
struct Model {
    img: Option<DynamicImage>,
    texture: Option<Texture>,
    progress: f64, 
    last_img: Option<DynamicImage>, 
    egui: Egui,
    settings: Settings,
}
struct Settings {
    sigma: f64,
    theta: f64,
    kx: f64,
    ky: f64,
    v:f64,
    limit: f64,
    open_file_dialog: bool,
    show_ui: bool,
}
fn model(app: &App) -> Model {
    let _w_id = app.new_window().size(800, 600).view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(_w_id).unwrap();
    let egui = Egui::from_window(&window);
    let settings = Settings {
        sigma: 255.0,
        theta: 0.0,
        kx: 0.05,
        ky: 105.0,
        v: 0.001,
        limit:1.0,
        open_file_dialog: false,
        show_ui: true,
    };
    Model {
        img: None,
        texture: None,
        progress: 0.0,
        last_img: None,
        egui,
        settings,
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    if _app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    let ctx = model.egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        if ui.button("Load Image").clicked() {
            model.settings.open_file_dialog = true;
        }
        ui.add(egui::Slider::new(&mut model.settings.sigma, 0.0..=256.0).text("sigma"));
        ui.add(egui::Slider::new(&mut model.settings.theta, 0.0..=360.0).text("theta"));
        ui.add(egui::Slider::new(&mut model.settings.kx, 0.0..=256.0).text("kx"));
        ui.add(egui::Slider::new(&mut model.settings.ky, 0.0..=256.0).text("ky"));
        ui.add(egui::Slider::new(&mut model.settings.v, 0.0000..=0.05).text("v"));
        ui.add(egui::Slider::new(&mut model.settings.limit, 0.0..=10.0).text("limit"));
    });
    if model.settings.open_file_dialog {
        if let Some(file_path) = FileDialog::new().pick_file() {
            if let Ok(img) = open(file_path) {
                model.img = Some(img);
                model.last_img = None; 
                model.progress = 0.0; 
            }
            model.settings.open_file_dialog = false;
        }
    }
    model.progress += model.settings.v; 
    if model.progress > model.settings.limit {
        model.progress -= model.settings.v;
    }
    if let Some(ref img) = model.img {
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let mut channels = [vec![], vec![], vec![]];
        for pixel in img.pixels() {
            let rgb = pixel;
            channels[0].push(Complex::new(rgb[0] as f64 / 255.0, 0.0));
            channels[1].push(Complex::new(rgb[1] as f64 / 255.0, 0.0));
            channels[2].push(Complex::new(rgb[2] as f64 / 255.0, 0.0));
        }
        let kx_ratio = model.settings.kx;
        let ky_ratio = model.settings.ky;
        let theta = model.settings.theta;
        let sigma: f64 = model.settings.sigma;
        let gabor_filter = create_gabor_filter(height as usize, width as usize, kx_ratio, ky_ratio, theta, sigma);
        let mut img_buffer = img.clone();
        for channel in 0..3 {
            let mut img_matrix = DMatrix::from_vec(width as usize, height as usize, channels[channel].clone());
            img_matrix = fft_2d(img_matrix);
            img_matrix = fftshift(&img_matrix);
            let filtered_img_buffer = img_matrix.component_mul(&gabor_filter);
            img_matrix = ifftshift(&filtered_img_buffer);
            img_matrix = ifft_2d(img_matrix);
            let fft_coef = 1.0 / (width * height) as f64;
            for x in img_matrix.iter_mut() {
                *x *= fft_coef;
            }
            let img_data: Vec<u8> = img_matrix.iter().map(|c| (c.norm().min(1.0) * 255.0) as u8).collect();
            for (i, val) in img_data.iter().enumerate() {
                let x = (i % width as usize) as u32;
                let y = (i / width as usize) as u32;
                let pixel = img_buffer.get_pixel_mut(x, y);
                pixel[channel] = *val;
            }
        }
        if let Some(last_img) = &model.last_img {
            let last_img = last_img.to_rgb8();
            for (i, pixel) in img_buffer.pixels_mut().enumerate() {
                let last_pixel = last_img.get_pixel(i as u32 % width, i as u32 / width);
                pixel[0] = (pixel[0] as f64 * model.progress + last_pixel[0] as f64 * (1.0 - model.progress)) as u8;
                pixel[1] = (pixel[1] as f64 * model.progress + last_pixel[1] as f64 * (1.0 - model.progress)) as u8;
                pixel[2] = (pixel[2] as f64 * model.progress + last_pixel[2] as f64 * (1.0 - model.progress)) as u8;
            }
        }
        model.last_img = Some(DynamicImage::ImageRgb8(img_buffer.clone()));
        model.img = Some(DynamicImage::ImageRgb8(img_buffer));
        model.texture = model.img.as_ref().map(|img| Texture::from_image(_app, img));
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    if let Some(texture) = &model.texture {
        draw.texture(texture);
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
fn create_gabor_filter(height: usize, width: usize, kx_ratio: f64, ky_ratio: f64, theta: f64, sigma: f64) -> DMatrix<Complex<f64>> {
    let mut filter = DMatrix::zeros(height, width);
    let center_x = width as f64 / 2.0;
    let center_y = height as f64 / 2.0;
    for y in 0..height {
        for x in 0..width {
            let dx = (x as f64 - center_x).abs();
            let dy = (y as f64 - center_y).abs();
            let x_theta = theta.cos() * dx + theta.sin() * dy;
            let y_theta = -theta.sin() * dx + theta.cos() * dy;
            let gaussian = (-0.5 * (x_theta.powi(2) / sigma.powi(2) + y_theta.powi(2) / sigma.powi(2))).exp();
            let sinusoid = Complex::new(0.0, 2.0 * std::f64::consts::PI * (kx_ratio * x_theta + ky_ratio * y_theta)).exp();
            filter[(y, x)] = gaussian * sinusoid;
        }
    }
    filter
}
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}