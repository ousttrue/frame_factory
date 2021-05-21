pub mod scene_manager;
pub use scene_manager::*;

pub mod camera;
pub use camera::*;

pub mod model;
pub use model::*;

pub mod frame;
pub use frame::*;

pub mod loader;

pub mod screen_state;
pub use screen_state::*;

pub struct Scene {
    pub camera: Camera,
    pub models: Vec<Model>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),
            models: Vec::new(),
        }
    }
}
