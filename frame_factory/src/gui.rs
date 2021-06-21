use std::default::Default;
use std::{ffi::c_void, ptr};

use com_ptr::ComPtr;
use gen::imgui;
use gen::sdl;
use winapi::um::commdlg::OPENFILENAMEA;
use winapi::um::d3d11;

use crate::scene_view::SceneView;

pub struct Gui {
    show_demo_window: bool,
    show_another_window: bool,
    pub clear_color: [f32; 4],
    f: f32,
    counter: i32,

    scenes: Vec<SceneView>,
}

impl Drop for Gui {
    fn drop(&mut self) {
        unsafe {
            imgui::ImGui_ImplDX11_Shutdown();
            imgui::ImGui_ImplSDL2_Shutdown();
            imgui::DestroyContext(ptr::null_mut());
        }
    }
}

impl Gui {
    pub unsafe fn new(
        sdl_window: *mut c_void,
        device: &ComPtr<d3d11::ID3D11Device>,
        context: &ComPtr<d3d11::ID3D11DeviceContext>,
    ) -> Gui {
        imgui::CreateContext(ptr::null_mut());
        let mut io = imgui::GetIO().as_mut().unwrap();
        io.ConfigFlags |= imgui::ImGuiConfigFlags_NavEnableKeyboard as i32; // Enable Keyboard Controls
                                                                            //io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
        io.ConfigFlags |= imgui::ImGuiConfigFlags_DockingEnable as i32; // Enable Docking
        io.ConfigFlags |= imgui::ImGuiConfigFlags_ViewportsEnable as i32; // Enable Multi-Viewport / Platform Windows
                                                                          //io.ConfigViewportsNoAutoMerge = true;
                                                                          //io.ConfigViewportsNoTaskBarIcon = true;

        // Setup Dear ImGui style
        imgui::StyleColorsDark(ptr::null_mut());
        //imgui::StyleColorsClassic();

        // When viewports are enabled we tweak WindowRounding/WindowBg so platform windows can look identical to regular ones.
        let style = imgui::GetStyle().as_mut().unwrap();
        if (io.ConfigFlags & imgui::ImGuiConfigFlags_ViewportsEnable as i32) != 0 {
            style.WindowRounding = 0.0f32;
            style.Colors[imgui::ImGuiCol_WindowBg as usize].w = 1.0f32;
        }

        // Setup Platform/Renderer backends
        imgui::ImGui_ImplSDL2_InitForD3D(sdl_window);
        imgui::ImGui_ImplDX11_Init(
            device.as_ptr() as *mut c_void,
            context.as_ptr() as *mut c_void,
        );

        // Load Fonts
        // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use imgui::PushFont()/PopFont() to select them.
        // - AddFontFromFileTTF() will return the ImFont* so you can store it if you need to select the font among multiple.
        // - If the file cannot be loaded, the function will return NULL. Please handle those errors in your application (e.g. use an assertion, or display an error and quit).
        // - The fonts will be rasterized at a given size (w/ oversampling) and stored into a texture when calling ImFontAtlas::Build()/GetTexDataAsXXXX(), which ImGui_ImplXXXX_NewFrame below will call.
        // - Read 'docs/FONTS.md' for more instructions and details.
        // - Remember that in C/C++ if you want to include a backslash \ in a string literal you need to write a double backslash \\ !
        //io.Fonts.AddFontDefault();
        //io.Fonts.AddFontFromFileTTF("../../misc/fonts/Roboto-Medium.ttf", 16.0f);
        //io.Fonts.AddFontFromFileTTF("../../misc/fonts/Cousine-Regular.ttf", 15.0f);
        //io.Fonts.AddFontFromFileTTF("../../misc/fonts/DroidSans.ttf", 16.0f);
        //io.Fonts.AddFontFromFileTTF("../../misc/fonts/ProggyTiny.ttf", 10.0f);
        //ImFont* font = io.Fonts.AddFontFromFileTTF("c:\\Windows\\Fonts\\ArialUni.ttf", 18.0f, NULL, io.Fonts.GetGlyphRangesJapanese());
        //IM_ASSERT(font != NULL);

        Gui {
            show_demo_window: true,
            show_another_window: false,
            clear_color: [0.45f32, 0.55f32, 0.60f32, 1.00f32],
            f: 0.0f32,
            counter: 0,

            scenes: Vec::new(),
        }
    }

