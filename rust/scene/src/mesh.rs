use std::{collections::HashMap, rc::Rc};

use cgmath::One;

use super::{AccessorBytes, Material};

pub struct Submesh {
    pub material: Rc<Material>,
    pub offset: u32,
    pub index_count: u32,
}

pub struct Mesh {
    id: u32,
    vertex_count: u32,
    pub transform: cgmath::Matrix4<f32>,
    pub vertex_buffers: HashMap<String, AccessorBytes>,
    pub index_buffer: AccessorBytes,
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
    pub fn new(indices: AccessorBytes, vertex_count: u32) -> Mesh {
        let id = next_id();
        Mesh {
            id,
            vertex_count,
            transform: cgmath::Matrix4::<f32>::one(),
            vertex_buffers: HashMap::new(),
            index_buffer: indices,
            submeshes: Vec::new(),
        }
    }

    pub fn create_triangle() -> Mesh {
        let size = 0.5f32;
        let mut mesh = Self::new(AccessorBytes::from(&[0, 1, 2]), 3);

        mesh.vertex_buffers.insert(
            String::from("POSITION"),
            AccessorBytes::from(&[
                (size, -size, 0.0f32),
                (-size, -size, 0.0f32),
                (0.0f32, size, 0.0f32),
            ]),
        );

        mesh
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn push_vertex_buffer(&mut self, key: &str, bytes: &[u8], stride: u32, count: u32) {
        if !self.vertex_buffers.contains_key(key) {
            self.vertex_buffers.insert(
                key.to_owned(),
                AccessorBytes::create(stride, self.vertex_count),
            );
        }

        self.vertex_buffers
            .get_mut(key)
            .unwrap()
            .push(bytes, stride, count);
    }
}
