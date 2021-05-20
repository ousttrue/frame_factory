extern crate gltf2;
extern crate serde_json;
mod asset_manager;
mod com_util;
mod renderer;
mod rendertarget;
mod scene;
mod shader;
mod vertex_buffer;
use scene::{loader::Loader, model::Model, scene_manager, screen_state::ScreenState, Scene};
use shader::{Shader};
use std::{error::Error, ffi::CStr, path::Path};
use vertex_buffer::VertexBuffer;

use winapi::um::d3d11::{self};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn to_string(p: *const i8) -> String {
    unsafe { CStr::from_ptr(p).to_str().unwrap().to_owned() }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_asset_path(path: *const i8) {
    if let Some(asset_manager) = asset_manager::get() {
        asset_manager.asset_path = to_string(path);
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_shutdown() {
    scene::scene_manager::shutdown();
    asset_manager::shutdown();
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_destroy(scene: u32) {
    if let Some(scene_manager) = scene_manager::get() {
        scene_manager.scenes.remove(&scene).unwrap();
    }
}

fn create_sample_scene(
    d3d_device: &d3d11::ID3D11Device,
    shader_path: *const i8,
) -> Result<u32, Box<dyn Error>> {
    let asset_manager = asset_manager::get().unwrap();
    let shader_source = asset_manager.get_shader_source(&to_string(shader_path))?;
    let scene_manager = scene::scene_manager::get().unwrap();
    let mut scene = Scene::new();

    if let Ok(vertex_buffer) = VertexBuffer::create_triangle(d3d_device) {
        if let Ok(shader) = Shader::compile(d3d_device, &shader_source) {
            let model = Model::new(vertex_buffer, shader);
            scene.models.push(model);
        }
    }

    Ok(scene_manager.add(scene))
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_sample(
    device: *mut d3d11::ID3D11Device,
    shader_path: *const i8,
) -> u32 {
    let d3d_device = unsafe { device.as_ref().unwrap() };

    if let Ok(scene) = create_sample_scene(d3d_device, shader_path) {
        scene
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_load(
    device: *mut d3d11::ID3D11Device,
    path: *const i8,
) -> u32 {
    let d3d_device = unsafe { device.as_ref().unwrap() };

    if let Some(scene_manager) = scene::scene_manager::get() {
        let path = unsafe { CStr::from_ptr(path) };
        if let Ok(path) = path.to_str() {
            let path = Path::new(path);

            let loader = Loader::new();
            if let Ok(()) = loader.load(d3d_device, path) {
                let mut scene = Scene::new();
                for m in loader.models {
                    scene.models.push(m);
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
    state: *const ScreenState,
) -> bool {
    if let Some(scene_manager) = scene::scene_manager::get() {
        if let Some(scene) = scene_manager.scenes.get_mut(&scene) {
            unsafe {
                scene.render(
                    device.as_ref().unwrap(),
                    context.as_ref().unwrap(),
                    texture,
                    state.as_ref().unwrap(),
                );
                return true;
            }
        }
    }

    false
}
