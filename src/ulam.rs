use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    path: Vec<(Point2, bool)>, 
    counter: usize,
    dx: isize,
    dy: isize,
    n: isize,
    x: isize,
    y: isize,
    numbers: Vec<bool>,
}

fn model(app: &App) -> Model {
    let _window_id = app
        .new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let size = 10000;
    let mut numbers = vec![false; size];
    for i in 2..size {
        if is_prime(i) {
            numbers[i] = true;
        }
    }

    Model {
        path: Vec::new(),
        counter: 1,
        dx: 1,
        dy: 0,
        n: 1,
        x: 0,
        y: 0,
        numbers,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let i = model.counter;
    let x = model.x;
    let y = model.y;
    let pt = pt2(x as f32, y as f32);
    
    model.path.push((pt, model.numbers[i])); // Add the point along with its prime status

    if model.n * model.n + 1 == i as isize {
        model.dy = ((model.n % 2) * 2 - 1) * 7;
        model.dx = 0;
        model.n += 1;
    } else if model.n * model.n - model.n + 1 == i as isize {
        model.dx = ((model.n % 2) * 2 - 1) * 7;
        model.dy = 0;
    }
    
    model.x += model.dx;
    model.y += model.dy;
    
    model.counter += 1;
    if model.counter >= model.numbers.len() {
        model.counter = 1;
        model.n = 1;
        model.dx = 7;
        model.dy = 0;
        model.x = 0;
        model.y = 0;
        model.path.clear();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for &(point, is_prime) in &model.path {
        if is_prime {
            draw.ellipse()
                .xy(point)
                .radius(3.0)
                .color(WHITE);
        } else {
            draw.ellipse()
                .xy(point)
                .radius(1.5)
                .color(RED);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..((n as f64).sqrt() as usize + 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}