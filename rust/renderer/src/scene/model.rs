use winapi::um::d3d11;

use super::frame::c0;
use crate::{shader::Shader, vertex_buffer::VertexBuffer};
use cgmath::One;

pub struct Model {
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,
    shader: Shader,
}

impl Model {
    pub fn new(vertex_buffer: VertexBuffer, shader: Shader) -> Model {
        Model {
            model: cgmath::Matrix4::one(),
            shader,
            vertex_buffer,
        }
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
