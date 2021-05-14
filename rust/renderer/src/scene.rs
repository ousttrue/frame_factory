use cgmath::{Matrix, One};
use winapi::um::d3d11;

use super::rendertarget::RenderTarget;
use crate::{com_util::ComError, shader::Shader, vertex_buffer::VertexBuffer};

pub struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,

    render_target: Option<RenderTarget>,
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

        let fovy = cgmath::Rad::from(cgmath::Deg(60.0));
        let projection: cgmath::Matrix4<f32> = cgmath::perspective(fovy, 1.0, 0.1, 2.0);
        let view: cgmath::Matrix4<f32> = cgmath::Matrix4::one();
        let model: cgmath::Matrix4<f32> = cgmath::Matrix4::one();

        Ok(Scene {
            shader,
            model,
            vertex_buffer,
            render_target: None,
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
