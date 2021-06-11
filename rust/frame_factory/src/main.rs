extern crate imgui_raw;
// extern crate sdl2;

use sdl2::sys::SDL_GetWindowWMInfo;
use sdl2::sys::SDL_SysWMinfo;
use std::convert::TryInto;
use std::ffi::c_void;
use std::ptr;
use winapi::Interface;

use winapi::shared::dxgi;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::windef::HWND;
use winapi::um::d3d11;
use winapi::um::d3dcommon;

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
}

pub fn main() -> Result<(), String> {
    unsafe {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 800, 600)
            .position_centered()
            .resizable()
            .allow_highdpi()
            .build()
            .map_err(|e| e.to_string())?;

        let mut wm_info = [0 as u8; std::mem::size_of::<SDL_SysWMinfo>()];
        let p = wm_info.as_mut_ptr() as *mut SDL_SysWMinfo;
        SDL_GetWindowWMInfo(window.raw(), p);
        let info = p.as_mut().unwrap().info;
        let array: [u8; 8] = info.dummy[0..8].try_into().unwrap();
        let addr = u64::from_le_bytes(array);

        let device = Device::create(addr as HWND).unwrap();

        imgui_raw::CreateContext(ptr::null_mut());
        let mut io = imgui_raw::GetIO().as_mut().unwrap();
        io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::NavEnableKeyboard as i32; // Enable Keyboard Controls
                                                                                  //io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
        io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::DockingEnable as i32; // Enable Docking
        io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::ViewportsEnable as i32; // Enable Multi-Viewport / Platform Windows
                                                                                //io.ConfigViewportsNoAutoMerge = true;
                                                                                //io.ConfigViewportsNoTaskBarIcon = true;

        // Setup Dear ImGui style
        imgui_raw::StyleColorsDark(ptr::null_mut());
        //imgui_raw::StyleColorsClassic();

        // When viewports are enabled we tweak WindowRounding/WindowBg so platform windows can look identical to regular ones.
        let style = imgui_raw::GetStyle().as_mut().unwrap();
        if (io.ConfigFlags & imgui_raw::ImGuiConfigFlags_::ViewportsEnable as i32) != 0 {
            style.WindowRounding = 0.0f32;
            style.Colors[imgui_raw::ImGuiCol_::WindowBg as usize].w = 1.0f32;
        }

        // Setup Platform/Renderer backends
        imgui_raw::ImGui_ImplSDL2_InitForD3D(window.raw() as *mut c_void);
        // imgui_raw::ImGui_ImplDX11_Init(g_pd3dDevice, g_pd3dDeviceContext);

        // Load Fonts
        // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use imgui_raw::PushFont()/PopFont() to select them.
        // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
        // - If the file cannot be loaded, the function will return NULL. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
        // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::Build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
        // - Read 'docs/FONTS.md' for more instructions and details.
        // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
        //io.Fonts->AddFontDefault();
        //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0f);
        //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0f);
        //io.Fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0f);
        //io.Fonts->AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0f);
        //ImFont* font = io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\ArialUni.ttf", 18.0f, NULL, io.Fonts->GetGlyphRangesJapanese());
        //IM_ASSERT(font != NULL);

        // Our state
        let show_demo_window = true;
        let show_another_window = false;
        let clear_color = imgui_raw::ImVec4 {
            x: 0.45f32,
            y: 0.55f32,
            z: 0.60f32,
            w: 1.00f32,
        };

        //
        // main loop
        //

        // Cleanup
        imgui_raw::ImGui_ImplDX11_Shutdown();
        imgui_raw::ImGui_ImplSDL2_Shutdown();
        imgui_raw::DestroyContext(ptr::null_mut());

        Ok(())
    }
}
