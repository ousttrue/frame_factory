use std::error;

mod window;

extern crate renderer;
use renderer::{render_target::RenderTarget, renderer::Renderer};

fn run() -> Result<(), Box<dyn error::Error>> {
    let hwnd = window::create_window("WindowClass", "D3D11 Demo")?;
    let renderer = Renderer::new(hwnd)?;
    let render_target =
        RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain)?;

    for _ in window::main_loop(hwnd) {
        render_target.prepare(&renderer.d3d_context);
        renderer.present();
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
