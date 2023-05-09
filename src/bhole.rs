use nannou::prelude::*;
use std::f32::consts::PI;

const N: usize = 5; 
const U: usize = 1000;
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
            let a = (i as f32 / U as f32) * 40.0 * PI; 
            let r = 280.0;  // radius -> r
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
    let t = app.time / 10.0; // t -> t
    for (i, bh) in m.b.iter_mut().enumerate() {  
        let a = (t * i as f32) * 8.0 * PI; 
        *bh = pt2(a.cos() * i as f32 * 20.0, a.sin() * i as f32 * 20.0); 
    }
    for (i, s) in m.s.iter_mut().enumerate() {  
        s.update(&m.b, i); 
    }
}
fn v(app: &App, m: &M, f: Frame) { 
    let d = app.draw();  
    d.background().color(BLACK); 
    for s in &m.s {
        d.ellipse()
            .xy(s.p)
            .radius(1.0)
            .color(hsla(s.c, 1.0, 0.5, 1.0)); 
    }
    d.to_frame(app, &f).unwrap();
}