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
    Model { t: 0.0 }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    model.t = app.time;
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let num_plumes = 100;
    let plume_length = 333.0;
    let plume_width = 125.0;
    let plume_speed = 1.8;
    for i in 0..num_plumes {
        let angle = map_range(i, 0, num_plumes, 0.0, 360.0);
        let plume_x = (angle.to_radians()).cos() * (app.time * plume_speed).cos() * plume_length;
        let plume_y = (angle.to_radians()).sin() * (app.time * plume_speed).sin() * plume_length;
        let plume_points = vec![
            pt2(plume_x - plume_width, plume_y + plume_width),
            pt2(plume_x + plume_width, plume_y + plume_width),
            pt2(plume_x + plume_width, plume_y - plume_width),
            pt2(plume_x - plume_width, plume_y - plume_width),
        ];
        let plume_color = Hsl::new(map_range(i, 0, num_plumes, 0.0, 360.0), 1.0, 0.5);
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(plume_x, plume_y))
            .weight(3.0)
            .color(plume_color);
        draw.polyline()
            .weight(5.0)
            .points(plume_points)
            .color(plume_color);
    }
    draw.to_frame(app, &frame).unwrap();
}
