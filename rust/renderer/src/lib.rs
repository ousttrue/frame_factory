mod com_util;
mod renderer;
mod shader;
mod vertex_buffer;

use cgmath::{Matrix, One};
use com_ptr::ComPtr;
use com_util::{ComCreate, ComError};
use renderer::Renderer;
use shader::Shader;
use std::{ffi::c_void, ptr};
use vertex_buffer::VertexBuffer;
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, windef::HWND},
    um::{
        d3d11::{self, ID3D11Resource},
        d3d11sdklayers,
    },
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct RenderTarget {
    width: f32,
    height: f32,
    texture: *mut d3d11::ID3D11Texture2D,
    rtv: ComPtr<d3d11::ID3D11RenderTargetView>,
}

impl RenderTarget {
    fn create(
        d3d_device: &d3d11::ID3D11Device,
        texture: *mut d3d11::ID3D11Texture2D,
    ) -> Result<RenderTarget, ComError> {
        let mut desc = d3d11::D3D11_TEXTURE2D_DESC::default();
        unsafe { texture.as_ref().unwrap().GetDesc(&mut desc) };

        // rtv
        let rtv = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateRenderTargetView(texture as *mut ID3D11Resource, ptr::null(), pp)
        })?;

        Ok(RenderTarget {
            width: desc.Width as f32,
            height: desc.Height as f32,
            texture,
            rtv,
        })
    }

    fn set_and_clear(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        // clear
        unsafe {
            d3d_context.ClearRenderTargetView(self.rtv.as_ptr(), &[0.2f32, 0.2f32, 0.2f32, 1.0f32])
        };

        // set backbuffer
        let rtv_list = [self.rtv.as_ptr()];
        unsafe { d3d_context.OMSetRenderTargets(1, rtv_list.as_ptr(), ptr::null_mut()) };

        let viewports = d3d11::D3D11_VIEWPORT {
            TopLeftX: 0.0f32,
            TopLeftY: 0.0f32,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0f32,
            MaxDepth: 1.0f32,
        };
        unsafe { d3d_context.RSSetViewports(1, &viewports) };
    }
}

struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,

    render_target: Option<RenderTarget>,
}

impl Scene {
    fn get_or_create_rtv(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        texture: *mut d3d11::ID3D11Texture2D,
    ) {
        if let Some(render_target) = &self.render_target {
            if render_target.texture == texture {
                return;
            }
        }

        self.render_target = RenderTarget::create(d3d_device, texture).ok();
    }

    fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
    ) {
        self.get_or_create_rtv(d3d_device, texture);
        if let Some(render_target) = &self.render_target {
            render_target.set_and_clear(d3d_context);

            // update constant buffer
            self.shader
                .vs_constant_buffer
                .update(d3d_context, 0, self.model.as_ptr());

            // model
            self.shader.set(d3d_context);
            self.vertex_buffer.draw(d3d_context);
        }
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
                    render_target: None,
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
    texture: *mut d3d11::ID3D11Texture2D,
) -> bool {
    if let Some(scene) = unsafe { &mut G_SCENE } {
        unsafe {
            scene.render(
                device.as_ref().unwrap(),
                context.as_ref().unwrap(),
                texture,
            );
            return true;
        }
    }

    false
}
