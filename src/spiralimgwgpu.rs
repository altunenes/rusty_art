use nannou::image;
use nannou::image::{open,RgbaImage,DynamicImage};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};
use rfd::FileDialog;
use std::option::Option;
use nannou::wgpu::Texture;
struct Model {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    time_uniform: wgpu::Buffer,
    time_bind_group: wgpu::BindGroup,
    params_uniform: wgpu::Buffer,
    params_bind_group: wgpu::BindGroup,
    settings:Settings,
    egui:Egui,
    img: Option<RgbaImage>,
    texture: Option<Texture>,
    sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
}

struct Settings {
    lambda: f32,
    theta: f32,
    alpha:f32,
    sigma: f32,
    gamma:f32,
    blue:f32,
    use_texture_colors: bool,
    show_ui: bool,
    open_file_dialog: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
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
    let mut open_file_dialog: bool = model.settings.open_file_dialog;
    egui::Window::new("Shader Settings").show(&ctx, |ui| {
        if ui.button("Load Image").clicked() {
            open_file_dialog = true;
        }
        ui.add(egui::Slider::new(&mut model.settings.lambda, 1.0..=360.0).text("l"));
        ui.add(egui::Slider::new(&mut model.settings.theta, -6.2..=6.2).text("t"));
        ui.add(egui::Slider::new(&mut model.settings.alpha, 0.0..=1.0).text("a"));
        ui.add(egui::Slider::new(&mut model.settings.sigma, 0.0..=1.0).text("r"));
        ui.add(egui::Slider::new(&mut model.settings.gamma, 0.0..=1.0).text("g"));
        ui.add(egui::Slider::new(&mut model.settings.blue, 0.0..=1.0).text("b"));
        ui.checkbox(&mut model.settings.use_texture_colors, "Use Texture Colors"); 

    });
    if open_file_dialog {
        if let Some(file_path) = FileDialog::new().pick_file() {
            if let Ok(img) = open(&file_path).map(|i| i.to_rgba8()) {
                let dyn_image = DynamicImage::ImageRgba8(img.clone());
                model.img = Some(img);
                let main_window = app.main_window();
                let device = main_window.device();  // Accessing device directly

                let new_texture = Texture::from_image(app, &dyn_image);
                model.texture = Some(new_texture);

                let new_texture_view = model.texture.as_ref().unwrap().view().build();
                model.bind_group = create_bind_group(device, &model.bind_group_layout, &new_texture_view, &model.sampler);
            }
            model.settings.open_file_dialog = false;
        }
    }
    let params_data = [model.settings.lambda, model.settings.theta, model.settings.alpha, model.settings.sigma, model.settings.gamma, model.settings.blue,if model.settings.use_texture_colors { 1.0 } else { 0.0 }];
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

    let w_id = app.new_window().raw_event(raw_window_event).size(800, 600).view(view).build().unwrap();
    let window = app.window(w_id).unwrap();
    let device = window.device();
    let format = Frame::TEXTURE_FORMAT;
    let msaa_samples = window.msaa_samples();
    let vs_desc = wgpu::include_wgsl!("../shaders/verteximg.wgsl");
    let fs_desc = wgpu::include_wgsl!("../shaders/spiralimgwgpu.wgsl");
    let vs_mod = device.create_shader_module(vs_desc);
    let fs_mod = device.create_shader_module(fs_desc);
    let sampler_desc = wgpu::SamplerBuilder::new().into_descriptor();
    let sampler_filtering = wgpu::sampler_filtering(&sampler_desc);
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Texture Sampler"),
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    let settings = Settings {
        lambda:35.0,
        theta:0.7,
        alpha:0.7,
        sigma:0.1,
        gamma:0.1,
        blue:0.1,
        use_texture_colors: false,  
        show_ui:true,
        open_file_dialog:false,
    };
    let params_data = [
        settings.lambda, 
        settings.theta, 
        settings.alpha,
        settings.sigma,
        settings.gamma,
        settings.blue,
        if settings.use_texture_colors { 1.0 } else { 0.0 },
    ];    
    let params_bytes = bytemuck::cast_slice(&params_data);
    let params_uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Params Uniform"),
        contents: params_bytes,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let params_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("params_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: wgpu::BufferSize::new((std::mem::size_of::<f32>() * 7) as _),
            },
            count: None,
        }],
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
    let mut dummy_img = RgbaImage::new(800, 600);
    dummy_img.put_pixel(0, 0, image::Rgba([255, 255, 255, 255])); 
    let texture = Texture::from_image(app, &image::DynamicImage::ImageRgba8(dummy_img));
    let texture_view = texture.view().build();

    let texture_bind_group_layout = create_bind_group_layout(device, wgpu::TextureSampleType::Float { filterable: true }, true);
    let bind_group_layout =
        create_bind_group_layout(device, texture_view.sample_type(), sampler_filtering);
    let bind_group = create_bind_group(device, &bind_group_layout, &texture_view, &sampler);
    let pipeline_layout = create_pipeline_layout(device, &[&texture_bind_group_layout, &time_bind_group_layout, &params_bind_group_layout]);
    let render_pipeline = create_render_pipeline(
        device,
        &pipeline_layout,
        &vs_mod,
        &fs_mod,
        format,
        msaa_samples,
    );
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsages::VERTEX;
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: vertices_bytes,
        usage,
    });
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
    let params_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &params_bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 2,
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
        bind_group,
        vertex_buffer,
        render_pipeline,
        time_uniform,
        time_bind_group,
        img: None,
        texture: Some(texture),
        sampler,
        bind_group_layout,
    }
}
fn view(app: &App, model: &Model, frame: Frame) {
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
    render_pass.set_bind_group(0, &model.bind_group, &[]);
    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_bind_group(1, &model.time_bind_group, &[]); 
    render_pass.set_bind_group(2, &model.params_bind_group, &[]);
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
fn create_bind_group_layout(
    device: &wgpu::Device,
    texture_sample_type: wgpu::TextureSampleType,
    sampler_filtering: bool,
) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .texture(
            wgpu::ShaderStages::FRAGMENT,
            false,
            wgpu::TextureViewDimension::D2,
            texture_sample_type,
        )
        .sampler(wgpu::ShaderStages::FRAGMENT, sampler_filtering)
        .build(device)
}
fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .texture_view(texture)
        .sampler(sampler)
        .build(device, layout)
}
fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[],
    };
    device.create_pipeline_layout(&desc)
}
fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, vs_mod)
        .fragment_shader(fs_mod)
        .color_format(dst_format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .build(device)
}
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}