extern crate renderer;
extern crate scene;

use std::{ffi::CStr, path::Path, ptr};

use renderer::{asset_manager, resource};
use winapi::um::d3d11;

fn to_string(p: *const i8) -> String {
    unsafe { CStr::from_ptr(p).to_str().unwrap().to_owned() }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_message_peek(i: u32) -> *const u8 {
    if let Some(message_manager) = scene::message::get() {
        if let Some(message) = message_manager.queue.get(i as usize) {
            message.as_ptr()
        } else {
            ptr::null() as *const u8
        }
    } else {
        ptr::null() as *const u8
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_message_clear() {
    if let Some(message_manager) = scene::message::get() {
        message_manager.queue.clear()
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_asset_path(path: *const i8) {
    if let Some(asset_manager) = asset_manager::get() {
        asset_manager.asset_path = to_string(path);
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_shutdown() {
    resource::shutdown();
    scene::scene_manager::shutdown();
    asset_manager::shutdown();
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_destroy(scene: u32) {
    if let Some(scene_manager) = scene::scene_manager::get() {
        scene_manager.scenes.remove(&scene).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_load(path: *const i8) -> u32 {
    if let Some(scene_manager) = scene::scene_manager::get() {
        let path = unsafe { CStr::from_ptr(path) };
        if let Ok(path) = path.to_str() {
            let path = Path::new(path);

            if let Ok(loader) = scene::loader::load(path) {
                let mut scene = scene::Scene::new();
                for root in loader
                    .nodes
                    .iter()
                    .filter(|node| node.get_parent().is_none())
                {
                    scene.roots.push(root.clone());
                }
                return scene_manager.add(scene);
            }
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_render(
    scene: u32,
    device: *mut d3d11::ID3D11Device,
    context: *mut d3d11::ID3D11DeviceContext,
    texture: *mut d3d11::ID3D11Texture2D,
    state: *const scene::ScreenState,
) -> bool {
    if let Some(scene_manager) = scene::scene_manager::get() {
        if let Some(scene) = scene_manager.scenes.get_mut(&scene) {
            let state = unsafe { state.as_ref().unwrap() };
            // update camera
            scene.camera.update(state);

            if let Some(resource_manager) = resource::get() {
                unsafe {
                    resource_manager.render(
                        device.as_ref().unwrap(),
                        context.as_ref().unwrap(),
                        texture,
                        scene,
                    )
                };

                return true;
            }
        }
    }

    false
}
