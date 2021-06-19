use std::ptr;

use com_ptr::ComPtr;
use winapi::{
    Interface,
    shared::{dxgi, dxgiformat, dxgitype, minwindef, windef::HWND},
    um::{d3d11, d3dcommon},
};

use com_ptr_util::{ComCreate, ComError};

pub struct DX11 {
    pub d3d_device: ComPtr<d3d11::ID3D11Device>,
    pub d3d_context: ComPtr<d3d11::ID3D11DeviceContext>,
    pub dxgi_swapchain: ComPtr<dxgi::IDXGISwapChain>,
    pub back_buffer: Option<ComPtr<d3d11::ID3D11RenderTargetView>>,
}

#[allow(dead_code)]
impl DX11 {
    pub fn new(hwnd: HWND) -> Result<DX11, &'static str> {
        let swapchain_desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                ..Default::default()
            },
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferCount: 2,
            SwapEffect: dxgi::DXGI_SWAP_EFFECT_FLIP_DISCARD,
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
            Ok(DX11 {
                d3d_device: ComPtr::from_raw(d3d_device),
                d3d_context: ComPtr::from_raw(d3d_context),
                dxgi_swapchain: ComPtr::from_raw(dxgi_swapchain),
                back_buffer: None,
            })
        }
    }

    pub fn get_or_create_backbuffer(
        &mut self,
        width: u32,
        height: u32,
    ) -> Result<ComPtr<d3d11::ID3D11RenderTargetView>, ComError> {
        if let Some(rtv) = &self.back_buffer {
            let mut desc = dxgi::DXGI_SWAP_CHAIN_DESC::default();
            unsafe { self.dxgi_swapchain.GetDesc(&mut desc) };
            if desc.BufferDesc.Width == width && desc.BufferDesc.Height == height {
                // already created
                return Ok(rtv.clone());
            }
        }

        // clear
        self.back_buffer = None;

        // resize
        unsafe {
            self.dxgi_swapchain
                .ResizeBuffers(0, width, height, dxgiformat::DXGI_FORMAT_UNKNOWN, 0)
        };

        let d3d_back_buffer: ComPtr<d3d11::ID3D11Resource> =
            ComPtr::create_if_success(|pp| unsafe {
                self.dxgi_swapchain.GetBuffer(
                    0,
                    &d3d11::ID3D11Texture2D::uuidof(),
                    pp as *mut *mut winapi::ctypes::c_void,
                )
            })?;

        let d3d_rtv: ComPtr<d3d11::ID3D11RenderTargetView> =
            ComPtr::create_if_success(|pp| unsafe {
                self.d3d_device
                    .CreateRenderTargetView(d3d_back_buffer.as_ptr(), ptr::null(), pp)
            })?;

        let clone = d3d_rtv.clone();

        self.back_buffer = Some(d3d_rtv);

        Ok(clone)
    }

    pub fn present(&self) {
        unsafe {
            self.d3d_context
                .OMSetRenderTargets(0, ptr::null(), ptr::null_mut());
            self.dxgi_swapchain.Present(0, 0);
        }
    }
}

impl Drop for DX11 {
    fn drop(&mut self) {
        println!("Renderer.drop");
    }
}
