use std::ptr;

use com_ptr::ComPtr;
use winapi::{Interface, shared::dxgi, um::d3d11};

pub struct RenderTarget {
    d3d_rtv: *mut d3d11::ID3D11RenderTargetView,
}

impl RenderTarget {
    pub fn from_swapchain(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        dxgi_swapchain: &ComPtr<dxgi::IDXGISwapChain>,
    ) -> Result<RenderTarget, &'static str> {
        let mut d3d_back_buffer: *mut d3d11::ID3D11Texture2D = ptr::null_mut();
        let result = unsafe {
            let mut p: *mut *mut d3d11::ID3D11Texture2D = &mut d3d_back_buffer;
            dxgi_swapchain.GetBuffer(
                0,
                &d3d11::ID3D11Texture2D::uuidof(),
                p as *mut *mut winapi::ctypes::c_void,
            )
        };
        if result != 0 {
            return Err("ID3D11Texture2D::get_buffer");
        }
        assert!(!d3d_back_buffer.is_null());

        let mut d3d_rtv: *mut d3d11::ID3D11RenderTargetView = ptr::null_mut();
        let result = unsafe {
            d3d_device.CreateRenderTargetView(
                d3d_back_buffer as *mut d3d11::ID3D11Resource,
                ptr::null(),
                &mut d3d_rtv,
            )
        };
        if result != 0 {
            return Err("create_render_target_view");
        }
        assert!(!d3d_rtv.is_null());

        Ok(RenderTarget { d3d_rtv: d3d_rtv })
    }

    pub fn prepare(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
        unsafe {
            // set render target
            d3d_context.OMSetRenderTargets(1, [self.d3d_rtv].as_ptr(), ptr::null_mut());

            // viewport
            let viewport = d3d11::D3D11_VIEWPORT {
                Width: 1280.0,
                Height: 720.0,
                ..Default::default()
            };
            d3d_context.RSSetViewports(1, &viewport);

            // clear
            d3d_context.ClearRenderTargetView(self.d3d_rtv, &[0.0, 0.2, 0.4, 1.0]);
        }
    }
}

