extern crate com_rs;
extern crate directx_sys;
extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::{error, ffi::OsStr};

use com_rs::ComPtr;
use directx_sys::{d3d11, dxgi};
use kernel32::*;
use user32::*;

use winapi::{
    CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, HWND, LPARAM, LPCWSTR, LRESULT, MSG,
    PM_REMOVE, SW_SHOW, TRUE, UINT, WM_DESTROY, WM_QUIT, WNDCLASSEXW, WPARAM, WS_EX_APPWINDOW,
    WS_OVERLAPPEDWINDOW,
};

fn utf16_str(s: &str) -> Vec<u16> {
    OsStr::new(s)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

unsafe extern "system" fn wndproc(
    window: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
            return 0;
        }
        _ => DefWindowProcW(window, msg, wparam, lparam),
    }
}

fn create_window(class_name: &str, window_name: &str) -> Result<HWND, &'static str> {
    unsafe {
        let class_name = utf16_str(class_name);
        let wcex = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW | CS_OWNDC,
            lpfnWndProc: Some(wndproc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(ptr::null()),
            hbrBackground: ptr::null_mut(),
            lpszMenuName: ptr::null_mut(),
            hIcon: ptr::null_mut(),
            hCursor: ptr::null_mut(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: ptr::null_mut(),
        };
        if RegisterClassExW(&wcex) == 0 {
            return Err("RegisterClassExWS");
        }

        let window_title = utf16_str(window_name);
        let hwnd = CreateWindowExW(
            WS_EX_APPWINDOW,
            class_name.as_ptr(),
            window_title.as_ptr() as LPCWSTR,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            1280,
            720,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleW(ptr::null()),
            ptr::null_mut(),
        );
        if hwnd.is_null() {
            return Err("CreateWindowExW");
        }

        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
        Ok(hwnd)
    }
}

struct Renderer {
    d3d_device: ComPtr<d3d11::ID3D11Device>,
    d3d_context: ComPtr<d3d11::ID3D11DeviceContext>,
    dxgi_swapchain: ComPtr<dxgi::IDXGISwapChain>,
}

impl Renderer {
    fn new(hwnd: HWND) -> Result<Renderer, &'static str> {
        let mut d3d_device: ComPtr<d3d11::ID3D11Device> = ComPtr::new();
        let mut d3d_context: ComPtr<d3d11::ID3D11DeviceContext> = ComPtr::new();
        let mut dxgi_swapchain: ComPtr<dxgi::IDXGISwapChain> = ComPtr::new();

        let mut usage = dxgi::Usage::default();
        usage.set_dxgi_usage(dxgi::USAGE_RENDER_TARGET_OUTPUT);
        let swapchain_desc = dxgi::SwapChainDesc {
            buffer_desc: dxgi::ModeDesc {
                width: 1280,
                height: 720,
                format: dxgi::Format::R8G8B8A8Unorm,
                ..Default::default()
            },
            sample_desc: dxgi::SampleDesc {
                count: 1,
                quality: 0,
            },
            buffer_count: 1,
            buffer_usage: usage,
            output_window: hwnd,
            windowed: TRUE,
            ..Default::default()
        };

        let feature_levels = [d3d11::FeatureLevel::Level_11_0];
        let mut actual_feature_level: d3d11::FeatureLevel = feature_levels[0];
        let result = unsafe {
            d3d11::D3D11CreateDeviceAndSwapChain(
                ptr::null(),
                d3d11::DriverType::Hardware,
                ptr::null_mut(),
                d3d11::CREATE_DEVICE_DEBUG,
                feature_levels.as_ptr(),
                feature_levels.len() as u32,
                d3d11::SDK_VERSION,
                &swapchain_desc,
                dxgi_swapchain.as_mut_ptr(),
                d3d_device.as_mut_ptr(),
                &mut actual_feature_level,
                d3d_context.as_mut_ptr(),
            )
        };
        if result != 0 {
            return Err("D3D11CreateDeviceAndSwapChain");
        }
        assert!(!dxgi_swapchain.is_null());
        assert!(!d3d_device.is_null());
        assert!(!d3d_context.is_null());

        Ok(Renderer {
            dxgi_swapchain,
            d3d_device,
            d3d_context,
        })
    }

    fn present(&self) {
        unsafe {
            self.dxgi_swapchain
                .present(0, dxgi::PresentFlags::default());
        }
    }
}

struct RenderTarget {
    d3d_rtv: ComPtr<d3d11::ID3D11RenderTargetView>,
}

impl RenderTarget {
    fn from_swapchain(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        dxgi_swapchain: &ComPtr<dxgi::IDXGISwapChain>,
    ) -> Result<RenderTarget, &'static str> {
        let mut d3d_back_buffer: ComPtr<d3d11::ID3D11Texture2D> = ComPtr::new();
        let result = unsafe {
            dxgi_swapchain.get_buffer(0, &d3d_back_buffer.iid(), d3d_back_buffer.as_mut_ptr())
        };
        if result != 0 {
            return Err("ID3D11Texture2D::get_buffer");
        }
        assert!(!d3d_back_buffer.is_null());

        let mut d3d_rtv: ComPtr<d3d11::ID3D11RenderTargetView> = ComPtr::new();
        let result = unsafe {
            d3d_device.create_render_target_view(
                d3d_back_buffer.as_ptr(),
                ptr::null(),
                d3d_rtv.as_mut_ptr(),
            )
        };
        if result != 0 {
            return Err("create_render_target_view");
        }
        assert!(!d3d_rtv.is_null());

        Ok(RenderTarget { d3d_rtv })
    }

    fn prepare(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
        unsafe {
            // set render target
            d3d_context.om_set_render_targets(1, &self.d3d_rtv.as_ptr(), ptr::null());

            // viewport
            let viewport = d3d11::Viewport {
                width: 1280.0,
                height: 720.0,
                ..Default::default()
            };
            d3d_context.rs_set_viewports(1, &viewport);

            // clear
            d3d_context.clear_render_target_view(self.d3d_rtv.as_ptr(), &[0.0, 0.2, 0.4, 1.0]);
        }
    }
}

struct FrameIter {
    hwnd: HWND,
}

impl FrameIter {
    fn new(hwnd: HWND) -> FrameIter {
        FrameIter { hwnd }
    }
}

impl Iterator for FrameIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: MSG = mem::zeroed();
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > 0 {
                if msg.message == WM_QUIT {
                    return None;
                };
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }

        Some(())
    }
}

fn run() -> Result<(), Box<dyn error::Error>> {
    let hwnd = create_window("WindowClass", "D3D11 Demo")?;
    let renderer = Renderer::new(hwnd)?;
    let render_target =
        RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain)?;
    for _ in FrameIter::new(hwnd) {
        render_target.prepare(&renderer.d3d_context);
        renderer.present();
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
