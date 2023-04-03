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
    // change the parameters based on some function of time. These values are from the http://paulbourke.net/fractals/clifford/ website
    let a = -1.4;
    let b = -10.4 ;
    let c = -1.0 ;
    let d = 1.2 ;
    let num_shapes = 500;
    let shape_size = 10.0;


    for n in 0..num_shapes {
        let prev_x = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -1.0, 1.0);
        let prev_y = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -1.0, 1.0);
        let t = model.t;
        let x = (a * prev_y).sin() + c *(a*prev_x.cos()+t) * (b * prev_x + t).sin();
        let y = (b * prev_x).sin() + d *(b*prev_y.cos()+t) * (a * prev_y + t).sin();

        //another pattern based on the http://paulbourke.net/fractals/clifford/
        //let x = (a * prev_y).sin() + c *(a*prev_x+t).cos();
        //let y = (b * prev_x).sin() + d *(b*prev_y+t).cos();

        // map the x and y values to a smaller range
        let x = map_range(x, -10.0, 10.0, -400.0, 200.0);
        let y = map_range(y, -10.0, 10.0, -400.0, 200.0);

        let color = Hsl::new(map_range(n as f32, 0.0, num_shapes as f32, 0.0, 360.0), 3.0, 0.5);
        draw.ellipse().x_y(x, y).w_h(shape_size, shape_size).color(WHITE);
        draw.line().start(pt2(prev_x, prev_y)).end(pt2(x, y)).weight(1.0).color(color);        

    }


    draw.to_frame(app, &frame).unwrap();
}
