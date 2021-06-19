extern crate gen;
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
use scene_view::*;

use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr;

use winapi::shared::dxgi;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::windef::HWND;
use winapi::um::d3d11;
use winapi::um::d3dcommon;
use winapi::Interface;

struct Device {
    pub device: *mut d3d11::ID3D11Device,
    pub context: *mut d3d11::ID3D11DeviceContext,
    pub swapchain: *mut dxgi::IDXGISwapChain,
    pub rendertarget: *mut d3d11::ID3D11RenderTargetView,
}

impl Drop for Device {
    fn drop(&mut self) {
        self.cleanup_render_target();
        unsafe {
            if let Some(swapchain) = self.swapchain.as_ref() {
                swapchain.Release();
                self.swapchain = ptr::null_mut();
            }
            if let Some(context) = self.context.as_ref() {
                context.Release();
                self.context = ptr::null_mut();
            }
            if let Some(device) = self.device.as_ref() {
                device.Release();
                self.device = ptr::null_mut();
            }
        }
    }
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

        let mut device = Device {
            device: ptr::null_mut(),
            context: ptr::null_mut(),
            swapchain: ptr::null_mut(),
            rendertarget: ptr::null_mut(),
        };

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
                &mut device.swapchain,
                &mut device.device,
                &mut feature_level,
                &mut device.context,
            )
        };
        if hr != 0 {
            return Err(hr);
        }

        device.create_render_target();

        Ok(device)
    }

    fn create_render_target(&mut self) {
        let mut back_buffer: *mut d3d11::ID3D11Texture2D = ptr::null_mut();
        let pp = &mut back_buffer as *mut *mut d3d11::ID3D11Texture2D;

        if let Some(swap_chain) = unsafe { self.swapchain.as_ref() } {
            unsafe {
                swap_chain.GetBuffer(
                    0,
                    &d3d11::ID3D11Texture2D::uuidof(),
                    pp as *mut *mut winapi::ctypes::c_void,
                )
            };
            if let Some(device) = unsafe { self.device.as_ref() } {
                unsafe {
                    device.CreateRenderTargetView(
                        back_buffer as *mut d3d11::ID3D11Resource,
                        ptr::null(),
                        &mut self.rendertarget,
                    )
                };
                if let Some(backbuffer) = unsafe { back_buffer.as_ref() } {
                    unsafe { backbuffer.Release() };
                }
            }
        }
    }

    fn cleanup_render_target(&mut self) {
        if let Some(render_target) = unsafe { self.rendertarget.as_ref() } {
            unsafe { render_target.Release() };
            self.rendertarget = ptr::null_mut();
        }
    }

    fn resize(&mut self) {
        // Release all outstanding references to the swap chain's buffers before resizing.
        self.cleanup_render_target();
        if let Some(swapchain) = unsafe { self.swapchain.as_mut() } {
            unsafe { swapchain.ResizeBuffers(0, 0, 0, dxgiformat::DXGI_FORMAT_UNKNOWN, 0) };
        }
        self.create_render_target();
    }

    fn clear(&mut self, clear_color_with_alpha: &[f32; 4]) {
        if let Some(context) = unsafe { self.context.as_mut() } {
            unsafe { context.OMSetRenderTargets(1, &mut self.rendertarget, ptr::null_mut()) };
            unsafe { context.ClearRenderTargetView(self.rendertarget, clear_color_with_alpha) };
        }
    }

    fn present(&mut self) {
        if let Some(swapchain) = unsafe { self.swapchain.as_mut() } {
            unsafe { swapchain.Present(1, 0) }; // Present with vsync
        }
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

        let mut gui = Gui::new(
            window,
            device.device as *mut c_void,
            device.context as *mut c_void,
        );

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
            gui.update(
                window,
                device.device as *mut c_void,
                device.context as *mut c_void,
            );

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
