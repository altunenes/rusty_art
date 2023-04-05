use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    t: f32,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    Model { t: 0.0}
}

fn update(app: &App, model: &mut Model, _update: Update) {
    // update the time variable based on app time
    model.t = app.time;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // change the parameters based on some function of t
    let a = -4.1;
    let b = 1.6 ;
    let c = 1.2 ;
    let d = -0.7 ;
    let num_shapes = 1000;
    let shape_size = 14.0;

    for n in 0..num_shapes {
        let prev_x = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -10.0, 10.0).cos();
        let prev_y = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -10.0, 10.0).sin();
        let t = model.t;

        let x = (a * prev_y).sin() + c *(a*prev_x.cos()+t) * (b * prev_x + t).sin();
        let y = (b * prev_x).sin() + d *(b*prev_y.cos()+t) * (a * prev_y + t).sin();

        //another pattern based on the http://paulbourke.net/fractals/clifford/
        //let x = (a * prev_y).sin() + c *(a*prev_x+t).cos();
        //let y = (b * prev_x).sin() + d *(b*prev_y+t).cos();

        // map the x and y values to a smaller range
        let x = map_range(x, -10.0, 10.0, -400.0, 200.0);
        let y = map_range(y, -10.0, 10.0, -400.0, 200.0);

        let hue = map_range(n as f32, 1.0, 100.0 as f32, 4.0, 5.0);
        let color = hsla(hue, 1.0, 0.5, 1.0);

        draw.polyline()
            .weight(shape_size)
            .color(color)
            .points(vec![pt2(prev_x, prev_y), pt2(x, y)]);
    }

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
