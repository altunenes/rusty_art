use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};



fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    counter: f32,
    stripes_visible: bool,
    egui: Egui,
    settings: Settings,
    scale: f32,
}

struct Settings {
    STRIP_WIDTH: f32,
    BLOCK_WIDTH: f32,
    BLOCK_HEIGHT: f32,
    Y_YELLOW: f32,
    Y_BLUE: f32,
    SPEED: f32,
    show_ui: bool,
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
     Model { counter: 0.0, 
        stripes_visible: true,
        scale: 1.0,
        egui,
        settings: Settings {
            STRIP_WIDTH: 7.0,
            BLOCK_WIDTH: 44.0,
            BLOCK_HEIGHT: 30.0,
            Y_YELLOW: 40.0,
            Y_BLUE: 180.0,
            SPEED: 0.04,
            show_ui:true,
        },
}
}
fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;
    let _settings = &model.settings;
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui.set_elapsed_time(_update.since_start);
    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("STRIP_WIDTH:");
        ui.add(egui::Slider::new(
            &mut model.settings.STRIP_WIDTH,
            1.0..=100.0,
        ));
        ui.label("BLOCK_WIDTH:");
        ui.add(egui::Slider::new(
            &mut model.settings.BLOCK_WIDTH,
            1.0..=100.0,
        ));
        ui.label("BLOCK_HEIGHT:");
        ui.add(egui::Slider::new(
            &mut model.settings.BLOCK_HEIGHT,
            1.0..=100.0,
        ));
        ui.label("Y_YELLOW:");
        ui.add(egui::Slider::new(
            &mut model.settings.Y_YELLOW,
            1.0..=100.0,
        ));
        ui.label("Y_BLUE:");
        ui.add(egui::Slider::new(
            &mut model.settings.Y_BLUE,
            1.0..=100.0,
        ));
        ui.label("SPEED:");
        ui.add(egui::Slider::new(
            &mut model.settings.SPEED,
            0.0001..=0.8,
        ));
    });

    let SPEED = model.settings.SPEED;

    model.counter += SPEED;
    if model.counter >= app.window_rect().w()/2.0 {
        model.counter = 0.0;
    }
    if app.mouse.buttons.left().is_down() {
        model.stripes_visible = false;
    }
    if app.mouse.buttons.left().is_up() {
        model.stripes_visible = true;
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw().scale(model.scale);
    let _window = app.window_rect();
    let STRIP_WIDTH = model.settings.STRIP_WIDTH;
    let BLOCK_WIDTH = model.settings.BLOCK_WIDTH;
    let BLOCK_HEIGHT = model.settings.BLOCK_HEIGHT;
    let Y_YELLOW = model.settings.Y_YELLOW;
    let Y_BLUE = model.settings.Y_BLUE;
    draw.background().color(WHITE);
    draw.text("click and hold to hide the stripes")
        .align_text_middle_y()
        .color(RED)
        .font_size(20);
    if model.stripes_visible {
        let window = app.window_rect();
        for i in (window.left() as i32..window.right() as i32).step_by((2.0 * STRIP_WIDTH) as usize) {
            let x = i as f32;
            draw.rect()
                .x_y(x + STRIP_WIDTH / 2.0, window.y())
                .w_h(STRIP_WIDTH, window.h())
                .color(BLACK);
        }
    }
    let yellow_x = model.counter;
    let blue_x = model.counter;
    draw.rect()
        .x_y(yellow_x + BLOCK_WIDTH, Y_YELLOW + BLOCK_HEIGHT)
        .w_h(BLOCK_WIDTH, BLOCK_HEIGHT)
        .color(YELLOW);

    draw.rect()
        .x_y(blue_x + BLOCK_WIDTH, Y_BLUE + BLOCK_HEIGHT)
        .w_h(BLOCK_WIDTH, BLOCK_HEIGHT)
        .color(BLUE);

        draw.to_frame(app, &frame).unwrap();
        if model.settings.show_ui {
            model.egui.draw_to_frame(&frame).unwrap();
        }
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
        if let nannou::winit::event::WindowEvent::MouseWheel { delta, .. } = event {
            let cursor_over_egui = model.egui.ctx().wants_pointer_input();
            if !cursor_over_egui {
                match delta {
                    nannou::winit::event::MouseScrollDelta::LineDelta(_, y) => {
                        model.scale *= 1.0 + *y * 0.05;
                        model.scale = model.scale.max(0.1).min(10.0);
                    }
                    _ => (),
                }
            }
        }
        if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
            if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
                (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
            {
                let window = _app.main_window();
                let fullscreen = window.fullscreen().is_some();
                window.set_fullscreen(!fullscreen);
            }
        }
    }