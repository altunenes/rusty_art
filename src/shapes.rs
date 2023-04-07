use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    t: f32,
    egui: Egui,
    settings: Settings,
}

struct Settings {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    num_shapes: usize,
    shape_size: f32,
    range_x:f32,
    range_y:f32,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    Model { t: 0.0,
    egui,
    settings: Settings {
        a: -1.4,
        b: 1.6,
        c: 1.0,
        d: 0.7,
        num_shapes: 400,
        shape_size: 1.0,
        range_x:10.0,
        range_y:10.0,
    },
}
}
fn update(app: &App, model: &mut Model, _update: Update) {
    model.t = app.time;
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("a:");
        ui.add(egui::Slider::new(&mut model.settings.a, -10.0..=10.0));
        ui.label("b:");
        ui.add(egui::Slider::new(&mut model.settings.b, -10.0..=10.0));
        ui.label("c:");
        ui.add(egui::Slider::new(&mut model.settings.c, -10.0..=10.0));
        ui.label("d:");
        ui.add(egui::Slider::new(&mut model.settings.d, -10.0..=10.0));
        ui.label("num_shapes:");
        ui.add(egui::Slider::new(&mut model.settings.num_shapes, 0..=1000));
        ui.label("shape_size:");
        ui.add(egui::Slider::new(&mut model.settings.shape_size, 0.0..=100.0));
        ui.label("range_x:");
        ui.add(egui::Slider::new(&mut model.settings.range_x, 0.0..=100.0));
        ui.label("range_y:");
        ui.add(egui::Slider::new(&mut model.settings.range_y, 0.0..=100.0));
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let a = model.settings.a;
    let b = model.settings.b;
    let c = model.settings.c;
    let d = model.settings.d;
    let num_shapes = model.settings.num_shapes;
    let shape_size = 1.0;
    let radius = model.settings.shape_size;

    for n in 0..num_shapes {
        let prev_x = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -model.settings.range_x, model.settings.range_x);
        let prev_y = map_range(n as f32 - 1.0, 0.0, num_shapes as f32, -model.settings.range_y, model.settings.range_y);
        let t = model.t;
        let x = (a * prev_y).sin() + c *(a*prev_x.cos()+t) * (b * prev_x + t).sin();
        let y = (b * prev_x).sin() + d *(b*prev_y.cos()+t) * (a * prev_y + t).sin();
        //another pattern based on the http://paulbourke.net/fractals/clifford/
        //let x = (a * prev_y).sin() + c *(a*prev_x+t).cos();
        //let y = (b * prev_x).sin() + d *(b*prev_y+t).cos();
        // map the x and y values to a smaller range
        let x = map_range(x, -10.0, 10.0, -400.0, 200.0);
        let y = map_range(y, -10.0, 10.0, -400.0, 200.0);
        let color = Hsl::new(map_range(n as f32, 0.0, num_shapes as f32, 0.0, 360.0), 1.0, 0.5);
        draw.line()
            .start(pt2(prev_x, prev_y))
            .end(pt2(x, y))
            .color(color)
            .weight(0.5);

        draw.ellipse()
            .x_y(x, y)
            .w_h(shape_size, shape_size)
            .color(color)
            .radius(radius);
    }

    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();    
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}
fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}