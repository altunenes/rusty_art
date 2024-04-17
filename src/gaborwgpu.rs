use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
//some of comments from the nannou example: https://github.com/nannou-org/nannou/blob/master/examples/wgpu/wgpu_triangle/wgpu_triangle.rs
//and I don't want to remove them since I m also learning WGPU and they are really useful to follow a path.... :-)
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
    sigma: f32,
    gamma:f32,
    show_ui: bool,
}
// The vertex type that we will use to represent a point on our triangle. (Not in case on our GABOR code ofc)
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
// The vertices that make up our triangle.
// The vertices that make up our fullscreen quad.
// Two triangles covering the entire NDC space [-1, 1] for x and y.
//since its creative coding right? :-)
const VERTICES: [Vertex; 6] = [
    Vertex { position: [-1.0, -1.0] }, // Bottom-left
    Vertex { position: [ 1.0, -1.0] }, // Bottom-right
    Vertex { position: [-1.0,  1.0] }, // Top-left
    Vertex { position: [ 1.0, -1.0] }, // Bottom-right
    Vertex { position: [ 1.0,  1.0] }, // Top-right
    Vertex { position: [-1.0,  1.0] }, // Top-left
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
        ui.add(egui::Slider::new(&mut model.settings.lambda, 0.01..=1.0).text("lambda"));
        ui.add(egui::Slider::new(&mut model.settings.theta, -PI..=PI).text("Theta"));
        ui.add(egui::Slider::new(&mut model.settings.sigma, 0.01..=1.0).text("Sigma"));
        ui.add(egui::Slider::new(&mut model.settings.gamma, 0.01..=1.0).text("gamma"));
    });
    let params_data = [model.settings.lambda, model.settings.theta, model.settings.sigma,model.settings.gamma];
    let params_bytes = bytemuck::cast_slice(&params_data);
    app.main_window().queue().write_buffer(&model.params_uniform, 0, &params_bytes);
}
fn raw_window_event(app: &App, _model: &mut Model, event: &nannou::winit::event::WindowEvent) {
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
    // Load shader modules.
    let vs_desc = wgpu::include_wgsl!("../shaders/vs.wgsl");
    let fs_desc = wgpu::include_wgsl!("../shaders/gabor.wgsl");
    let vs_mod = device.create_shader_module(vs_desc);
    let fs_mod = device.create_shader_module(fs_desc);
    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsages::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });
    let time_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        entries: &[
            // This defines the layout for our time uniform buffer
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
                min_binding_size: wgpu::BufferSize::new((std::mem::size_of::<f32>() * 4) as _),
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
        lambda: 0.2,
        theta:0.0,
        sigma:0.1,
        gamma:1.0,
        show_ui:true,
    };
    let params_data = [settings.lambda, settings.theta, settings.sigma,settings.gamma];
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
    // Using this we will encode commands that will be submitted to the GPU.
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
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    }
}
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}