mod com_util;
mod renderer;
mod rendertarget;
mod scene;
mod shader;
mod vertex_buffer;

use std::{collections::HashMap, ffi::c_void};

use scene::{Scene, ScreenState};

use winapi::um::d3d11::{self};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct SceneManager {
    next_id: u32,
    scenes: HashMap<u32, Scene>,
}

impl SceneManager {
    fn add(&mut self, scene: Scene) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.scenes.insert(id, scene);
        id
    }
}

static mut G_SCENE: Option<Box<SceneManager>> = None;

fn or_create() {
    unsafe {
        if G_SCENE.is_none() {
            G_SCENE = Some(Box::new(SceneManager {
                next_id: 1,
                scenes: HashMap::new(),
            }))
        }
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_destroy() {
    unsafe { G_SCENE = None };
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_create(
    device: *mut d3d11::ID3D11Device,
    source: *const u8,
    source_size: usize,
    vs_main: *const u8,
    ps_main: *const u8,
) -> u32 {
    let d3d_device = unsafe { device.as_ref().unwrap() };

    or_create();

    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
        if let Ok(scene) = Scene::create(d3d_device, source, source_size, vs_main, ps_main) {
            return scene_manager.add(scene);
        }
    }

    0
}

enum MouseButtonFlags {
    None = 0,
    LeftDown = 0x01,
    RightDown = 0x02,
    MiddleDown = 0x04,
    WheelPlus = 0x08,
    WheelMinus = 0x10,
    CursorUpdate = 0x20,
}


#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_render(
    scene: u32,
    device: *mut d3d11::ID3D11Device,
    context: *mut d3d11::ID3D11DeviceContext,
    texture: *mut d3d11::ID3D11Texture2D,
    state: *const ScreenState,
) -> bool {
    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
        if let Some(scene) = scene_manager.scenes.get_mut(&scene) {
            unsafe {
                scene.render(device.as_ref().unwrap(), context.as_ref().unwrap(), texture, state.as_ref().unwrap());
                return true;
            }
        }
    }

    false
}
