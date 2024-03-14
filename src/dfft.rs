// Inspriation: Daniel Shiffman's Coding Challenge: Drawing with Fourier Transform and Epicycles
//draw something when the windows is open then let the fourier cycle draw it :-)
// note that, my aim was not the exact path from the user input, I just wanted to see the fourier cycle in action in some random way but of course, it still 
// some how follows the path of the user input :-)
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use std::f32::consts::PI;
use std::cell::RefCell;
fn main() {
    nannou::app(model)
        .update(update)
        .run();
}
struct Model {
    egui: Egui,
    drawing_state: DrawingState,
    user_drawing: Vec<Point2>,
    fourier_data: Vec<FourierComponent>,
    path: RefCell<Vec<Point2>>,
    draw_speed: f32,
    is_interacting_with_gui: bool,
    stroke_weight: f32,
    drawing_method: DrawingMethod,
    fourier_drawing_method: FourierDrawingMethod,


}
#[derive(PartialEq)]
enum DrawingState {
    UserDrawing,
    FourierDrawing,
}
#[derive(Copy, Clone)]
enum DrawingMethod {
    Line,
    Ellipse,
}
#[derive(Copy,Clone)]
enum FourierDrawingMethod {
    Line,
    Ellipse,
}

struct FourierComponent {
    amp: f32,
    freq: f32,
    phase: f32,
}
impl Model {
    fn new(window_id: WindowId, app: &App) -> Self {
        let window = app.window(window_id).unwrap();
        let egui = Egui::from_window(&window);
        Model {
            egui,
            drawing_state: DrawingState::UserDrawing,
            user_drawing: Vec::new(),
            fourier_data: Vec::new(),
            path: RefCell::new(Vec::new()),
            draw_speed: 1.0,
            is_interacting_with_gui: false,
            stroke_weight: 2.0,
            drawing_method: DrawingMethod::Line,
            fourier_drawing_method: FourierDrawingMethod::Line,


        }
    }
}
fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    Model::new(window_id, app)
}
fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    if model.drawing_state == DrawingState::UserDrawing && !model.is_interacting_with_gui && app.mouse.buttons.left().is_down() {
        let mouse_pos = app.mouse.position();
        if model.user_drawing.is_empty() || (model.user_drawing.last().unwrap().distance(mouse_pos) > 1.0) {
            model.user_drawing.push(mouse_pos);
        }
    }
    model.is_interacting_with_gui = model.egui.ctx().is_pointer_over_area();

    let ctx = model.egui.begin_frame();
    egui::Window::new("Control Panel").show(&ctx, |ui| {
        if ui.button("Compute Fourier Transform").clicked() {
            model.drawing_state = DrawingState::FourierDrawing;
            model.fourier_data = compute_dft(&model.user_drawing);
            model.path.borrow_mut().clear();
        }
        if ui.button("Reset Drawing").clicked() {
            model.drawing_state = DrawingState::UserDrawing;
            model.user_drawing.clear();
            model.fourier_data.clear();
            model.path.borrow_mut().clear();
        }
        ui.add(egui::Slider::new(&mut model.draw_speed, 0.0..=1.0).text("Speed"));
        ui.add(egui::Slider::new(&mut model.stroke_weight, 0.1..=10.0).text("Thickness"));

        ui.horizontal(|ui| {
            ui.label("User Drawing Method: ");

            if ui.button("Line").clicked() {
                model.drawing_method = DrawingMethod::Line;
            }
            if ui.button("Ellipse").clicked() {
                model.drawing_method = DrawingMethod::Ellipse;
            }
        });
        ui.horizontal(|ui| {
            ui.label("Fourier Drawing Method: ");

            if ui.button("Line").clicked() {
                model.fourier_drawing_method = FourierDrawingMethod::Line;
            }
            if ui.button("Ellipse").clicked() {
                model.fourier_drawing_method = FourierDrawingMethod::Ellipse;
            }
        });
        
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    match model.drawing_state {
        DrawingState::UserDrawing => draw_user_input(&draw, &model.user_drawing, model.drawing_method),
        DrawingState::FourierDrawing => {
            let mut path = model.path.borrow_mut();
            let cycle_complete = app.time % (2.0 * PI) < 0.01;

            if cycle_complete && !path.is_empty() {
                path.clear();
            }

            if !cycle_complete {
                draw_fourier_cycloids(&draw, &model.fourier_data, &mut path, app.time, model.draw_speed,model.stroke_weight, model.fourier_drawing_method);
            }
        },
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
fn draw_user_input(draw: &Draw, points: &[Point2], drawing_method: DrawingMethod) {
    match drawing_method {
        DrawingMethod::Line => {
            if points.len() > 1 {
                for window in points.windows(2) {
                    draw.line()
                        .start(window[0])
                        .end(window[1])
                        .color(WHITE);
                }
            }
        },
        DrawingMethod::Ellipse => {
            for point in points {
                draw.ellipse()
                    .x_y(point.x, point.y)
                    .radius(1.0)
                    .color(WHITE);
            }
        },
    }
}
fn compute_dft(points: &[Point2]) -> Vec<FourierComponent> {
    let n = points.len() as f32;
    let mut fourier_components = Vec::new();
    let n_half = (n as isize) / 2;

    for k in -n_half..=n_half {
        let mut sum = Complex::new(0.0, 0.0);
        for (i, point) in points.iter().enumerate() {
            let angle = (2.0 * PI * k as f32 * i as f32) / n;
            let c = Complex::from_polar(1.0, -angle);
            sum = sum + c * Complex::new(point.x, point.y);
        }
        sum = sum / n;
        fourier_components.push(FourierComponent {
            amp: sum.norm(),
            freq: k as f32, 
            phase: sum.arg(),
        });
    }
    fourier_components.sort_by(|a, b| b.amp.partial_cmp(&a.amp).unwrap());
    fourier_components
}

fn draw_fourier_cycloids(draw: &Draw, fourier_data: &[FourierComponent], path: &mut Vec<Point2>, time: f32, speed: f32,stroke_weight: f32, fourier_drawing_method: FourierDrawingMethod) {
    if fourier_data.is_empty() {
        return;
    }

    let mut x = 0.0;
    let mut y = 0.0;
    for (index, comp) in fourier_data.iter().enumerate() {
        let prev_x = x;
        let prev_y = y;
        x += comp.amp * (comp.freq * time * speed + comp.phase).cos();
        y += comp.amp * (comp.freq * time * speed + comp.phase).sin();
        let transparency = if index < 2 { 0.0 } else { 1.0 };
        draw.line()
            .start(pt2(prev_x, prev_y))
            .end(pt2(x, y))
            .color(rgba(1.0, 1.0, 1.0, transparency));
        let hue = comp.freq / fourier_data.len() as f32;
        draw.ellipse()
            .x_y(prev_x, prev_y)
            .radius(comp.amp)
            .no_fill()
            .stroke_color(hsla(hue, 0.8, 0.5, 0.8 * transparency))
            .stroke_weight(1.0);
    }
    path.push(pt2(x, y));

    if time < 0.01 || (path.is_empty() || path.last() != Some(&pt2(x, y))) {
        path.push(pt2(x, y));
    }
    if path.len() > 1 {
        path.windows(2).enumerate().for_each(|(i, points)| {
            let color = hsla(
                i as f32 / path.len() as f32,
                1.0,                          
                0.5,                        
                1.0,                         
            );

            match fourier_drawing_method {
                FourierDrawingMethod::Line => {
                    draw.line()
                        .start(points[0])
                        .end(points[1])
                        .color(color)
                        .stroke_weight(stroke_weight);
                },
                FourierDrawingMethod::Ellipse => {
                    draw.ellipse()
                        .x_y(points[0].x, points[0].y)
                        .radius(stroke_weight)
                        .color(color);
                },
            }


        });
    }
}
#[derive(Copy, Clone)]
struct Complex {
    re: f32,
    im: f32,
}
impl Complex {
    fn new(re: f32, im: f32) -> Self {
        Complex { re, im }
    }
    fn norm(&self) -> f32 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }
    fn arg(&self) -> f32 {
        self.im.atan2(self.re)
    }
    fn from_polar(r: f32, theta: f32) -> Self {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }
}
impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}
impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}
impl std::ops::Div<f32> for Complex {
    type Output = Complex;
    fn div(self, rhs: f32) -> Complex {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}