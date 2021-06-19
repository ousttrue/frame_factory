use std::{convert::TryInto, ffi::c_void, ptr};

use gen::sdl;

pub struct Gui {
    show_demo_window: bool,
    show_another_window: bool,
    pub clear_color: [f32; 4],
    f: f32,
    counter: i32,
}

impl Drop for Gui {
    fn drop(&mut self) {
        unsafe {
            gen::imgui::ImGui_ImplDX11_Shutdown();
            gen::imgui::ImGui_ImplSDL2_Shutdown();
            gen::imgui::DestroyContext(ptr::null_mut());
        }
    }
}

impl Gui {
    pub unsafe fn new(sdl_window: *mut c_void, device: *mut c_void, context: *mut c_void) -> Gui {
        gen::imgui::CreateContext(ptr::null_mut());
        let mut io = gen::imgui::GetIO().as_mut().unwrap();
        io.ConfigFlags |= gen::imgui::ImGuiConfigFlags_NavEnableKeyboard as i32; // Enable Keyboard Controls
                                                                                   //io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad;      // Enable Gamepad Controls
        io.ConfigFlags |= gen::imgui::ImGuiConfigFlags_DockingEnable as i32; // Enable Docking
        io.ConfigFlags |= gen::imgui::ImGuiConfigFlags_ViewportsEnable as i32; // Enable Multi-Viewport / Platform Windows
                                                                                 //io.ConfigViewportsNoAutoMerge = true;
                                                                                 //io.ConfigViewportsNoTaskBarIcon = true;

        // Setup Dear ImGui style
        gen::imgui::StyleColorsDark(ptr::null_mut());
        //gen::imgui::StyleColorsClassic();

        // When viewports are enabled we tweak WindowRounding/WindowBg so platform windows can look identical to regular ones.
        let style = gen::imgui::GetStyle().as_mut().unwrap();
        if (io.ConfigFlags & gen::imgui::ImGuiConfigFlags_ViewportsEnable as i32) != 0 {
            style.WindowRounding = 0.0f32;
            style.Colors[gen::imgui::ImGuiCol_WindowBg as usize].w = 1.0f32;
        }

        // Setup Platform/Renderer backends
        gen::imgui::ImGui_ImplSDL2_InitForD3D(sdl_window);
        gen::imgui::ImGui_ImplDX11_Init(device, context);

        // Load Fonts
        // - If no fonts are loaded, dear imgui will use the default font. You can also load multiple fonts and use gen::imgui::PushFont()/PopFont() to select them.
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

        Gui {
            show_demo_window: true,
            show_another_window: false,
            clear_color: [0.45f32, 0.55f32, 0.60f32, 1.00f32],
            f: 0.0f32,
            counter: 0,
        }
    }

    pub unsafe fn sdl_event(&self, p: *const sdl::SDL_Event) {
        gen::imgui::ImGui_ImplSDL2_ProcessEvent(p as *const c_void);
    }

    pub unsafe fn on_frame(&mut self, window: *mut c_void) {
        gen::imgui::ImGui_ImplDX11_NewFrame();
        gen::imgui::ImGui_ImplSDL2_NewFrame(window);
        gen::imgui::NewFrame();

        // 1. Show the big demo window (Most of the sample code is in gen::imgui::ShowDemoWindow()! You can browse its code to learn more about Dear ImGui!).
        if self.show_demo_window {
            gen::imgui::ShowDemoWindow(&mut self.show_demo_window);
        }

        // 2. Show a simple window that we create ourselves. We use a Begin/End pair to created a named window.
        {
            gen::imgui::Begin("Hello, world!\0".as_ptr() as *const i8, ptr::null_mut(), 0); // Create a window called "Hello, world!" and append into it.

            gen::imgui::Text("This is some useful text.".as_ptr() as *const i8); // Display some text (you can use a format strings too)
            gen::imgui::Checkbox(
                "Demo Window".as_ptr() as *const i8,
                &mut self.show_demo_window,
            ); // Edit bools storing our window open/close state
            gen::imgui::Checkbox(
                "Another Window".as_ptr() as *const i8,
                &mut self.show_another_window,
            );

            gen::imgui::SliderFloat(
                "float".as_ptr() as *const i8,
                &mut self.f,
                0.0f32,
                1.0f32,
                "%.3f".as_ptr() as *const i8,
                0,
            ); // Edit 1 float using a slider from 0.0f to 1.0f
            // let clear_color: [f32; 3] = self.clear_color[0..3].try_into().unwrap();
            gen::imgui::ColorEdit3("clear color".as_ptr() as *const i8, self.clear_color.as_mut_ptr(), 0); // Edit 3 floats representing a color

            if gen::imgui::Button(
                "Button".as_ptr() as *const i8,
                &gen::imgui::ImVec2 { x: 0f32, y: 0f32 },
            )
            // Buttons return true when clicked (most widgets return true when edited/activated)
            {
                self.counter += 1;
            }
            gen::imgui::SameLine(0f32, -1f32);
            // gen::imgui::TextV("counter = %d".as_ptr() as *const i8, counter);

            // gen::imgui::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / gen::imgui::GetIO().Framerate, gen::imgui::GetIO().Framerate);
            gen::imgui::End();
        }

        // 3. Show another simple window.
        if self.show_another_window {
            gen::imgui::Begin(
                "Another Window".as_ptr() as *const i8,
                &mut self.show_another_window,
                0,
            ); // Pass a pointer to our bool variable (the window will have a closing button that will clear the bool when clicked)
            gen::imgui::Text("Hello from another window!".as_ptr() as *const i8);
            if gen::imgui::Button(
                "Close Me".as_ptr() as *const i8,
                &gen::imgui::ImVec2 { x: 0f32, y: 0f32 },
            ) {
                self.show_another_window = false;
            }
            gen::imgui::End();
        }

        // Rendering
        gen::imgui::Render();
    }

    pub unsafe fn render(&self) {
        gen::imgui::ImGui_ImplDX11_RenderDrawData(gen::imgui::GetDrawData());

        // Update and Render additional Platform Windows
        let mut io = gen::imgui::GetIO().as_mut().unwrap();
        if (io.ConfigFlags & gen::imgui::ImGuiConfigFlags_ViewportsEnable as i32) != 0 {
            gen::imgui::UpdatePlatformWindows();
            gen::imgui::RenderPlatformWindowsDefault(ptr::null_mut(), ptr::null_mut());
        }
    }
}
