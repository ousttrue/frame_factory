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
        shader: Shader,
    ) -> Result<Model, ComError> {
        Ok(Model {
            shader,
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
