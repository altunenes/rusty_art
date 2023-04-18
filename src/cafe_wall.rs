use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
struct Model {
    time: f32,
    mouse_position: Vector2,
    scroll_offset: f32,
    is_mouse_pressed: bool,
    egui: Egui,
    settings: Settings,
}
struct Settings {
    t_size: f32,
    l_size: f32,
    l_width: f32,
}
fn main() {
    nannou::app(model)
        .update(update)
        .size(1024, 1024)
        .run();
}
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);    Model {
        time: 0.0,
        mouse_position: vec2(0.0, 0.0),
        scroll_offset: 0.0,
        is_mouse_pressed: false,
        egui, settings: Settings {
            t_size: 50.0,
            l_size: 76.0,
            l_width: 3.0,
        }
    }
}
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.time = _app.time;
    let egui = &mut model.egui;
    let _settings = &model.settings;
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Tile Size:");
        ui.add(egui::Slider::new(&mut model.settings.t_size, 1.0..=255.0));
        ui.label("Line Size:");
        ui.add(egui::Slider::new(&mut model.settings.l_size, 0.0..=255.0));
        ui.label("Line Width:");
        ui.add(egui::Slider::new(&mut model.settings.l_width, 0.0..=25.0));
    });
    model.mouse_position = _app.mouse.position();
    model.is_mouse_pressed = _app.mouse.buttons.left().is_down();

    if model.is_mouse_pressed {
        model.scroll_offset = model.mouse_position.x / _app.window_rect().w() * 10.0;
    } else {
        model.scroll_offset += 0.005;
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw: Draw = app.draw();
    draw.background().color(WHITE);
    let window_rect = app.window_rect();
    let tile_size = model.settings.t_size;
    let columns = (window_rect.w() / tile_size).ceil() as i32;
    let rows = (window_rect.h() / tile_size).ceil() as i32;
    for y in -rows..rows {
        for x in -columns..columns {
            let is_odd_row = y % 2 == 0;
            let direction = if is_odd_row { 1.0 } else { -1.0 };
            let tile_x = x as f32 * tile_size + direction * model.scroll_offset * tile_size;
            let color = if (x + y) % 2 == 0 { BLACK } else { WHITE };
            draw.rect()
                .x_y(tile_x, y as f32 * tile_size)
                .w_h(tile_size, tile_size)
                .color(color);
        }
    }
    let border_color = gray(0.5);
    let border_width = model.settings.l_width;
    for y in (-rows - 1)..rows {
        draw.line()
            .points(
                pt2(window_rect.left(), y as f32 * tile_size + model.settings.l_size),
                pt2(window_rect.right(), y as f32 * tile_size + model.settings.l_size),
            )
            .color(border_color)
            .weight(border_width);
    }
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
