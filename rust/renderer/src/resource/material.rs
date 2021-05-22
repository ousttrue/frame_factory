use std::{io::Read, ptr, rc::Rc};

use crate::com_util::{ComCreate, ComError};
use com_ptr::ComPtr;
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype},
    um::{d3d11, d3dcommon},
};

use crate::error::Error;

use super::Shader;

pub fn load_texture(
    d3d_device: &d3d11::ID3D11Device,
    buffer: &[u8],
) -> Result<ComPtr<d3d11::ID3D11Texture2D>, Error> {
    let loaded = image::load_from_memory(buffer).map_err(|e| Error::ImageError(e))?;

    let raw = loaded.into_rgba8();

    let desc = d3d11::D3D11_TEXTURE2D_DESC {
        Width: raw.width(),
        Height: raw.height(),
        MipLevels: 1,
        ArraySize: 1,
        Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
        SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: d3d11::D3D11_USAGE_DEFAULT,
        BindFlags: d3d11::D3D11_BIND_SHADER_RESOURCE,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };

    let data = d3d11::D3D11_SUBRESOURCE_DATA {
        pSysMem: raw.as_ptr() as *const winapi::ctypes::c_void,
        SysMemPitch: raw.width() * 4,
        SysMemSlicePitch: raw.width() * raw.height() * 4,
    };

    ComPtr::create_if_success(|pp| unsafe { d3d_device.CreateTexture2D(&desc, &data, pp) })
        .map_err(|e| Error::ComError(e))
}

pub struct Texture {
    pub buffer: ComPtr<d3d11::ID3D11Texture2D>,
    pub srv: ComPtr<d3d11::ID3D11ShaderResourceView>,
}

impl Texture {
    pub fn new(
        d3d_device: &d3d11::ID3D11Device,
        buffer: ComPtr<d3d11::ID3D11Texture2D>,
    ) -> Result<Texture, ComError> {
        let srv = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateShaderResourceView(
                buffer.as_ptr() as *mut d3d11::ID3D11Resource,
                ptr::null_mut(),
                pp,
            )
        })?;
        Ok(Texture { buffer, srv })
    }
}

pub struct Material {
    pub name: String,
    pub shader: Rc<Shader>,
    pub color_texture: Option<Rc<Texture>>,
}
