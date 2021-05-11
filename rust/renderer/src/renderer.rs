use std::ptr;

use com_ptr::ComPtr;
use winapi::{
    shared::{dxgi, dxgiformat, dxgitype, minwindef, windef::HWND},
    um::{d3d11, d3dcommon},
};

pub struct Renderer {
    pub d3d_device: ComPtr<d3d11::ID3D11Device>,
    pub d3d_context: ComPtr<d3d11::ID3D11DeviceContext>,
    pub dxgi_swapchain: ComPtr<dxgi::IDXGISwapChain>,
}

impl Renderer {
    pub fn new(hwnd: HWND) -> Result<Renderer, &'static str> {
        // let mut usage = dxgi::Usage::default();
        // usage.set_dxgi_usage(dxgi::USAGE_RENDER_TARGET_OUTPUT);
        let swapchain_desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: 1280,
                Height: 720,
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                ..Default::default()
            },
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferCount: 1,
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: hwnd,
            Windowed: minwindef::TRUE,
            ..Default::default()
        };

        let feature_levels = [d3dcommon::D3D_FEATURE_LEVEL_11_0];
        let mut actual_feature_level = feature_levels[0];

        let mut d3d_device: *mut d3d11::ID3D11Device = ptr::null_mut();
        let mut d3d_context: *mut d3d11::ID3D11DeviceContext = ptr::null_mut();
        let mut dxgi_swapchain: *mut dxgi::IDXGISwapChain = ptr::null_mut();

        unsafe {
            let result = d3d11::D3D11CreateDeviceAndSwapChain(
                ptr::null_mut(),
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                ptr::null_mut(),
                d3d11::D3D11_CREATE_DEVICE_DEBUG,
                feature_levels.as_ptr(),
                feature_levels.len() as u32,
                d3d11::D3D11_SDK_VERSION,
                &swapchain_desc,
                &mut dxgi_swapchain,
                &mut d3d_device,
                &mut actual_feature_level,
                &mut d3d_context,
            );
            if result != 0 {
                return Err("D3D11CreateDeviceAndSwapChain");
            }
            Ok(Renderer {
                d3d_device: ComPtr::from_raw(d3d_device),
                d3d_context: ComPtr::from_raw(d3d_context),
                dxgi_swapchain: ComPtr::from_raw(dxgi_swapchain),
            })
        }
    }

    pub fn present(&self) {
        unsafe {
            self.d3d_context.OMSetRenderTargets(0, ptr::null(), ptr::null_mut());
            self.dxgi_swapchain.Present(0, 0);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        println!("Renderer.drop");
    }
}
