mod window;

extern crate renderer;
use renderer::{render_target::RenderTarget, renderer::Renderer, shader::Shader};

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let hwnd = window::create_window("WindowClass", "D3D11 Demo")?;
    let renderer = Renderer::new(hwnd)?;
    let render_target =
        RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain)?;

    let source = std::fs::read_to_string("simple.hlsl")?;

    let vs = Shader::compile_vertex_shader(&renderer.d3d_device, &source, "vsMain\0")?;
    let ps = Shader::compile_pixel_shader(&renderer.d3d_device, &source, "psMain\0")?;
    let shader = Shader::new(vs, ps);

    for _ in window::main_loop(hwnd) {
        render_target.prepare(&renderer.d3d_context);
        renderer.present();
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
