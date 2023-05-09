use nannou::prelude::*;
use std::f32::consts::PI;

const N: usize = 2; 
const U: usize = 250;
const T: f32 =2.5;

fn main() {
    nannou::app(m).update(u).run(); 
}
struct M {  
    b: Vec<Point2>, 
    s: Vec<S>, 
}
struct S {  
    p: Point2,  
    sp: Point2,  
    c: f32, 
}
impl M {
    fn new() -> Self {
        let mut s = Vec::with_capacity(U); 
        for i in 0..U { 
            let a = (i as f32 / U as f32) * 720.0 * PI; 
            let r = 180.0;  // radius -> r
            let p = pt2(a.cos() * r, a.sin() * r); 
            s.push(S { 
                p,  
                sp: p, 
                c: (i as f32 % 360.0) / 360.0, 
            });
        }
        M {  
            b: vec![pt2(0.0, 0.0); N],
            s, 
        }
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
    app.new_window().size(1400, 960).view(v).build().unwrap();  // view -> v
    M::new()  // Model -> M
}
fn u(app: &App, m: &mut M, _u: Update) {        
    let t = app.time.sin() / 10.0; // t -> t
    for (i, bh) in m.b.iter_mut().enumerate() {  
        let a = (t * i as f32) * 8.0 * PI; 
        *bh = pt2(a.cos() * i as f32 * 40.0, a.sin() * i as f32 * 40.0); 
    }
    for (i, s) in m.s.iter_mut().enumerate() {  
        s.update(&m.b, i); 
    }
}
fn v(app: &App, m: &M, f: Frame) { 
    let d = app.draw();  
    d.background().color(BLACK); 
    for s in &m.s {
        let progress = s.c;
        let hue = 0.5 + 0.5 * (0.4 + app.time + progress * PI).sin();
        let saturation = progress;
        let lightness = 0.4 + 0.4 * (0.4 + app.time + progress * PI).cos();
        d.ellipse()
            .xy(s.p)
            .w_h(4.0, 4.0)
            .radius(1.0)
            .color(hsla(hue, saturation, lightness, 1.0));
    }
    d.to_frame(app, &f).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}