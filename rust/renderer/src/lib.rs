mod com_util;
mod renderer;
mod shader;
mod vertex_buffer;

use cgmath::{Matrix, One};
use renderer::Renderer;
use shader::Shader;
use std::{ffi::c_void, ptr};
use vertex_buffer::VertexBuffer;
use winapi::{
    shared::windef::HWND,
    um::{d3d11, d3d11sdklayers},
};

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
    if !hwnd.is_null() {
        if let Ok(renderer) = renderer::Renderer::new(hwnd) {
            unsafe {
                G_RENDERER = Some(renderer);
                if let Some(renderer) = &G_RENDERER {
                    return renderer.d3d_device.as_ptr() as *const c_void;
                }
            }
        }
    }

    unsafe {
        G_SCENE = None;
        // if let Some(renderer) = &G_RENDERER {
        //     if let Ok(debug) = renderer
        //         .d3d_device
        //         .query_interface::<d3d11sdklayers::ID3D11Debug>()
        //     {
        //         debug.ReportLiveDeviceObjects(d3d11sdklayers::D3D11_RLDO_DETAIL);
        //     }
        // }
        G_RENDERER = None;
    }
    ptr::null()
}

struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,
}

impl Scene {
    fn render(&self, renderer: &Renderer) {
        // update constant buffer
        self.shader
            .vs_constant_buffer
            .update(&renderer.d3d_context, 0, self.model.as_ptr());

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
pub extern "C" fn FRAME_FACTORY_new_frame(width: u32, height: u32) -> bool {
    if let Some(renderer) = unsafe { &mut G_RENDERER } {
        if let Ok(rtv) = renderer.get_or_create_backbuffer(width, height) {
            // render_target.prepare(&renderer.d3d_context);

            unsafe {
                // clear
                renderer
                    .d3d_context
                    .ClearRenderTargetView(rtv.as_ptr(), &[0.0, 0.2, 0.4, 1.0]);

                // set render target
                renderer.d3d_context.OMSetRenderTargets(
                    1,
                    [rtv.as_ptr()].as_ptr(),
                    ptr::null_mut(),
                );

                // viewport
                let viewport = d3d11::D3D11_VIEWPORT {
                    Width: width as f32,
                    Height: height as f32,
                    ..Default::default()
                };
                renderer.d3d_context.RSSetViewports(1, &viewport);
            }

            return true;
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
