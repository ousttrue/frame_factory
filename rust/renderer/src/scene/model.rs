use cgmath::One;

use super::AccessorBytes;
pub enum Shader {
    UnLight,
}

pub struct Model {
    id: u32,
    pub transform: cgmath::Matrix4<f32>,
    pub positions: AccessorBytes,
    pub indices: AccessorBytes,
    pub material: Shader,
}

static mut g_next_id: u32 = 1;

impl Model {
    pub fn new(positions: AccessorBytes, indices: AccessorBytes, material: Shader) -> Model {
        let id = unsafe { g_next_id };
        unsafe { g_next_id += 1 };
        Model {
            id,
            transform: cgmath::Matrix4::<f32>::one(),
            positions,
            indices,
            material,
        }
    }

    pub fn create_triangle() -> Model {
        let size = 0.5f32;
        Self::new(
            AccessorBytes::from(&[
                (size, -size, 0.0f32),
                (-size, -size, 0.0f32),
                (0.0f32, size, 0.0f32),
            ]),
            AccessorBytes::from(&[0, 1, 2]),
            Shader::UnLight,
        )
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
