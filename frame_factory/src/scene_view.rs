use std::{ffi::{CStr}, path::Path, ptr};

use com_ptr::ComPtr;
use com_ptr_util::{ComCreate, ComError};
use frame_factory::{FRAME_FACTORY_scene_load, FRAME_FACTORY_scene_render, FRAME_FACTORY_shutdown};
use winapi::{
    shared::{dxgiformat, dxgitype::DXGI_SAMPLE_DESC},
    um::d3d11,
};

static mut G_NEXT_VIEW_ID: u32 = 1;

fn next_view_id() -> u32 {
    unsafe {
        let id = G_NEXT_VIEW_ID;
        G_NEXT_VIEW_ID += 1;
        id
    }
}

#[derive(Debug)]
struct TextureWithSRV {
    texture: ComPtr<d3d11::ID3D11Texture2D>,
    srv: ComPtr<d3d11::ID3D11ShaderResourceView>,
}

#[derive(Debug)]
pub struct SceneView {
    pub name: String,
    scene_id: u32,
    width: i16,
    height: i16,

    texture_srv: Option<TextureWithSRV>,
}

impl Drop for SceneView {
    fn drop(&mut self) {
        FRAME_FACTORY_shutdown();
    }
}

impl SceneView {
    pub fn load_gltf(path: &[i8]) -> Option<SceneView> {
        let scene_id = FRAME_FACTORY_scene_load(path.as_ptr());
        if scene_id == 0 {
            return None;
        }
        
        let cstr = unsafe { CStr::from_ptr(path.as_ptr()) }.to_owned();
        let path = cstr.to_string_lossy().to_string();
        let path = Path::new(&path);
        Some(SceneView {
            name: format!("{}##view{}\0", path.file_stem().unwrap().to_string_lossy(), next_view_id()),
            scene_id: scene_id,
            width: 0,
            height: 0,
            texture_srv: None,
        })
    }

    pub fn render(
        &mut self,
        device: &ComPtr<d3d11::ID3D11Device>,
        context: &ComPtr<d3d11::ID3D11DeviceContext>,
        state: &scene::ScreenState,
    ) -> Result<ComPtr<d3d11::ID3D11ShaderResourceView>, ComError> {
        if self.width != state.width || self.height != state.height {
            self.texture_srv = None;
            self.width = state.width;
            self.height = state.height;
        }

        if self.texture_srv.is_none() {
            if self.width == 0 || self.height == 0 {
                return Err(ComError::StaticMessage("no texture"));
            }

            let desc = d3d11::D3D11_TEXTURE2D_DESC {
                Width: state.width as u32,
                Height: state.height as u32,
                MipLevels: 1,
                ArraySize: 1,
                Format: dxgiformat::DXGI_FORMAT_B8G8R8X8_UNORM,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: d3d11::D3D11_USAGE_DEFAULT,
                BindFlags: d3d11::D3D11_BIND_SHADER_RESOURCE | d3d11::D3D11_BIND_RENDER_TARGET,
                ..Default::default()
            };

            let texture = ComPtr::create_if_success(|pp| unsafe {
                device.CreateTexture2D(&desc, ptr::null_mut(), pp)
            })?;

            // create SRV
            let srv = ComPtr::create_if_success(|pp| unsafe {
                device.CreateShaderResourceView(
                    texture.as_ptr() as *mut d3d11::ID3D11Resource,
                    ptr::null(),
                    pp,
                )
            })?;

            self.texture_srv = Some(TextureWithSRV { texture, srv });
        }

        let TextureWithSRV { texture, srv } = &self.texture_srv.as_ref().unwrap();
        if FRAME_FACTORY_scene_render(
            self.scene_id,
            device.as_ptr(),
            context.as_ptr(),
            texture.as_ptr(),
            state,
        ) {
            Ok(srv.clone())
        } else {
            Err(ComError::StaticMessage("fail to scene_render"))
        }
    }
}
