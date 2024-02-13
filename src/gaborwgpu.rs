use nannou::prelude::*;
//some of comments from the nannou example: https://github.com/nannou-org/nannou/blob/master/examples/wgpu/wgpu_triangle/wgpu_triangle.rs
struct Model {
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

// The vertex type that we will use to represent a point on our triangle.
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
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let w_id = app.new_window().size(512, 512).view(view).build().unwrap();

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

    // Create the render pipeline.
    let bind_group_layout = wgpu::BindGroupLayoutBuilder::new().build(device);
    let bind_group = wgpu::BindGroupBuilder::new().build(device, &bind_group_layout);
    let pipeline_layout = wgpu::create_pipeline_layout(device, None, &[&bind_group_layout], &[]);
    let render_pipeline = wgpu::RenderPipelineBuilder::from_layout(&pipeline_layout, &vs_mod)
        .fragment_shader(&fs_mod)
        .color_format(format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float32x2])
        .sample_count(sample_count)
        .build(device);

    Model {
        bind_group,
        vertex_buffer,
        render_pipeline,
    }
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Using this we will encode commands that will be submitted to the GPU.
    let mut encoder = frame.command_encoder();

    // The render pass can be thought of a single large command consisting of sub commands. Here we
    // begin a render pass that outputs to the frame's texture. Then we add sub-commands for
    // setting the bind group, render pipeline, vertex buffers and then finally drawing.
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_bind_group(0, &model.bind_group, &[]);
    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_vertex_buffer(0, model.vertex_buffer.slice(..));
    let vertex_range = 0..VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
    if app.keys.down.contains(&Key::Space) {
        let file_path = app
            .project_path()
            .expect("failed to locate project directory")
            .join("frames")
            .join(format!("{:0}.png", app.elapsed_frames()));
        app.main_window().capture_frame(file_path);
    } 
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}