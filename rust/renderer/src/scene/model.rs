use winapi::um::d3d11;

use super::frame::c0;
use crate::{com_util::ComError, shader::Shader, vertex_buffer::VertexBuffer};
use cgmath::One;

pub struct Model {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,
}

impl Model {
    pub fn create_triangle(
        d3d_device: &d3d11::ID3D11Device,
        source: *const u8,
        source_size: usize,
        vs_main: *const u8,
        ps_main: *const u8,
    ) -> Result<Model, ComError> {
        let (vs, input_layout, vs_constant_buffer) =
            Shader::compile_vertex_shader(&d3d_device, source, source_size, vs_main)?;
        let ps = Shader::compile_pixel_shader(d3d_device, source, source_size, ps_main)?;

        Ok(Model {
            shader: Shader::new(vs, vs_constant_buffer, ps, input_layout),
            model: cgmath::Matrix4::one(),
            vertex_buffer: VertexBuffer::create_triangle(d3d_device)?,
        })
    }

    pub fn render(&self, d3d_context: &d3d11::ID3D11DeviceContext, frame: &c0) {
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
