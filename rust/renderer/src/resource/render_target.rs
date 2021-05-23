use std::ptr;

use com_ptr::ComPtr;
use winapi::{
    shared::{dxgiformat, dxgitype},
    um::d3d11,
};

use crate::com_util::{ComCreate, ComError};

pub struct RenderTarget {
    width: f32,
    height: f32,
    pub texture: *mut d3d11::ID3D11Texture2D,
    pub rtv: ComPtr<d3d11::ID3D11RenderTargetView>,
    pub depth: ComPtr<d3d11::ID3D11Texture2D>,
    pub dsv: ComPtr<d3d11::ID3D11DepthStencilView>,
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
            d3d_device.CreateRenderTargetView(
                texture as *mut d3d11::ID3D11Resource,
                ptr::null(),
                pp,
            )
        })?;

        // depth
        let depth_desc = d3d11::D3D11_TEXTURE2D_DESC {
            Width: desc.Width,
            Height: desc.Height,
            MipLevels: 1,
            ArraySize: 1,
            Format: dxgiformat::DXGI_FORMAT_D24_UNORM_S8_UINT,
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: d3d11::D3D11_USAGE_DEFAULT,
            BindFlags: d3d11::D3D11_BIND_DEPTH_STENCIL,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let depth = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateTexture2D(&depth_desc, ptr::null(), pp)
        })?;

        let dsv = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateDepthStencilView(
                depth.as_ptr() as *mut d3d11::ID3D11Resource,
                ptr::null(),
                pp,
            )
        })?;

        Ok(RenderTarget {
            width: desc.Width as f32,
            height: desc.Height as f32,
            texture,
            rtv,
            depth,
            dsv,
        })
    }

    pub fn set_and_clear(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        // clear
        unsafe {
            d3d_context.ClearRenderTargetView(self.rtv.as_ptr(), &[0.2f32, 0.2f32, 0.2f32, 1.0f32]);
			d3d_context.ClearDepthStencilView(self.dsv.as_ptr()
				, d3d11::D3D11_CLEAR_DEPTH | d3d11::D3D11_CLEAR_STENCIL, 1f32, 0);

        };

        // set backbuffer
        let rtv_list = [self.rtv.as_ptr()];
        unsafe { d3d_context.OMSetRenderTargets(1, rtv_list.as_ptr(), self.dsv.as_ptr()) };

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
