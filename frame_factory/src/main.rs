extern crate gen;

use gen::sdl;
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

macro_rules! T {
    ($x:literal) => {
        if $x.ends_with("\0") {
            $x.as_ptr() as *mut i8
        } else {
            concat!($x, "\0").as_ptr() as *mut i8
        }
    };
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

        // let window = video_subsystem
        //     .window("rust-sdl2 demo: Video", 800, 600)
        //     .position_centered()
        //     .resizable()
        //     .allow_highdpi()
        //     .build()
        //     .map_err(|e| e.to_string())?;

        // let mut wm_info = [0 as u8; std::mem::size_of::<SDL_SysWMinfo>()];
        // let p = wm_info.as_mut_ptr() as *mut SDL_SysWMinfo;
        // SDL_GetWindowWMInfo(window.raw(), p);
        // let info = p.as_mut().unwrap().info;
        // let array: [u8; 8] = info.dummy[0..8].try_into().unwrap();
        // let addr = u64::from_le_bytes(array);

        // let mut device = Device::create(addr as HWND).unwrap();

        // imgui_raw::CreateContext(ptr::null_mut());
        // let mut io = imgui_raw::GetIO().as_mut().unwrap();
        // io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::NavEnableKeyboard as i32; // Enable Keyboard Controls
        //                                                                           //io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
        // io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::DockingEnable as i32; // Enable Docking
        // io.ConfigFlags |= imgui_raw::ImGuiConfigFlags_::ViewportsEnable as i32; // Enable Multi-Viewport / Platform Windows
        //                                                                         //io.ConfigViewportsNoAutoMerge = true;
        //                                                                         //io.ConfigViewportsNoTaskBarIcon = true;

        // // Setup Dear ImGui style
        // imgui_raw::StyleColorsDark(ptr::null_mut());
        // //imgui_raw::StyleColorsClassic();

        // // When viewports are enabled we tweak WindowRounding/WindowBg so platform windows can look identical to regular ones.
        // let style = imgui_raw::GetStyle().as_mut().unwrap();
        // if (io.ConfigFlags & imgui_raw::ImGuiConfigFlags_::ViewportsEnable as i32) != 0 {
        //     style.WindowRounding = 0.0f32;
        //     style.Colors[imgui_raw::ImGuiCol_::WindowBg as usize].w = 1.0f32;
        // }

        // // Setup Platform/Renderer backends
        // imgui_raw::ImGui_ImplSDL2_InitForD3D(window.raw() as *mut c_void);
        // imgui_raw::ImGui_ImplDX11_Init(device.device as *mut c_void, device.context as *mut c_void);

        // // Load Fonts
        // // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use imgui_raw::PushFont()/PopFont() to select them.
        // // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
        // // - If the file cannot be loaded, the function will return NULL. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
        // // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::Build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
        // // - Read 'docs/FONTS.md' for more instructions and details.
        // // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
        // //io.Fonts->AddFontDefault();
        // //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0f);
        // //io.Fonts->AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0f);
        // //io.Fonts->AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0f);
        // //io.Fonts->AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0f);
        // //ImFont* font = io.Fonts->AddFontFromFileTTF("c:\\Windows\\Fonts\\ArialUni.ttf", 18.0f, NULL, io.Fonts->GetGlyphRangesJapanese());
        // //IM_ASSERT(font != NULL);

        // // Our state
        // let mut show_demo_window = true;
        // let mut show_another_window = false;
        // let mut clear_color = [
        //     0.45f32,
        //     0.55f32,
        //     0.60f32,
        //     1.00f32,
        // ];
        // let mut f = 0.0f32;
        // let mut counter = 0;

        // // Main loop
        // let mut done = false;
        // while !done {
        //     let mut event = [0 as u8; std::mem::size_of::<SDL_Event>()];
        //     let p_event = event.as_mut_ptr() as *mut SDL_Event;
        //     while _SDL_PollEvent(p_event as *mut c_void) != 0 {
        //         imgui_raw::ImGui_ImplSDL2_ProcessEvent(p_event as *mut c_void);

        //         if let Some(event) = p_event.as_ref() {
        //             match event.type_ {
        //                 256 /* SDL_QUIT */ => {
        //                     done = true;
        //                 },
        //                 512 /* SDL_WINDOWEVENT */ => {
        //                     if event.window.event == 14 /* SDL_WINDOWEVENT_CLOSE */ && event.window.windowID == window.id()
        //                     {
        //                         done = true;
        //                     }
        //                     if event.window.event == 14 /* SDL_WINDOWEVENT_RESIZED */ && event.window.windowID == window.id()
        //                     {
        //                         device.resize();
        //                     }
        //                 }
        //                 _ =>{

        //                 }
        //             }
        //         }
        //     }

        //     imgui_raw::ImGui_ImplDX11_NewFrame();
        //     imgui_raw::ImGui_ImplSDL2_NewFrame(window.raw() as *mut c_void);
        //     imgui_raw::NewFrame();

        //     // 1. Show the big demo window (Most of the sample code is in imgui_raw::ShowDemoWindow()! You can browse its code to learn more about Dear ImGui!).
        //     if show_demo_window{
        //         imgui_raw::ShowDemoWindow(&mut show_demo_window);
        //     }

        //     // 2. Show a simple window that we create ourselves. We use a Begin/End pair to created a named window.
        //     {
        //         imgui_raw::Begin("Hello, world!\0".as_ptr() as *const i8, ptr::null_mut(), 0);                          // Create a window called "Hello, world!" and append into it.

        //         imgui_raw::Text("This is some useful text.".as_ptr() as *const i8);               // Display some text (you can use a format strings too)
        //         imgui_raw::Checkbox("Demo Window".as_ptr() as *const i8, &mut show_demo_window);      // Edit bools storing our window open/close state
        //         imgui_raw::Checkbox("Another Window".as_ptr() as *const i8, &mut show_another_window);

        //         imgui_raw::SliderFloat("float".as_ptr() as *const i8, &mut f, 0.0f32, 1.0f32, "%.3f".as_ptr() as *const i8, 0);            // Edit 1 float using a slider from 0.0f to 1.0f
        //         let clear_color: [f32; 3] = clear_color[0..3].try_into().unwrap();
        //         imgui_raw::ColorEdit3("clear color".as_ptr() as *const i8, clear_color, 0); // Edit 3 floats representing a color

        //         if imgui_raw::Button("Button".as_ptr() as *const i8, &imgui_raw::ImVec2{x: 0f32, y:0f32})                            // Buttons return true when clicked (most widgets return true when edited/activated)
        //         {
        //             counter+=1;
        //         }
        //         imgui_raw::SameLine(0f32, -1f32);
        //         // imgui_raw::TextV("counter = %d".as_ptr() as *const i8, counter);

        //         // imgui_raw::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / imgui_raw::GetIO().Framerate, imgui_raw::GetIO().Framerate);
        //         imgui_raw::End();
        //     }

        //     // 3. Show another simple window.
        //     if show_another_window
        //     {
        //         imgui_raw::Begin("Another Window".as_ptr() as *const i8, &mut show_another_window, 0);   // Pass a pointer to our bool variable (the window will have a closing button that will clear the bool when clicked)
        //         imgui_raw::Text("Hello from another window!".as_ptr() as *const i8);
        //         if imgui_raw::Button("Close Me".as_ptr() as *const i8, &imgui_raw::ImVec2{x: 0f32, y:0f32})
        //         {
        //             show_another_window = false;
        //         }
        //         imgui_raw::End();
        //     }

        //     // Rendering
        //     imgui_raw::Render();
        //     let clear_color_with_alpha = [ clear_color[0] * clear_color[3], clear_color[1] * clear_color[3], clear_color[2] * clear_color[3], clear_color[3] ];
        //     device.clear(&clear_color_with_alpha);

        //     imgui_raw::ImGui_ImplDX11_RenderDrawData(imgui_raw::GetDrawData());

        //     // Update and Render additional Platform Windows
        //     if (io.ConfigFlags & imgui_raw::ImGuiConfigFlags_::ViewportsEnable as i32)!=0
        //     {
        //         imgui_raw::UpdatePlatformWindows();
        //         imgui_raw::RenderPlatformWindowsDefault(ptr::null_mut(), ptr::null_mut());
        //     }

        //     device.present();
        // }

        // // Cleanup
        // imgui_raw::ImGui_ImplDX11_Shutdown();
        // imgui_raw::ImGui_ImplSDL2_Shutdown();
        // imgui_raw::DestroyContext(ptr::null_mut());

        Ok(())
    }
}
