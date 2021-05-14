mod com_util;
mod renderer;
mod shader;
mod vertex_buffer;
mod scene;
mod rendertarget;

use cgmath::{Matrix, One};
use com_ptr::ComPtr;
use com_util::{ComCreate, ComError};
use renderer::Renderer;
use scene::Scene;
use shader::Shader;
use std::{ffi::c_void, ptr};
use vertex_buffer::VertexBuffer;
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, windef::HWND},
    um::{
        d3d11::{self, ID3D11Resource},
        d3d11sdklayers,
    },
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

static mut G_SCENE: Option<Scene> = None;

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
) -> bool {
    let d3d_device = unsafe { device.as_ref().unwrap() };

    if let Ok(scene) = Scene::create(d3d_device, source, source_size, vs_main, ps_main)
    {
        unsafe { G_SCENE = Some(scene) };
        true
    }
    else{
        false
    }
}

#[no_mangle]
pub extern "C" fn FRAME_FACTORY_sample_render(
    device: *mut d3d11::ID3D11Device,
    context: *mut d3d11::ID3D11DeviceContext,
    texture: *mut d3d11::ID3D11Texture2D,
) -> bool {
    if let Some(scene) = unsafe { &mut G_SCENE } {
        unsafe {
            scene.render(
                device.as_ref().unwrap(),
                context.as_ref().unwrap(),
                texture,
            );
            return true;
        }
    }

    false
}
