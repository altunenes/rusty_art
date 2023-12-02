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
    scale_factor:f32,
    is_interacting_with_gui: bool,

}
#[derive(PartialEq)]
enum DrawingState {
    UserDrawing,
    FourierDrawing,
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
            scale_factor:1.0,
            is_interacting_with_gui: false,


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

    if model.drawing_state == DrawingState::UserDrawing && !model.is_interacting_with_gui {
        if app.mouse.buttons.left().is_down() {
            let mouse_pos = app.mouse.position();
            if model.user_drawing.last() != Some(&mouse_pos) {
                model.user_drawing.push(mouse_pos);
            }
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
        ui.add(egui::Slider::new(&mut model.scale_factor, 0.0..=1.0).text("Scale"));
        
    });
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    match model.drawing_state {
        DrawingState::UserDrawing => draw_user_input(&draw, &model.user_drawing),
        DrawingState::FourierDrawing => {
            let mut path = model.path.borrow_mut();
            let cycle_complete = app.time % (2.0 * PI) < 0.01;

            if cycle_complete && !path.is_empty() {
                path.clear();
            }

            if !cycle_complete {
                draw_fourier_cycloids(&draw, &model.fourier_data, &mut path, app.time, model.draw_speed,model.scale_factor);
            }
        },
    }
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
fn draw_user_input(draw: &Draw, points: &[Point2]) {
    if points.len() > 1 {
        for window in points.windows(2) {
            draw.line()
                .start(window[0])
                .end(window[1])
                .color(WHITE);
        }
    }
}
fn compute_dft(points: &[Point2]) -> Vec<FourierComponent> {
    let n = points.len();
    let mut fourier_components: Vec<FourierComponent> = (0..n).map(|k| {
        let mut sum = Complex { re: 0.0, im: 0.0 };
        for (i, point) in points.iter().enumerate() {
            let phi = (2.0 * PI * k as f32 * i as f32) / n as f32;
            let c = Complex::new(phi.cos(), -phi.sin());
            sum = sum + c * Complex::new(point.x, point.y);
        }
        sum = sum / n as f32;
        FourierComponent {
            amp: sum.norm(),
            freq: k as f32, 
            phase: sum.arg(),
        }
    }).collect();
    fourier_components.sort_by(|a, b| b.amp.partial_cmp(&a.amp).unwrap());
    for (i, component) in fourier_components.iter_mut().enumerate() {
        component.freq = i as f32;
    }
    fourier_components
}
fn draw_fourier_cycloids(draw: &Draw, fourier_data: &[FourierComponent], path: &mut Vec<Point2>, time: f32, speed: f32,scale_factor:f32) {
    if fourier_data.is_empty() {
        return;
    }
    let mut x = 0.0;
    let mut y = 0.0;
    for comp in fourier_data {
        let prev_x = x;
        let prev_y = y;
        let scale_factor = scale_factor;
        x += scale_factor * comp.amp * (comp.freq * time * speed + comp.phase).cos();
        y += scale_factor * comp.amp * (comp.freq * time * speed + comp.phase).sin();
        draw.line()
            .start(pt2(prev_x, prev_y))
            .end(pt2(x, y))
            .color(WHITE);
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
            draw.line()
                .start(points[0])
                .end(points[1])
                .stroke_weight(2.0)
                .color(color);
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