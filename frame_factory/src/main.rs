extern crate gen;
use com_ptr::ComPtr;
extern crate com_ptr_util;
use com_ptr_util::ComCreate;
use frame_factory::FRAME_FACTORY_asset_path;
use gen::sdl;

macro_rules! T {
    ($x:literal) => {
        if $x.ends_with("\0") {
            $x.as_ptr() as *mut i8
        } else {
            concat!($x, "\0").as_ptr() as *mut i8
        }
    };
}

mod gui;
use gui::*;
mod scene_view;


use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;

use winapi::shared::dxgi;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::windef::HWND;
use winapi::um::d3d11::{self};
use winapi::um::d3dcommon;
use winapi::Interface;

struct Device {
    pub device: ComPtr<d3d11::ID3D11Device>,
    pub context: ComPtr<d3d11::ID3D11DeviceContext>,
    pub swapchain: ComPtr<dxgi::IDXGISwapChain>,
    pub rendertarget: Option<ComPtr<d3d11::ID3D11RenderTargetView>>,
}

#[allow(dead_code)]
impl Device {
    fn create(hwnd: HWND) -> Result<Device, i32> {
        // Setup swap chain
        let sd = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferCount: 2,
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: 0,
                Height: 0,
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
                RefreshRate: dxgitype::DXGI_RATIONAL {
                    Numerator: 60,
                    Denominator: 1,
                },
                ..Default::default()
            },
            Flags: dxgi::DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH,
            BufferUsage: dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            OutputWindow: hwnd,
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Windowed: 1,
            SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
        };

        let create_device_flags: u32 = 0;
        // createDeviceFlags |= d3d11::D3D11_CREATE_DEVICE_DEBUG;

        let mut feature_level = d3dcommon::D3D_FEATURE_LEVEL::default();
        let feature_level_array = [
            d3dcommon::D3D_FEATURE_LEVEL_11_0,
            d3dcommon::D3D_FEATURE_LEVEL_10_0,
        ];

        let mut device: *mut d3d11::ID3D11Device = ptr::null_mut();
        let mut context: *mut d3d11::ID3D11DeviceContext = ptr::null_mut();
        let mut swapchain: *mut dxgi::IDXGISwapChain = ptr::null_mut();
        let hr = unsafe {
            d3d11::D3D11CreateDeviceAndSwapChain(
                ptr::null_mut(),
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                ptr::null_mut(),
                create_device_flags,
                feature_level_array.as_ptr(),
                feature_level_array.len() as u32,
                d3d11::D3D11_SDK_VERSION,
                &sd,
                &mut swapchain,
                &mut device,
                &mut feature_level,
                &mut context,
            )
        };
        if hr != 0 {
            return Err(hr);
        }

        let mut device = Device {
            device: unsafe { ComPtr::from_raw(device) },
            context: unsafe { ComPtr::from_raw(context) },
            swapchain: unsafe { ComPtr::from_raw(swapchain) },
            rendertarget: None,
        };

        device.create_render_target().unwrap();

        Ok(device)
    }

    fn create_render_target(&mut self) -> Result<(), com_ptr_util::ComError> {
        let backbuffer: ComPtr<d3d11::ID3D11Texture2D> = ComPtr::create_if_success(|pp| unsafe {
            self.swapchain.GetBuffer(
                0,
                &d3d11::ID3D11Texture2D::uuidof(),
                pp as *mut *mut winapi::ctypes::c_void,
            )
        })?;

        let rendertarget = ComPtr::create_if_success(|pp| unsafe {
            self.device.CreateRenderTargetView(
                backbuffer.as_ptr() as *mut d3d11::ID3D11Resource,
                ptr::null(),
                pp,
            )
        })?;

        self.rendertarget = Some(rendertarget);

        Ok(())
    }

    fn resize(&mut self) {
        // Release all outstanding references to the swap chain's buffers before resizing.
        self.rendertarget = None;
        unsafe {
            self.swapchain
                .ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0)
        };
        self.create_render_target().unwrap();
    }

    fn clear(&mut self, clear_color_with_alpha: &[f32; 4]) {
        if let Some(rendertarget) = &self.rendertarget {
            unsafe {
                self.context
                    .OMSetRenderTargets(1, &rendertarget.as_ptr(), ptr::null_mut())
            };
            unsafe {
                self.context
                    .ClearRenderTargetView(rendertarget.as_ptr(), clear_color_with_alpha)
            };
        }
    }

    fn present(&mut self) {
        // Present with vsync
        unsafe { self.swapchain.Present(1, 0) };
    }
}

#[link(name = "sdl2", kind = "static")]
extern "C" {
    #[link_name = "SDL_PollEvent"]
    fn _SDL_PollEvent(event: *mut c_void) -> i32;
}

pub fn main() -> Result<(), String> {
    unsafe {
        if sdl::SDL_Init(sdl::SDL_INIT_VIDEO | sdl::SDL_INIT_TIMER | sdl::SDL_INIT_GAMECONTROLLER)
            != 0
        {
            panic!();
            // panic!("Error: %s\n", SDL_GetError());
        }

        let window_flags = sdl::SDL_WINDOW_RESIZABLE | sdl::SDL_WINDOW_ALLOW_HIGHDPI;
        let window = sdl::SDL_CreateWindow(
            T!("Dear ImGui SDL2+DirectX11 example"),
            sdl::SDL_WINDOWPOS_CENTERED as i32,
            sdl::SDL_WINDOWPOS_CENTERED as i32,
            1280,
            720,
            window_flags as u32,
        );

        let mut wm_info = MaybeUninit::<sdl::SDL_SysWMinfo>::zeroed().assume_init();
        // sdl::SDL_VERSION(&wmInfo.version);
        sdl::SDL_GetWindowWMInfo_REAL(window, &mut wm_info);
        let addr = wm_info.info.win.window as *mut c_void;

        let mut device = Device::create(addr as HWND).unwrap();

        let args: Vec<String> = std::env::args().collect();
        let assets = format!("{}\0", args[1]);
        FRAME_FACTORY_asset_path(assets.as_ptr() as *const i8);
        let mut gui = Gui::new(window, &device.device, &device.context);

        // Main loop
        let mut done = false;
        while !done {
            // SDL event
            let mut event: sdl::SDL_Event = MaybeUninit::zeroed().assume_init();
            let p = &mut event as *mut sdl::SDL_Event;
            while sdl::SDL_PollEvent(p) != 0 {
                gui.sdl_event(p);

                match event.r#type {
                                256 /* SDL_QUIT */ => {
                                    done = true;
                                },
                                512 /* SDL_WINDOWEVENT */ => {
                                    if event.window.event == sdl::SDL_WINDOWEVENT_CLOSE as u8 && event.window.windowID == sdl::SDL_GetWindowID(window)
                                    {
                                        done = true;
                                    }
                                    if event.window.event == sdl::SDL_WINDOWEVENT_RESIZED as u8 && event.window.windowID == sdl::SDL_GetWindowID(window)
                                    {
                                        device.resize();
                                    }
                                }
                                _ =>{

                                }
                            }
            }

            // gui
            gui.update(window, &device.device, &device.context);

            // d3d
            let clear_color_with_alpha = [
                gui.clear_color[0] * gui.clear_color[3],
                gui.clear_color[1] * gui.clear_color[3],
                gui.clear_color[2] * gui.clear_color[3],
                gui.clear_color[3],
            ];
            device.clear(&clear_color_with_alpha);

            gui.render();

            device.present();
        }

        Ok(())
    }
}
