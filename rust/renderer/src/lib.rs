extern crate gltf;
extern crate serde_json;
mod com_util;
mod renderer;
mod rendertarget;
mod scene;
mod shader;
mod vertex_buffer;
use scene::{loader::Loader, model::Model, screenstate::ScreenState, Scene};
use shader::{Shader, ShaderSource};
use std::{collections::HashMap, ffi::CStr, path::Path};
use vertex_buffer::VertexBuffer;

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
pub extern "C" fn FRAME_FACTORY_shutdown() {
    unsafe { G_SCENE = None };
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_destroy(scene: u32) {
    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
        scene_manager.scenes.remove(&scene).unwrap();
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_sample(
    device: *mut d3d11::ID3D11Device,
    source: *const u8,
    source_size: usize,
    vs_main: *const i8,
    ps_main: *const i8,
) -> u32 {
    let d3d_device = unsafe { device.as_ref().unwrap() };
    or_create();

    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
        let mut scene = Scene::new();

        let source = ShaderSource::new(source, source_size, vs_main, ps_main);

        if let Ok(vertex_buffer) = VertexBuffer::create_triangle(d3d_device) {
            if let Ok(shader) = Shader::compile(d3d_device, &source) {
                let model = Model::new(vertex_buffer, shader);
                scene.models.push(model);
            }
        }

        return scene_manager.add(scene);
    }

    0
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_scene_load(
    device: *mut d3d11::ID3D11Device,
    path: *const i8,
) -> u32 {
    let d3d_device = unsafe { device.as_ref().unwrap() };
    or_create();

    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
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
    if let Some(scene_manager) = unsafe { &mut G_SCENE } {
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
