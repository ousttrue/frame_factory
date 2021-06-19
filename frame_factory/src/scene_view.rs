use std::ptr;

use com_ptr::ComPtr;
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

pub struct SceneView {
    pub name: String,
    scene_id: u32,
    width: u32,
    height: u32,
    texture: Option<ComPtr<d3d11::ID3D11Texture2D>>,
    srv: Option<ComPtr<d3d11::ID3D11ShaderResourceView>>,
}

impl Drop for SceneView {
    fn drop(&mut self) {
        FRAME_FACTORY_shutdown();
    }
}

impl SceneView {
    pub fn load_gltf(path: &str) -> Option<SceneView> {
        let scene_id = FRAME_FACTORY_scene_load(path.as_ptr() as *mut i8);
        if scene_id == 0 {
            return None;
        }

        Some(SceneView {
            name: format!("{}##view{}", path, next_view_id()),
            scene_id: scene_id,
            width: 0,
            height: 0,
            srv: None,
            texture: None,
        })
    }

    pub fn render(
        &mut self,
        device: &ComPtr<d3d11::ID3D11Device>,
        context: &ComPtr<d3d11::ID3D11DeviceContext>,
        width: u32,
        height: u32,
    ) -> Option<ComPtr<d3d11::ID3D11ShaderResourceView>> {
        if self.width != width || self.height != height {
            self.srv = None;
            self.texture = None;
        }

        if self.texture.is_none() {
            if width == 0 || height == 0 {
                return None;
            }

            self.width = width;
            self.height = height;

            let desc = d3d11::D3D11_TEXTURE2D_DESC {
                Width: width,
                Height: height,
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
            // if unsafe {device.CreateTexture2D(&desc, nullptr, &m_texture)!=0}
            // {
            //     return None;
            // }

            // // create SRV
            // if unsafe LED(device->CreateShaderResourceView(m_texture.Get(),
            //                                             nullptr, &m_srv)))
            // {
            //     return nullptr;
            // }
        }

        if let Some(srv) = &self.srv {
            // if FRAME_FACTORY_scene_render(m_scene, device, context, m_texture.Get(), &state) == 0 {
            //     return None;
            // }
            Some(srv.clone())
        } else {
            None
        }
    }
}
