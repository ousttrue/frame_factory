use std::ptr;

use com_ptr::ComPtr;
use winapi::um::d3d11;

use crate::com_util::{ComError, ComCreate};

pub struct RenderTarget {
    width: f32,
    height: f32,
    pub texture: *mut d3d11::ID3D11Texture2D,
    pub rtv: ComPtr<d3d11::ID3D11RenderTargetView>,
}

impl RenderTarget {
    pub fn create(
        d3d_device: &d3d11::ID3D11Device,
        texture: *mut d3d11::ID3D11Texture2D,
    ) -> Result<RenderTarget, ComError> {
        let mut desc = d3d11::D3D11_TEXTURE2D_DESC::default();
        unsafe { texture.as_ref().unwrap().GetDesc(&mut desc) };

        // rtv
        let rtv = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateRenderTargetView(texture as *mut d3d11::ID3D11Resource, ptr::null(), pp)
        })?;

        Ok(RenderTarget {
            width: desc.Width as f32,
            height: desc.Height as f32,
            texture,
            rtv,
        })
    }

    pub fn set_and_clear(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        // clear
        unsafe {
            d3d_context.ClearRenderTargetView(self.rtv.as_ptr(), &[0.2f32, 0.2f32, 0.2f32, 1.0f32])
        };

        // set backbuffer
        let rtv_list = [self.rtv.as_ptr()];
        unsafe { d3d_context.OMSetRenderTargets(1, rtv_list.as_ptr(), ptr::null_mut()) };

        let viewports = d3d11::D3D11_VIEWPORT {
            TopLeftX: 0.0f32,
            TopLeftY: 0.0f32,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0f32,
            MaxDepth: 1.0f32,
        };
        unsafe { d3d_context.RSSetViewports(1, &viewports) };
    }
}
