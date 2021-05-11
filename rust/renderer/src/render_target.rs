use std::ptr;

use crate::com_util::{ComCreate, ComError};
use com_ptr::ComPtr;
use winapi::{shared::dxgi, um::d3d11, Interface};

pub struct RenderTarget {
    d3d_rtv: ComPtr<d3d11::ID3D11RenderTargetView>,
}

impl RenderTarget {
    pub fn from_swapchain(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        dxgi_swapchain: &ComPtr<dxgi::IDXGISwapChain>,
    ) -> Result<RenderTarget, ComError> {
        let d3d_back_buffer: ComPtr<d3d11::ID3D11Resource> =
            ComPtr::create_if_success(|pp| unsafe {
                dxgi_swapchain.GetBuffer(
                    0,
                    &d3d11::ID3D11Texture2D::uuidof(),
                    pp as *mut *mut winapi::ctypes::c_void,
                )
            })?;

        let d3d_rtv: ComPtr<d3d11::ID3D11RenderTargetView> =
            ComPtr::create_if_success(|pp| unsafe {
                d3d_device.CreateRenderTargetView(d3d_back_buffer.as_ptr(), ptr::null(), pp)
            })?;

        Ok(RenderTarget { d3d_rtv: d3d_rtv })
    }

    pub fn prepare(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
        unsafe {
            // set render target
            d3d_context.OMSetRenderTargets(1, [self.d3d_rtv.as_ptr()].as_ptr(), ptr::null_mut());

            // viewport
            let viewport = d3d11::D3D11_VIEWPORT {
                Width: 1280.0,
                Height: 720.0,
                ..Default::default()
            };
            d3d_context.RSSetViewports(1, &viewport);

            // clear
            d3d_context.ClearRenderTargetView(self.d3d_rtv.as_ptr(), &[0.0, 0.2, 0.4, 1.0]);
        }
    }
}
