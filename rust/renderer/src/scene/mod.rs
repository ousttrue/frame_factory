use std::cell::Cell;

use cgmath::{Matrix, One};
use winapi::um::d3d11;

use super::rendertarget::RenderTarget;
use crate::{com_util::ComError, shader::Shader, vertex_buffer::VertexBuffer};

pub mod camera;
use self::{camera::Camera, screenstate::ScreenState};
pub mod screenstate;
pub mod loader;

#[repr(C)]
struct c0 {
    view: [f32; 16],
    projection: [f32; 16],
}

#[allow(dead_code)]
#[repr(C)]
struct c1 {
    model: [f32; 16],
}

pub struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,

    render_target: Option<RenderTarget>,
    camera: Camera,
}

struct BytesReader<'a> {
    pos: Cell<usize>,
    buf: &'a [u8],
}

impl Scene {
    pub fn create(
        d3d_device: &d3d11::ID3D11Device,
        source: *const u8,
        source_size: usize,
        vs_main: *const u8,
        ps_main: *const u8,
    ) -> Result<Scene, ComError> {
        let (vs, input_layout, vs_constant_buffer) =
            Shader::compile_vertex_shader(&d3d_device, source, source_size, vs_main)?;
        let ps = Shader::compile_pixel_shader(d3d_device, source, source_size, ps_main)?;
        let shader = Shader {
            vs,
            ps,
            input_layout,
            vs_constant_buffer,
        };
        let vertex_buffer = VertexBuffer::create_triangle(d3d_device)?;

        let model: cgmath::Matrix4<f32> = cgmath::Matrix4::one();

        Ok(Scene {
            shader,
            model,
            vertex_buffer,
            render_target: None,
            camera: Camera::new(),
        })
    }

    pub fn get_or_create_rtv(
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

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
        state: &ScreenState,
    ) {
        // update camera
        self.camera.update(state);

        // render
        self.get_or_create_rtv(d3d_device, texture);
        if let Some(render_target) = &self.render_target {
            render_target.set_and_clear(d3d_context);

            // update constant buffer
            let frame = c0 {
                view: Default::default(),
                projection: Default::default(),
            };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.camera.view.as_ptr() as *const u8,
                    frame.view.as_ptr() as *mut u8,
                    64,
                );
                std::ptr::copy_nonoverlapping(
                    self.camera.projection.as_ptr() as *const u8,
                    frame.projection.as_ptr() as *mut u8,
                    64,
                );
            }
            self.shader
                .vs_constant_buffer
                .update(d3d_context, 0, &frame);

            self.shader
                .vs_constant_buffer
                .update(d3d_context, 1, &self.model);

            // model
            self.shader.set(d3d_context);
            self.vertex_buffer.draw(d3d_context);
        }
    }
}
