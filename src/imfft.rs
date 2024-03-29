// This code serves as an experimental venture into real-time image processing using Fast Fourier Transform (FFT) with the Nannou library. 
// Initially, the goal was to perform simple low-pass filtering on an image in the frequency domain. 
// However, the focus shifted to explore dynamic and evolving visual patterns through FFT-based image manipulation. 
// The code performs FFT on individual color channels of the image and applies a filter that is modulated over time. 
// It uses Nannou for visualization, fft2d for the FFT calculations, and nalgebra for matrix operations.

// One of the key lines that affect the visual output is: 
//     filtered_img[(y, x)] = old_val * Complex::new(smoothing as f64, 0.0) + new_val * Complex::new((1.0 - smoothing) as f64, 0.0);
// Here, `old_val` and `new_val` are complex numbers representing the original and filtered frequency components of the image, respectively. 
// The term `smoothing` modulates how much of the old frequency content is retained. 
// When this line is modified to:
//     filtered_img[(y, x)] = old_val * Complex::new(smoothing as f64, 0.0) + new_val * Complex::new((15.0 - smoothing) as f64, 0.0);
// It amplifies the new frequency components by a factor of 15, leading to more pronounced and intricate visual patterns.

// Currently, the code successfully generates dynamic patterns but faces challenges with maintaining the visibility of the original image over time.
// Further refinements and experimentation are planned, including potential egui implementations for real-time control over parameters.
use std::path::PathBuf;
use nannou::prelude::*;
use nannou::wgpu::Texture;
use fft2d::nalgebra::{fft_2d, fftshift, ifft_2d, ifftshift};
use nannou::image::{open, DynamicImage};
use nalgebra::DMatrix;
use rustfft::num_complex::Complex;

fn get_image_path(relative_path: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(relative_path)
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    img: DynamicImage,
    texture: Option<Texture>,
    current_radius: f32,
}

fn model(app: &App) -> Model {
    let img_path = get_image_path("images/mona.jpg");
    let img = open(img_path).unwrap().to_rgb8();
    let (width, height) = img.dimensions();
    
    let _w_id = app.new_window().size(width, height).view(view).build().unwrap();
    
    Model {
        img: DynamicImage::ImageRgb8(img),
        texture: None,
        current_radius: 100.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.current_radius += 0.001;
    if model.current_radius > 1.0 {
        model.current_radius = 0.0;
    }

    let img = model.img.to_rgb8();
    let (width, height) = img.dimensions();
    
    let mut channels = [vec![], vec![], vec![]];
    for pixel in img.pixels() {
        let rgb = pixel;
        channels[0].push(Complex::new(rgb[0] as f64 / 255.0, 0.0));
        channels[1].push(Complex::new(rgb[1] as f64 / 255.0, 0.0));
        channels[2].push(Complex::new(rgb[2] as f64 / 255.0, 0.0));
    }

    let fft_filter = create_fft_filter(height as usize, width as usize, model.current_radius);
    
    let mut img_buffer = img.clone();
    for channel in 0..3 {
        let mut img_matrix = DMatrix::from_vec(width as usize, height as usize, channels[channel].clone());
        
        img_matrix = fft_2d(img_matrix);
        img_matrix = fftshift(&img_matrix);
        
        let filtered_img_buffer = apply_filter(&img_matrix, &fft_filter, 0.01, &img_matrix);

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
    
    model.img = DynamicImage::ImageRgb8(img_buffer);
    model.texture = Some(Texture::from_image(_app, &model.img));

}
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let draw = app.draw();
    if let Some(texture) = &model.texture {
        draw.texture(texture);
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
          .project_path()
          .expect("failed to locate project directory")
          .join("frames") 
          .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path); 
    
    } 
}

fn create_fft_filter(height: usize, width: usize, strength: f32) -> DMatrix<f64> {
    let mut filter = DMatrix::zeros(height, width);
    let center_x = width as f32 / 2.0;
    let center_y = height as f32 / 2.0;
    let radius = strength * ((width.pow(2) + height.pow(2)) as f32).sqrt() / 2.0;

    for y in 0..height {
        for x in 0..width {
            let dx = (x as f32 - center_x).abs();
            let dy = (y as f32 - center_y).abs();
            let distance = (dx.powi(2) + dy.powi(2)).sqrt();
            if distance <= radius {
                filter[(y, x)] = 1.0;
            }
        }
    }
    filter
}

fn apply_filter(img: &DMatrix<Complex<f64>>, filter: &DMatrix<f64>, step_size: f64, last_img: &DMatrix<Complex<f64>>) -> DMatrix<Complex<f64>> {
    let mut filtered_img = DMatrix::zeros(img.nrows(), img.ncols());
    let smoothing = 1.0; 
    for (i, img_val) in img.iter().enumerate() {
        let y = i / img.ncols();
        let x = i % img.ncols();
        let filter_val = filter[(y, x)] * step_size + (0.01 - step_size); 
        let new_val = img_val * Complex::new(filter_val, 0.0);
        let old_val = last_img[(y, x)];
        filtered_img[(y, x)] = old_val * Complex::new(smoothing, 0.0) + new_val * Complex::new(15.0 - smoothing, 0.0);
    }
    filtered_img
}

