mod com_util;
mod renderer;
mod shader;
mod vertex_buffer;

use cgmath::{Matrix, One};
use com_ptr::ComPtr;
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

struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,
}

impl Scene {
    fn render(&self, d3d_device: &d3d11::ID3D11Device, d3d_context: &d3d11::ID3D11DeviceContext) {
        // update constant buffer
        self.shader
            .vs_constant_buffer
            .update(d3d_context, 0, self.model.as_ptr());

        // model
        self.shader.set(d3d_context);
        self.vertex_buffer.draw(d3d_context);
    }
}

static mut G_SCENE: Option<Scene> = None;

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_destroy() {
    unsafe { G_SCENE = None };
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_create(
    device: *mut d3d11::ID3D11Device,
    source: *const u8,
    source_size: usize,
    vs_main: *const u8,
    ps_main: *const u8,
) -> bool {
    // if let Some(renderer) = unsafe { &G_RENDERER } {
    let d3d_device = unsafe { ComPtr::from_raw(device) };

    if let Ok((vs, input_layout, vs_constant_buffer)) =
        Shader::compile_vertex_shader(&d3d_device, source, source_size, "vsMain\0")
    {
        if let Ok(ps) = Shader::compile_pixel_shader(&d3d_device, source, source_size, "psMain\0") {
            let shader = Shader {
                vs,
                ps,
                input_layout,
                vs_constant_buffer,
            };

            if let Ok(vertex_buffer) = VertexBuffer::create_triangle(&d3d_device) {
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
    // }

    false
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_render(
    device: *mut d3d11::ID3D11Device,
    context: *mut d3d11::ID3D11DeviceContext,
) {
    if let Some(scene) = unsafe { &G_SCENE } {
        unsafe {
            scene.render(device.as_ref().unwrap(), context.as_ref().unwrap());
        }
    }
}
