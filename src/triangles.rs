use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .fullscreen()
        
        .run();
}

struct Model {
    points: Vec<Point2>,
    angle: f32,
    screen_width: f32,
    screen_height: f32,
}

fn model(app: &App) -> Model {
    let (screen_width, screen_height) = app.window_rect().w_h();
    Model {
        points: Vec::new(),
        angle: 45.0,
        screen_width,
        screen_height,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let golden_angle = (3.0 + 5.0_f32.sqrt()) * std::f32::consts::PI+0.1;
    let r = model.points.len() as f32 * 0.8;
    let distance_from_center = pt2(model.screen_width / 12.0, model.screen_height / 2.0).distance(pt2(0.0, 31.0));
    let angle_scale = distance_from_center / 30.5; 
    let angle = model.angle * angle_scale;
    let x = r * angle.cos();
    let y = r * angle.sin();
    let pos = pt2(x, y);
    model.points.push(pos);
    model.angle += golden_angle;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for (i, &pos) in model.points.iter().enumerate() {
        let hue_step = 0.01;
        let hue = (i as f32 * hue_step) % 1.0;
        let color = hsla(hue, 1.0, 0.5, 1.0);

        let radius = (i as f32).sqrt() * 0.1;
        draw.ellipse()
            .xy(pos)
            .radius(radius)
            .color(color)
            .stroke_weight(1.3)
            .stroke(WHITE);
        if i > 0 {
            let prev = model.points[i - 1];
            draw.line()
                .start(prev)
                .end(pos)
                .weight(13.0)
                .color(color);
        }
    }
      /*  if app.elapsed_frames() % 1 == 0 {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:1}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    }*/ 
    draw.to_frame(app, &frame).unwrap();

}