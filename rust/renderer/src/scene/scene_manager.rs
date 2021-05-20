use super::Scene;
use std::collections::HashMap;

pub struct SceneManager {
    next_id: u32,
    pub scenes: HashMap<u32, Scene>,
}

impl SceneManager {
    pub fn add(&mut self, scene: Scene) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.scenes.insert(id, scene);
        id
    }
}

static mut G_SCENE: Option<Box<SceneManager>> = None;

pub fn get() -> Option<&'static mut Box<SceneManager>> {
    unsafe {
        if G_SCENE.is_none() {
            G_SCENE = Some(Box::new(SceneManager {
                next_id: 1,
                scenes: HashMap::new(),
            }))
        }
    }

    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
        Some(scene_manager)
    } else {
        None
    }
}

pub fn shutdown() {
    unsafe { G_SCENE = None };
}
