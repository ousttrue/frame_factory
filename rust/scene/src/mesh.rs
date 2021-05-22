use std::rc::Rc;

use cgmath::One;

use super::{AccessorBytes, Material};

pub struct Submesh {
    pub material: Rc<Material>,
    pub offset: u32,
    pub index_count: u32,
}

pub struct Mesh {
    id: u32,
    pub transform: cgmath::Matrix4<f32>,
    pub positions: AccessorBytes,
    pub indices: AccessorBytes,
    pub submeshes: Vec<Submesh>,
}

static mut G_NEXT_ID: u32 = 1;

fn next_id() -> u32 {
    unsafe {
        let id = G_NEXT_ID;
        G_NEXT_ID += 1;
        id
    }
}

impl Mesh {
    pub fn new(positions: AccessorBytes, indices: AccessorBytes) -> Mesh {
        let id = next_id();
        Mesh {
            id,
            transform: cgmath::Matrix4::<f32>::one(),
            positions,
            indices,
            submeshes: Vec::new(),
        }
    }

    pub fn create_triangle() -> Mesh {
        let size = 0.5f32;
        Self::new(
            AccessorBytes::from(&[
                (size, -size, 0.0f32),
                (-size, -size, 0.0f32),
                (0.0f32, size, 0.0f32),
            ]),
            AccessorBytes::from(&[0, 1, 2]),
        )
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}