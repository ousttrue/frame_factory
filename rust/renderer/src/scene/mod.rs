pub mod scene_manager;
pub use scene_manager::*;

pub mod camera;
pub use camera::*;

pub mod accessor_bytes;
pub use accessor_bytes::*;

pub mod material;
pub use material::*;

pub mod mesh;
pub use mesh::*;

pub mod frame;
pub use frame::*;

pub mod loader;

pub mod screen_state;
pub use screen_state::*;

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Mesh>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            models: Vec::new(),
        }
    }
}