    pub unsafe fn sdl_event(&self, p: *const sdl::SDL_Event) {
        imgui::ImGui_ImplSDL2_ProcessEvent(p as *const c_void);
    }

    pub unsafe fn docking(&mut self) {
        let mut window_flags = imgui::ImGuiWindowFlags_MenuBar | imgui::ImGuiWindowFlags_NoDocking;

        {
            let viewport = imgui::GetMainViewport().as_ref().unwrap();
            imgui::SetNextWindowPos(&viewport.WorkPos, 0, &Default::default());
            imgui::SetNextWindowSize(&viewport.WorkSize, 0);
            imgui::SetNextWindowViewport(viewport.ID);
            imgui::PushStyleVar(imgui::ImGuiStyleVar_WindowRounding, 0.0f32);
            imgui::PushStyleVar(imgui::ImGuiStyleVar_WindowBorderSize, 0.0f32);
            window_flags |= imgui::ImGuiWindowFlags_NoTitleBar
                | imgui::ImGuiWindowFlags_NoCollapse
                | imgui::ImGuiWindowFlags_NoResize
                | imgui::ImGuiWindowFlags_NoMove;
            window_flags |=
                imgui::ImGuiWindowFlags_NoBringToFrontOnFocus | imgui::ImGuiWindowFlags_NoNavFocus;
        }
        if imgui::Begin(T!("Master Window"), ptr::null_mut(), window_flags) {
            // Declare Central dockspace
            let dockspace_id = imgui::GetID(T!("MyDockSpace"));
            imgui::DockSpace(
                dockspace_id,
                &Default::default(),
                imgui::ImGuiDockNodeFlags_None,
                ptr::null_mut(),
            );
        }

        if imgui::BeginMenuBar() {
            if imgui::BeginMenu(T!("File"), true) {
                if imgui::MenuItem(T!("Open"), T!("Ctrl+O"), false, true) {
                    let mut str_custom = [0; 256];
                    let mut str_file = [0; winapi::shared::minwindef::MAX_PATH];
                    let mut ofn = OPENFILENAMEA {
                        lStructSize: std::mem::size_of::<OPENFILENAMEA>() as u32,
                        hwndOwner: ptr::null_mut(),
                        lpstrFilter: T!("glb files {*.glb}\0*.glb\0All files {*.*}\0*.*\0\0"),
                        lpstrCustomFilter: str_custom.as_mut_ptr(),
                        nMaxCustFilter: std::mem::size_of_val(&str_custom) as u32,
                        nFilterIndex: 0,
                        lpstrFile: str_file.as_mut_ptr(),
                        nMaxFile: std::mem::size_of_val(&str_file) as u32,
                        Flags: winapi::um::commdlg::OFN_FILEMUSTEXIST,
                        ..Default::default()
                    };
                    if winapi::um::commdlg::GetOpenFileNameA(&mut ofn) != 0 {
                        if let Some(scene) = SceneView::load_gltf(&str_file) {
                            self.scenes.push(scene);
                        }
                    }
                }
                imgui::EndMenu();
            }
            imgui::EndMenuBar();
        }
        imgui::End();
        imgui::PopStyleVar(2);
    }

