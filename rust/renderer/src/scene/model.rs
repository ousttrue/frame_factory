use winapi::um::d3d11;

use crate::{com_util::ComError, shader::Shader, vertex_buffer::VertexBuffer};
use cgmath::One;
use super::frame::c0;

pub struct Model {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,
}

impl Model {
    pub fn create(
        d3d_device: &d3d11::ID3D11Device,
        source: *const u8,
        source_size: usize,
        vs_main: *const u8,
        ps_main: *const u8,
    ) -> Result<Model, ComError> {
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

        Ok(Model {
            shader,
            model,
            vertex_buffer,
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
