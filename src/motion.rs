//this one is unfinished:
//Brenner, E., & Smeets, J. Spatial Vision for Action. Oxford Research Encyclopedia of Psychology. Retrieved 18 Oct. 2022, from https://oxfordre.com/psychology/view/10.1093/acrefore/9780190236557.001.0001/acrefore-9780190236557-e-842.
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};


struct Model {
    angle1: f32,
    angle2: f32,
    egui: Egui,
    settings: Settings,
}

struct Settings {
    a: f32,
    b: f32,
}

fn main() {
    nannou::app(model).update(update).run();
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

    let angle1 = 0.0;
    let angle2 = 0.0;
    Model { angle1, angle2, egui, settings: Settings { a: 0.001, b: 0.001 }
}
}



fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _win = app.window_rect();

    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.settings.a, 0.0..=10.01).text("a"));
        ui.add(egui::Slider::new(&mut model.settings.b, 0.0..=10.01).text("b"));
    });




    model.angle1 += model.settings.a;
    model.angle2 -= model.settings.b;

}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let win = app.window_rect();
    let radius = 300.0; // The distance from the center of the window
    let x1 = radius * model.angle1.cos(); // The x coordinate of the first circle
    let y1 = radius * model.angle1.sin(); // The y coordinate of the first circle
    let x2 = radius * model.angle2.cos(); // The x coordinate of the second circle
    let y2 = radius * model.angle2.sin(); // The y coordinate of the second circle
    let r1_min_radius = 5.0; // The minimum radius value for the first circle
    let r1_max_radius = 65.0; // The maximum radius value for the first circle
    let r2_min_radius = 65.0; // The minimum radius value for the second circle
    let r2_max_radius = 5.0; // The maximum radius value for the second circle
    let r1 = map_range(model.angle1.sin(), 1.0, -1.0, r1_min_radius, r1_max_radius); // The radius of the first circle
    let r2 = map_range(model.angle2.sin(), -1.0, 1.0, r2_min_radius, r2_max_radius); // The radius of the second circle
    for _ in 0..5 {
        let x = random_range(-win.w() / 2.0, win.w() / 2.0);
        let y = random_range(-win.h() / 2.0, win.h() / 2.0);
        let r = random_range(1.0, 5.0);
        draw.ellipse().color(WHITE).radius(r).x_y(x, y);
    }
    draw.ellipse().color(RED).radius(r1).x_y(x1, y1);
    draw.ellipse().color(BLUE).radius(r2).x_y(x2, y2);
    draw.line()
        .color(WHITE)
        .start(pt2(x1, y1))
        .end(pt2(x2, y2));
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
          .project_path()
          .expect("failed to locate project directory")
          .join("frames") 
          .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path); 

    } 
}



fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}