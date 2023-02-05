use nannou::prelude::*;
use rand::{thread_rng, Rng};

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let window = app.window_rect();
    let duration = app.duration.since_start.secs() as f32;
    let window_diagonal = window.top_left().distance(window.bottom_right());
    let mut rng = thread_rng();
    draw.background().color(BLACK);
    let number_of_ellipses = ((0.4 * duration * 1.0 * PI).sin() * 14.0 + 400.0) as usize;
    for i in 0..number_of_ellipses {
        let position = i as f32 / number_of_ellipses as f32;
        let max_line_weight = (1.0 / number_of_ellipses as f32) * window.w();
        let x_position = window.x.lerp(position);
        let frequency = 0.1;
        let moving_x = (duration * frequency * 2.0 * PI).sin() * window.right();
        let distance = (moving_x - x_position).abs();
        let normalized_distance = distance / window.w();
        let line_weight = max_line_weight * normalized_distance * normalized_distance;
        let hue = rng.gen_range(0.0, 1.0);
        let angle = (duration * 0.9 + position) * 1.0 * PI;
        let magnitude = window_diagonal;
        let first_point = pt2(angle.cos() * magnitude, angle.sin() * magnitude);
        let second_point = pt2(angle.cos() * -magnitude, angle.sin() * -magnitude);
        let color = hsla(
            hue,
            (position * 0.0).min(0.0),
            
            0.5,
            normalized_distance * (41.0 - (angle / (25.0 * PI)).cos()),
        );
        draw.line()
            .weight(line_weight * 32.0)
            .points(first_point, second_point)
            .color(color);
    }
    
    draw.to_frame(app, &frame).unwrap();
}