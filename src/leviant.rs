use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    _app.new_window().size(720, 720).view(view).build().unwrap();
    Model {}
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    let window_width = _app.window_rect().w();
    let window_height = _app.window_rect().h();
    draw.background().color(WHITE);

    let n_lines = 120;
    for i in 0..n_lines {
        let angle = i as f32 / n_lines as f32 * TAU;
        draw.line()
            .weight(1.0)
            .points(
                pt2(0.0, 0.0),
                pt2(angle.cos() * window_width/2.0, angle.sin() * window_height/2.0),
            )
            .color(BLACK);
    }
    let n_circles = 56;
    let circle_radius = 10.0;
    let n_sectors = 140;
    for i in 0..n_circles {
        let radius = circle_radius * (i + 1) as f32 * window_width.min(window_height) / 2.0;
        for j in 0..n_sectors {
            let start_angle = j as f32 / n_sectors as f32 * TAU;
            let end_angle = (j + 1) as f32 / n_sectors as f32 * TAU;
            let color = if j % 2 == 0 { BLACK } else { WHITE };
            let points = vec![
                pt2(0.0, 0.0),
                pt2(start_angle.cos() * radius, start_angle.sin() * radius),
                pt2(end_angle.cos() * radius, end_angle.sin() * radius),
            ];
            draw.polygon().color(color).points(points);
        }
    }

    let ring_inner_radius = [0.10, 0.14, 0.40, 0.45, 0.7, 0.75];
    let ring_outer_radius = [0.14, 0.18, 0.45, 0.50, 0.75, 0.80];
    let ring_colors = [rgb(230.0/255.0,13.0/255.0,294.0/255.0), rgb(158.0/255.0,33.0/255.0,137.0/255.0),
                       rgb(230.0/255.0,13.0/255.0,294.0/255.0), rgb(158.0/255.0,33.0/255.0,137.0/255.0),
                       rgb(230.0/255.0,13.0/255.0,294.0/255.0), rgb(158.0/255.0,33.0/255.0,137.0/255.0)];
    let ring_resolution = 100;

    let ring_radius_scale = window_width.min(window_height) / 2.0;

    for r in 0..6 {
        let mut ring_points = Vec::new();

        for i in 0..=ring_resolution {
            let angle = i as f32 / ring_resolution as f32 * TAU;
            ring_points.push(pt2(angle.cos() * ring_outer_radius[r] * ring_radius_scale, angle.sin() * ring_outer_radius[r] * ring_radius_scale));
        }
        for i in (0..=ring_resolution).rev() {
            let angle = i as f32 / ring_resolution as f32 * TAU;
            ring_points.push(pt2(angle.cos() * ring_inner_radius[r] * ring_radius_scale, angle.sin() * ring_inner_radius[r] * ring_radius_scale));
        }

        draw.polygon().color(ring_colors[r]).points(ring_points);
    }

    draw.to_frame(_app, &frame).unwrap();
}