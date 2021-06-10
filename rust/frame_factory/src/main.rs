extern crate imgui_raw;
extern crate sdl2;

use sdl2::sys::*;
use std::ffi::c_void;
use std::ptr;
use std::time::Duration;

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

    let mut wmInfo: SDL_SysWMinfo = unsafe { std::mem::uninitialized() };
    unsafe { SDL_GetWindowWMInfo(window.raw(), &mut wmInfo) };
    let hwnd = unsafe { ptr::addr_of_mut!(wmInfo.info.dummy) } as *mut c_void;

    // Initialize Direct3D
    if !CreateDeviceD3D(hwnd) {
        CleanupDeviceD3D();
        panic!();
    }

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


    // Cleanup
    imgui_raw::ImGui_ImplDX11_Shutdown();
    imgui_raw::ImGui_ImplSDL2_Shutdown();
    imgui_raw::DestroyContext(ptr::null_mut());

    // CleanupDeviceD3D();

    Ok(())
}
}

fn CreateDeviceD3D(hWnd: *mut c_void) -> bool {
    // // Setup swap chain
    // DXGI_SWAP_CHAIN_DESC sd;
    // ZeroMemory(&sd, sizeof(sd));
    // sd.BufferCount = 2;
    // sd.BufferDesc.Width = 0;
    // sd.BufferDesc.Height = 0;
    // sd.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
    // sd.BufferDesc.RefreshRate.Numerator = 60;
    // sd.BufferDesc.RefreshRate.Denominator = 1;
    // sd.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
    // sd.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
    // sd.OutputWindow = hWnd;
    // sd.SampleDesc.Count = 1;
    // sd.SampleDesc.Quality = 0;
    // sd.Windowed = TRUE;
    // sd.SwapEffect = DXGI_SWAP_EFFECT_DISCARD;

    // UINT createDeviceFlags = 0;
    // //createDeviceFlags |= D3D11_CREATE_DEVICE_DEBUG;
    // D3D_FEATURE_LEVEL featureLevel;
    // const D3D_FEATURE_LEVEL featureLevelArray[2] = { D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_10_0, };
    // if (D3D11CreateDeviceAndSwapChain(NULL, D3D_DRIVER_TYPE_HARDWARE, NULL, createDeviceFlags, featureLevelArray, 2, D3D11_SDK_VERSION, &sd, &g_pSwapChain, &g_pd3dDevice, &featureLevel, &g_pd3dDeviceContext) != S_OK)
    //     return false;

    // CreateRenderTarget();
    return true;
}

fn CleanupDeviceD3D() {
    // CleanupRenderTarget();
    // if (g_pSwapChain) { g_pSwapChain->Release(); g_pSwapChain = NULL; }
    // if (g_pd3dDeviceContext) { g_pd3dDeviceContext->Release(); g_pd3dDeviceContext = NULL; }
    // if (g_pd3dDevice) { g_pd3dDevice->Release(); g_pd3dDevice = NULL; }
}
