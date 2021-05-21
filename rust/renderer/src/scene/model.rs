use winapi::um::d3d11;

use super::frame::c0;
use crate::resource;
use cgmath::One;

pub struct Model {
    pub model: cgmath::Matrix4<f32>,
    pub vertex_buffer: resource::VertexBuffer,
    pub shader: resource::Shader,
}

impl Model {
    pub fn new(vertex_buffer: resource::VertexBuffer, shader: resource::Shader) -> Model {
        Model {
            model: cgmath::Matrix4::one(),
            shader,
            vertex_buffer,
        }
    }
}
