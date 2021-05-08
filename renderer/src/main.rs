extern crate com_ptr;
extern crate winapi;

use std::mem;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use std::{error, ffi::OsStr};

use com_ptr::ComPtr;
use winapi::Interface;
use winapi::{
    shared::minwindef::*,
    um::{libloaderapi, winuser},
};
use winapi::{
    shared::{dxgi, dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, dxgitype, windef::*},
    um::{d3d11, d3dcommon},
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
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(0);
            0
        }
        _ => winuser::DefWindowProcW(window, msg, wparam, lparam),
    }
}

struct Window {
    hwnd: HWND,
}

impl Window {
    fn new(class_name: &str, window_name: &str) -> Result<Window, &'static str> {
        unsafe {
            let class_name = utf16_str(class_name);
            let wcex = winuser::WNDCLASSEXW {
                cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as u32,
                style: winuser::CS_HREDRAW | winuser::CS_VREDRAW | winuser::CS_OWNDC,
                lpfnWndProc: Some(wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
                hbrBackground: ptr::null_mut(),
                lpszMenuName: ptr::null_mut(),
                hIcon: ptr::null_mut(),
                hCursor: ptr::null_mut(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: ptr::null_mut(),
            };
            if winuser::RegisterClassExW(&wcex) == 0 {
                return Err("RegisterClassExWS");
            }

            let window_name = utf16_str(window_name);
            let hwnd = winuser::CreateWindowExW(
                winuser::WS_EX_APPWINDOW,
                class_name.as_ptr(),
                window_name.as_ptr(),
                winuser::WS_OVERLAPPEDWINDOW,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                1280,
                720,
                ptr::null_mut(),
                ptr::null_mut(),
                libloaderapi::GetModuleHandleW(ptr::null()),
                ptr::null_mut(),
            );
            if hwnd.is_null() {
                return Err("CreateWindowExW");
            }

            winuser::ShowWindow(hwnd, winuser::SW_SHOW);
            winuser::UpdateWindow(hwnd);
            Ok(Window { hwnd })
        }
    }
}

impl Iterator for Window {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut msg: winuser::MSG = mem::zeroed();
            while winuser::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) > 0 {
                if msg.message == winuser::WM_QUIT {
                    return None;
                };
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageW(&msg);
            }
        }

        Some(())
    }
}

struct Renderer {
    d3d_device: ComPtr<d3d11::ID3D11Device>,
    d3d_context: ComPtr<d3d11::ID3D11DeviceContext>,
    dxgi_swapchain: ComPtr<dxgi::IDXGISwapChain>,
}

impl Renderer {
    fn new(window: &Window) -> Result<Renderer, &'static str> {
        // let mut usage = dxgi::Usage::default();
        // usage.set_dxgi_usage(dxgi::USAGE_RENDER_TARGET_OUTPUT);
        let swapchain_desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: 1280,
                Height: 720,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                ..Default::default()
            },
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferCount: 1,
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: window.hwnd,
            Windowed: TRUE,
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

    fn present(&self) {
        unsafe {
            self.dxgi_swapchain.Present(0, 0);
        }
    }
}

struct RenderTarget {
    d3d_rtv: *mut d3d11::ID3D11RenderTargetView,
}

impl RenderTarget {
    fn from_swapchain(
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

        unsafe { Ok(RenderTarget { d3d_rtv: d3d_rtv }) }
    }

    fn prepare(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
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

fn run() -> Result<(), Box<dyn error::Error>> {
    let window = Window::new("WindowClass", "D3D11 Demo")?;
    let renderer = Renderer::new(&window)?;
    let render_target =
        RenderTarget::from_swapchain(&renderer.d3d_device, &renderer.dxgi_swapchain)?;
    for _ in window {
        render_target.prepare(&renderer.d3d_context);
        renderer.present();
    }
    Ok(())
}

fn main() {
    run().unwrap();
}
