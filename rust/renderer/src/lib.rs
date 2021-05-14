mod com_util;
mod renderer;
mod rendertarget;
mod scene;
mod shader;
mod vertex_buffer;

use scene::Scene;

use winapi::um::d3d11::{self};

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

    if let Ok(scene) = Scene::create(d3d_device, source, source_size, vs_main, ps_main) {
        unsafe { G_SCENE = Some(scene) };
        true
    } else {
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
            scene.render(device.as_ref().unwrap(), context.as_ref().unwrap(), texture);
            return true;
        }
    }

    false
}
