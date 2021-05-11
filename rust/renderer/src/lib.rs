mod com_util;
mod render_target;
mod renderer;
mod shader;
mod vertex_buffer;

use cgmath::{Matrix, One};
use render_target::RenderTarget;
use renderer::Renderer;
use shader::Shader;
use std::{ffi::c_void, ptr};
use vertex_buffer::VertexBuffer;
use winapi::{shared::windef::HWND, um::d3d11sdklayers};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

static mut G_RENDERER: Option<Renderer> = None;

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_initialize(hwnd: HWND) -> *const c_void {
    if let Ok(renderer) = renderer::Renderer::new(hwnd) {
        unsafe {
            G_RENDERER = Some(renderer);
            if let Some(renderer) = &G_RENDERER {
                return renderer.d3d_device.as_ptr() as *const c_void;
            }
        }
    }

    unsafe {
        G_SCENE = None;
        if let Some(renderer) = &G_RENDERER {
            if let Ok(debug) = renderer
                .d3d_device
                .query_interface::<d3d11sdklayers::ID3D11Debug>()
            {
                debug.ReportLiveDeviceObjects(d3d11sdklayers::D3D11_RLDO_DETAIL);
            }
        }
        G_RENDERER = None;
    }
    ptr::null()
}

struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    render_target: RenderTarget,
    vertex_buffer: VertexBuffer,
}

impl Scene {
    fn render(&self, renderer: &Renderer) {
        // update constant buffer
        self.shader
            .vs_constant_buffer
            .update(&renderer.d3d_context, 0, self.model.as_ptr());

        // frame
        self.render_target.prepare(&renderer.d3d_context);

        // model
        self.shader.set(&renderer.d3d_context);
        self.vertex_buffer.draw(&renderer.d3d_context);
    }
}

static mut G_SCENE: Option<Scene> = None;

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_scene(
    source: *const u8,
    source_size: usize,
    vs_main: *const u8,
    ps_main: *const u8,
) -> bool {
    if let Some(renderer) = unsafe { &G_RENDERER } {
        let render_target =
            RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain).unwrap();

        if let Ok((vs, input_layout, vs_constant_buffer)) =
            Shader::compile_vertex_shader(&renderer.d3d_device, source, source_size, "vsMain\0")
        {
            if let Ok(ps) =
                Shader::compile_pixel_shader(&renderer.d3d_device, source, source_size, "psMain\0")
            {
                let shader = Shader {
                    vs,
                    ps,
                    input_layout,
                    vs_constant_buffer,
                };

                if let Ok(vertex_buffer) = VertexBuffer::create_triangle(&renderer.d3d_device) {
                    let fovy = cgmath::Rad::from(cgmath::Deg(60.0));
                    let projection: cgmath::Matrix4<f32> = cgmath::perspective(fovy, 1.0, 0.1, 2.0);
                    let view: cgmath::Matrix4<f32> = cgmath::Matrix4::one();
                    let model: cgmath::Matrix4<f32> = cgmath::Matrix4::one();

                    let scene = Scene {
                        shader,
                        model,
                        render_target,
                        vertex_buffer,
                    };

                    unsafe { G_SCENE = Some(scene) };

                    return true;
                }
            }
        }
    }

    false
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_render() {
    if let Some(renderer) = unsafe { &G_RENDERER } {
        if let Some(scene) = unsafe { &G_SCENE } {
            scene.render(&renderer);
        }
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_flush() {
    if let Some(renderer) = unsafe { &G_RENDERER } {
        renderer.present();
    }
}
