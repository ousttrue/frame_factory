extern crate gltf2;

pub mod error;
use std::rc::Rc;

pub use error::*;

pub mod scene_manager;
pub use scene_manager::*;

pub mod camera;
pub use camera::*;

pub mod accessor_bytes;
pub use accessor_bytes::*;

pub mod image;
pub use image::*;

pub mod material;
pub use material::*;

pub mod mesh;
pub use mesh::*;

pub mod node;
pub use node::*;

pub mod loader;

pub mod screen_state;
pub use screen_state::*;


pub struct Scene {
    pub camera: Camera,
    pub roots: Vec<Rc<Node>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            roots: Vec::new(),
        }
    }
}
