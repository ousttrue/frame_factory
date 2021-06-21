extern crate gltf2;
#[macro_use]
extern crate bitflags;

pub mod message;

pub mod error;
use std::rc::Rc;

use cgmath::InnerSpace;
pub use error::*;

pub mod scene_manager;
pub use scene_manager::*;

pub mod camera;
pub use camera::*;

pub mod accessor_bytes;
pub use accessor_bytes::*;

pub mod image;
pub use image::*;

pub mod texture;
pub use texture::*;

pub mod light;
pub use light::*;

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
    pub light: Light,
    pub roots: Vec<Rc<Node>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            light: Light::DirectionalLight(cgmath::Vector3::new(1f32, -1f32, 1f32).normalize()),
            roots: Vec::new(),
        }
    }
}