    pub unsafe fn gui(
        &mut self,
        device: &ComPtr<d3d11::ID3D11Device>,
        context: &ComPtr<d3d11::ID3D11DeviceContext>,
        state: &scene::ScreenState,
    ) {
        // 1. Show the big demo window (Most of the sample code is in imgui::ShowDemoWindow()! You can browse its code to learn more about Dear ImGui!).
        if self.show_demo_window {
            imgui::ShowDemoWindow(&mut self.show_demo_window);
        }

        // 2. Show a simple window that we create ourselves. We use a Begin/End pair to created a named window.
        {
            imgui::Begin("Hello, world!\0".as_ptr() as *const i8, ptr::null_mut(), 0); // Create a window called "Hello, world!" and append into it.

            imgui::Text("This is some useful text.".as_ptr() as *const i8); // Display some text (you can use a format strings too)
            imgui::Checkbox(
                "Demo Window".as_ptr() as *const i8,
                &mut self.show_demo_window,
            ); // Edit bools storing our window open/close state
            imgui::Checkbox(
                "Another Window".as_ptr() as *const i8,
                &mut self.show_another_window,
            );

            imgui::SliderFloat(
                "float".as_ptr() as *const i8,
                &mut self.f,
                0.0f32,
                1.0f32,
                "%.3f".as_ptr() as *const i8,
                0,
            ); // Edit 1 float using a slider from 0.0f to 1.0f
               // let clear_color: [f32; 3] = self.clear_color[0..3].try_into().unwrap();
            imgui::ColorEdit3(
                "clear color".as_ptr() as *const i8,
                self.clear_color.as_mut_ptr(),
                0,
            ); // Edit 3 floats representing a color

            if imgui::Button(
                "Button".as_ptr() as *const i8,
                &imgui::ImVec2 { x: 0f32, y: 0f32 },
            )
            // Buttons return true when clicked (most widgets return true when edited/activated)
            {
                self.counter += 1;
            }
            imgui::SameLine(0f32, -1f32);
            // imgui::TextV("counter = %d".as_ptr() as *const i8, counter);

            // imgui::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / imgui::GetIO().Framerate, imgui::GetIO().Framerate);
            imgui::End();
        }

        // 3. Show another simple window.
        if self.show_another_window {
            imgui::Begin(
                "Another Window".as_ptr() as *const i8,
                &mut self.show_another_window,
                0,
            ); // Pass a pointer to our bool variable (the window will have a closing button that will clear the bool when clicked)
            imgui::Text("Hello from another window!".as_ptr() as *const i8);
            if imgui::Button(
                "Close Me".as_ptr() as *const i8,
                &imgui::ImVec2 { x: 0f32, y: 0f32 },
            ) {
                self.show_another_window = false;
            }
            imgui::End();
        }

        // 3d view
        for scene in &mut self.scenes {
            imgui::PushStyleVar_(imgui::ImGuiStyleVar_WindowPadding, &Default::default());
            imgui::SetNextWindowSize(
                &imgui::ImVec2 {
                    x: 512f32,
                    y: 512f32,
                },
                imgui::ImGuiCond_Once,
            );
            if imgui::Begin(
                scene.name.as_ptr() as *mut i8,
                ptr::null_mut(),
                imgui::ImGuiWindowFlags_NoScrollbar | imgui::ImGuiWindowFlags_NoScrollWithMouse,
            ) {
                let mut size = Default::default();
                imgui::pGetContentRegionAvail(&mut size);
                let mut pos = Default::default();
                imgui::pGetWindowPos(&mut pos);
                // https://forum.dlang.org/thread/dkamxcamwttszxwwxttv@forum.dlang.org

                let frame_height = imgui::GetFrameHeight() as i16;

                // render
                let crop = state.crop(
                    pos.x as i16,
                    pos.y as i16 + frame_height,
                    size.x as i16,
                    size.y as i16,
                );
                if let Ok(srv) = scene.render(device, context, &crop) {
                    imgui::ImageButton(
                        srv.as_ptr() as *mut c_void,
                        &size,
                        &Default::default(),
                        &imgui::ImVec2 {
                            x: 1.0f32,
                            y: 1.0f32,
                        },
                        0,
                        &Default::default(),
                        &imgui::ImVec4 {
                            x: 1f32,
                            y: 1f32,
                            z: 1f32,
                            w: 1f32,
                        },
                    );
                }
            }
            imgui::End();
            imgui::PopStyleVar(1);
        }
    }

    pub unsafe fn update(
        &mut self,
        window: *mut c_void,
        device: &ComPtr<d3d11::ID3D11Device>,
        context: &ComPtr<d3d11::ID3D11DeviceContext>,
        state: &scene::ScreenState,
    ) {
        imgui::ImGui_ImplDX11_NewFrame();
        imgui::ImGui_ImplSDL2_NewFrame(window);
        imgui::NewFrame();

        self.docking();
        self.gui(device, context, state);

        // Rendering
        imgui::Render();
    }

    pub unsafe fn render(&self) {
        imgui::ImGui_ImplDX11_RenderDrawData(imgui::GetDrawData());

        // Update and Render additional Platform Windows
        let io = imgui::GetIO().as_mut().unwrap();
        if (io.ConfigFlags & imgui::ImGuiConfigFlags_ViewportsEnable as i32) != 0 {
            imgui::UpdatePlatformWindows();
            imgui::RenderPlatformWindowsDefault(ptr::null_mut(), ptr::null_mut());
        }
    }
}
