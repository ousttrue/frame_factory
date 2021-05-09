mod window;

use winapi::ctypes::c_void;

extern crate renderer;
use renderer::{
    render_target::RenderTarget, renderer::Renderer, shader::Shader, vertex_buffer::VertexBuffer,
};

extern crate cgmath;
use cgmath::{Matrix, One};


fn run() -> Result<(), Box<dyn std::error::Error>> {
    let hwnd = window::create_window("WindowClass", "D3D11 Demo")?;
    let renderer = Renderer::new(hwnd)?;
    let render_target =
        RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain)?;

    let source = std::fs::read_to_string("shaders/mvp.hlsl")?;

    let (vs, input_layout, vs_constant_buffer) =
        Shader::compile_vertex_shader(&renderer.d3d_device, &source, "vsMain\0").unwrap();
    let ps = Shader::compile_pixel_shader(&renderer.d3d_device, &source, "psMain\0").unwrap();
    let shader = Shader {
        vs,
        ps,
        input_layout,
        vs_constant_buffer,
    };

    let vertex_buffer = VertexBuffer::create_triangle(&renderer.d3d_device)?;

    let fovy = cgmath::Rad::from(cgmath::Deg(60.0));
    let projection: cgmath::Matrix4<f32> = cgmath::perspective(fovy, 1.0, 0.1, 2.0);
    let view: cgmath::Matrix4<f32> = cgmath::Matrix4::one();
    let model: cgmath::Matrix4<f32> = cgmath::Matrix4::one();

    for _ in window::main_loop(hwnd) {
        // update constant buffer
        shader
            .vs_constant_buffer
            .update(&renderer.d3d_context, 0, model.as_ptr() as *const c_void);

        // frame
        render_target.prepare(&renderer.d3d_context);

        // model
        shader.set(&renderer.d3d_context);
        vertex_buffer.draw(&renderer.d3d_context);

        // flush
        renderer.present();
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
