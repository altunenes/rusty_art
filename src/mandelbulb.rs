use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
struct Model {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    time_uniform: wgpu::Buffer,
    time_bind_group: wgpu::BindGroup,
    params_uniform: wgpu::Buffer,
    params_bind_group: wgpu::BindGroup,
    settings:Settings,
    egui:Egui,
}
struct Settings {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma:f32,
    blue:f32,
    a:f32,
    b:f32,
    c:f32,
    d:f32,
    e:f32,
    f:f32,
    g:f32,
    iter:f32,
    bound:f32,
    aa:f32,
    tt:f32,
    show_ui: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
const VERTICES: [Vertex; 6] = [
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [ 1.0, -1.0] },
    Vertex { position: [-1.0,  1.0] },
    Vertex { position: [ 1.0, -1.0] },
    Vertex { position: [ 1.0,  1.0] },
    Vertex { position: [-1.0,  1.0] },
];
fn main() {
    nannou::app(model)
        .update(update) 
        .run();
}
fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    if app.keys.down.contains(&Key::H) {
        model.settings.show_ui = !model.settings.show_ui;
    }
    egui::Window::new("Shader Settings").show(&ctx, |ui| {
        ui.add(egui::Slider::new(&mut model.settings.lambda, 0.0..=10.0).text("color1"));
        ui.add(egui::Slider::new(&mut model.settings.theta, 0.0..=10.0).text("color2"));
        ui.add(egui::Slider::new(&mut model.settings.alpha, 0.0..=10.0).text("color3"));
        ui.add(egui::Slider::new(&mut model.settings.sigma, 0.0..=10.0).text("color4"));
        ui.add(egui::Slider::new(&mut model.settings.gamma, 0.0..=10.0).text("color5"));
        ui.add(egui::Slider::new(&mut model.settings.blue, 0.0..=10.0).text("color6"));
        ui.add(egui::Slider::new(&mut model.settings.a, 0.0..=10.0).text("color7"));
       ui.add(egui::Slider::new(&mut model.settings.b, 0.0..=10.0).text("color8"));
       ui.add(egui::Slider::new(&mut model.settings.c, 0.0..=10.0).text("color9"));
       ui.add(egui::Slider::new(&mut model.settings.d, 0.0..=5.0).text("bcolor1"));
        ui.add(egui::Slider::new(&mut model.settings.g, 0.0..=5.00).text("bcolor2"));
        ui.add(egui::Slider::new(&mut model.settings.e, 0.002..=5.0).text("bcolor3"));
        ui.add(egui::Slider::new(&mut model.settings.f, 0.1..=100.0).text("color"));
        ui.add(egui::Slider::new(&mut model.settings.iter, 1.0..=24.0).text("POWER"));
        ui.add(egui::Slider::new(&mut model.settings.bound, 0.0..=45.0).text("Branch"));
        ui.add(egui::Slider::new(&mut model.settings.aa, 1.0..=4.0).text("AA"));
        ui.add(egui::Slider::new(&mut model.settings.tt, 0.00001..=0.5).text("epsilon"));
    });
    let params_data = [model.settings.lambda, model.settings.theta,model.settings.alpha, model.settings.sigma,model.settings.gamma,model.settings.blue,model.settings.aa,model.settings.iter,model.settings.bound,model.settings.tt,model.settings.a,model.settings.b,model.settings.c,model.settings.d,model.settings.e,model.settings.f,model.settings.g];
    let params_bytes = bytemuck::cast_slice(&params_data);
    app.main_window().queue().write_buffer(&model.params_uniform, 0, &params_bytes);
}
fn raw_window_event(app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
    if let nannou::winit::event::WindowEvent::KeyboardInput { input, .. } = event {
        if let (Some(nannou::winit::event::VirtualKeyCode::F), true) =
            (input.virtual_keycode, input.state == nannou::winit::event::ElementState::Pressed)
        {
            let window = app.main_window();
            let fullscreen = window.fullscreen().is_some();
            window.set_fullscreen(!fullscreen);
        }
    }
}
fn model(app: &App) -> Model {
    let w_id = app.new_window().raw_event(raw_window_event).
    size(512, 512).view(view).build().unwrap();
    // The gpu device associated with the window's swapchain
    let window = app.window(w_id).unwrap();
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let sample_count = window.msaa_samples();
    let vs_desc = wgpu::include_wgsl!("../shaders/vs.wgsl");
    let fs_desc = wgpu::include_wgsl!("../shaders/mandelbulb.wgsl");
    let vs_mod = device.create_shader_module(vs_desc);
    let fs_mod = device.create_shader_module(fs_desc);
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsages::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });
    let time_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of::<f32>() as _),
                },
                count: None,
            },
        ],
        label: Some("time_bind_group_layout"),
    });
    let params_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("params_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new((std::mem::size_of::<f32>() * 17) as _),
            },
            count: None,
        }],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&params_bind_group_layout, &time_bind_group_layout],
        push_constant_ranges: &[],
    });
    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .build(device);
    let time_uniform = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Time Uniform Buffer"),
        size: std::mem::size_of::<f32>() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let time_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &time_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: time_uniform.as_entire_binding(),
            },
        ],
        label: Some("time_bind_group"),
    });
    let settings = Settings {
        lambda: 0.8,
        theta: 1.0,
        alpha: 0.7,
        sigma: 0.3,
        gamma: 0.7,
        blue: 0.3,
        show_ui: true,
        aa: 2.0,
        iter: 8.0,
        bound: 5.05,
        tt: 0.0001,
        a: 0.1,
        b: 0.4,
        c: 0.1,
        d: 0.4,
        e: 0.2,
        f: 0.0,
        g: 0.1,
    };
    let params_data = [settings.lambda, settings.theta, settings.alpha,settings.sigma,settings.gamma,settings.blue,settings.aa,settings.iter,settings.bound,settings.tt,settings.a,settings.b,settings.c,settings.d,settings.e,settings.f,settings.g];
    let params_bytes = bytemuck::cast_slice(&params_data);
    let params_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Params Uniform"),
        contents: params_bytes,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let params_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &params_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 1,
                resource: params_uniform.as_entire_binding(),
            },
        ],
        label: Some("params_bind_group"),
    });
    let window = app.window(w_id).unwrap();
    let egui = Egui::from_window(&window);
    Model {
        params_bind_group,
        settings,
        params_uniform,
        egui,
        vertex_buffer,
        render_pipeline,
        time_uniform,
        time_bind_group,
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let time = app.time; 
    let time_bytes = time.to_ne_bytes();
    let binding = app.main_window();
    let queue = binding.queue();
    {
        let mut encoder = frame.command_encoder();
        queue.write_buffer(&model.time_uniform, 0, &time_bytes);
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(frame.texture_view(), |color| color)
            .begin(&mut encoder);
        render_pass.set_bind_group(0, &model.params_bind_group, &[]);
        render_pass.set_bind_group(1, &model.time_bind_group, &[]);
        render_pass.set_pipeline(&model.render_pipeline);
        render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        let vertex_range = 0..VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }
    if model.settings.show_ui {
        model.egui.draw_to_frame(&frame).unwrap();
    }
    if app.keys.down.contains(&Key::Space) {
        match app.project_path() {
            Ok(project_path) => {
                let frames_path = project_path.join("frames");
                if let Err(e) = std::fs::create_dir_all(&frames_path) {
                    eprintln!("Failed to create frames directory: {:?}", e);
                    return;
                }
                let file_path = frames_path.join(format!("{:0}.png", app.elapsed_frames()));
                app.main_window().capture_frame(file_path);
            },
            Err(e) => {
                eprintln!("Failed to locate project directory: {:?}", e);
            }
        }
    }
}
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}