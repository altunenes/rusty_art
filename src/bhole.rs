use nannou::{color::white_point::A, prelude::*};
use std::f32::consts::PI;
use nannou_egui::{self, egui, Egui};

const N: usize = 2; 
const U: usize = 700;
const T: f32 =2.5;
struct Parameters {
    N: usize,
    U: usize,
    alpha: f32,
    beta: f32,
    a: f32,
    show_ui: bool,

}

fn main() {
    nannou::app(m).update(u).run(); 
}
struct M {
    b: Vec<Point2>,
    s: Vec<S>,
    egui: Egui,
    parameters: Parameters,
}
fn raw_window_event(_app: &App, m: &mut M, event: &nannou::winit::event::WindowEvent) {
    m.egui.handle_raw_event(event);
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
struct S {  
    p: Point2,  
    sp: Point2,  
    c: f32, 
}
impl M {
    
    fn new(app: &App) -> Self {
        let window_id = app
            .new_window()
            .size(1920, 1080)
            .view(v)
            .raw_event(raw_window_event)
            .build()
            .unwrap();
        let window = app.window(window_id).unwrap();
        let egui = Egui::from_window(&window);
        let parameters = Parameters {
            N: 2,
            U: 700,
            alpha: 50.0,
            beta: 180.0,
            a: 40.0,
            show_ui: true,


        };
        let s: Vec<S> = Vec::with_capacity(parameters.U);

        M {  
            b: vec![pt2(0.0, 0.0); N],
            s, 
            egui,
            parameters,

        }


    }
    fn create_s(parameters: &Parameters) -> Vec<S> {
        let mut s: Vec<S> = Vec::with_capacity(parameters.U);
        for i in 0..parameters.U { 
            let a = (i as f32 / parameters.U as f32) * 360.0 * PI; 
            let r =   parameters.beta;
            let p = pt2(a.cos() * r, a.sin() * r); 
            s.push(S { 
                p,  
                sp: p, 
                c: (i as f32 % 360.0) / 360.0, 
            });
        }
        s
    }
}
impl S {
    fn update(&mut self, b: &[Point2], i: usize) { 
        let mut a = 0.0;  
        for (j, bh) in b.iter().enumerate() { 
            let d = *bh - self.p;  // diff -> d
            let ba = d.y.atan2(d.x);  
            let aa = (i as f32 * j as f32 * 0.00001).sin() * 10.0 * PI; 
            a += ba + aa; 
            if d.length() < T {  
                self.p = self.sp; 
                return;
            }
        }
        self.p += pt2(a.cos() * T, a.sin() * T); 
    }
}
fn m(app: &App) -> M { 
    M::new(app)  // Model -> M
}
fn u(app: &App, m: &mut M, _u: Update) { 
    let egui = &mut m.egui;
    let mut reset = false;
    let _parameters = &m.parameters;
    egui.set_elapsed_time(_u.since_start);
    if app.keys.down.contains(&Key::H) {
        m.parameters.show_ui = !m.parameters.show_ui;
    }
    let ctx = egui.begin_frame();
    egui::Window::new("Parameters").show(&ctx, |ui| {
        ui.label("N:");
        ui.add(egui::Slider::new(&mut m.parameters.N, 0..=100));
        ui.label("U:");
        ui.add(egui::Slider::new(&mut m.parameters.U, 0..=1000));
        ui.label("alpha:");
        ui.add(egui::Slider::new(&mut m.parameters.alpha, 0.0..=360.0));
        ui.label("beta:");
        ui.add(egui::Slider::new(&mut m.parameters.beta, 0.0..=360.0));
        ui.label("a:");
        ui.add(egui::Slider::new(&mut m.parameters.a, 0.0..=360.0));
        if ui.button("run").clicked() {
            reset = true;
        }
    });
    if reset {
        m.b = vec![pt2(0.0, 0.0); m.parameters.N];
        m.s = M::create_s(&m.parameters);
    }
    let t = app.time.sin() / 10.0; // t -> t
    for (i, bh) in m.b.iter_mut().enumerate() {  
        let a = (t * i as f32) * m.parameters.a * PI; 
        *bh = pt2(a.cos() * i as f32 * m.parameters.alpha, a.sin() * i as f32 * m.parameters.alpha); 
    }
    for (i, s) in m.s.iter_mut().enumerate() {  
        s.update(&m.b, i); 
    }
}
fn v(app: &App, m: &M, f: Frame) { 
    m.egui.draw_to_frame(&f).unwrap();
    let d = app.draw();  
    d.background().color(BLACK); 
    for (_i, s) in m.s.iter().enumerate() {
        let progress = s.c*PI;
        let hue = 0.5 * (s.p.x + app.window_rect().w() / 2.0) / app.window_rect().w();
        let saturation = progress+0.5;
        let lightness = 0.2 + 0.1 * (0.7 + app.time + progress * PI).cos();
        d.ellipse()
            .xy(s.p)
            .radius(2.0)
            .color(hsla(hue, saturation, lightness, 1.0));
        d.ellipse()
            .xy(pt2(-s.p.x, s.p.y))
            .radius(2.0)
            .color(hsla(hue + 0.5, saturation, lightness, 1.0));
    }
    d.to_frame(app, &f).unwrap();
    if m.parameters.show_ui {
        m.egui.draw_to_frame(&f).unwrap();
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