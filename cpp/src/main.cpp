#include "window_util.h"
#include "extern_renderer.h"
#include <Windows.h>
#include <vector>
#include <fstream>
#include <wrl/client.h>
#include <imgui.h>
#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx11.h>

const auto CLASS_NAME = L"CPP_SAMPLE_CLASS";

static std::vector<char> ReadAllBytes(char const *filename)
{
    std::ifstream ifs(filename, std::ios::binary | std::ios::ate);
    if (!ifs)
    {
        return {};
    }
    auto pos = ifs.tellg();
    std::vector<char> buffer(pos);
    ifs.seekg(0, std::ios::beg);
    ifs.read(buffer.data(), pos);
    return buffer;
}


int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine, int nCmdShow)
{
    auto hwnd = create_window(CLASS_NAME, L"CPP_SAMPLE");
    if (!hwnd)
    {
        return 1;
    }

    auto d3d = FRAME_FACTORY_initialize(hwnd);
    if (!d3d)
    {
        return 2;
    }

    auto source = ReadAllBytes("../shaders/mvp.hlsl");
    if (source.empty())
    {
        return 3;
    }

    if (!FRAME_FACTORY_sample_scene(source.data(), source.size(), "vsMain",
                                    "psMain"))
    {
        return 4;
    }

    // Setup Dear ImGui context
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO &io = ImGui::GetIO();
    (void)io;
    // io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable
    // Keyboard Controls io.ConfigFlags |= ImGuiConfigFlags_NavEnableGamepad; //
    // Enable Gamepad Controls

    // Setup Dear ImGui style
    ImGui::StyleColorsDark();
    // ImGui::StyleColorsClassic();

    // Setup Platform/Renderer backends
    ImGui_ImplWin32_Init(hwnd);

    {
        Microsoft::WRL::ComPtr<ID3D11DeviceContext> context;
        d3d->GetImmediateContext(&context);

        // ImGui_ImplDX11_Init(d3d, context.Get());

        bool show_demo_window = true;
        while (main_loop(hwnd))
        {
            // {
            //     // Start the Dear ImGui frame
            //     ImGui_ImplDX11_NewFrame();
            //     ImGui_ImplWin32_NewFrame();
            //     ImGui::NewFrame();
            //     ImGui::ShowDemoWindow(&show_demo_window);
            //     // Rendering
            //     ImGui::Render();
            // }

            // const float clear_color_with_alpha[4] = { clear_color.x *
            // clear_color.w, clear_color.y * clear_color.w, clear_color.z *
            // clear_color.w, clear_color.w };
            // g_pd3dDeviceContext->OMSetRenderTargets(1, &g_mainRenderTargetView,
            // NULL);
            // g_pd3dDeviceContext->ClearRenderTargetView(g_mainRenderTargetView,
            // clear_color_with_alpha);
            FRAME_FACTORY_sample_render();
            // ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
            FRAME_FACTORY_flush();
        }

        // // Cleanup
        // ImGui_ImplDX11_Shutdown();
        // ImGui_ImplWin32_Shutdown();
        // ImGui::DestroyContext();
    }

    FRAME_FACTORY_destory();

    // CleanupDeviceD3D();
    DestroyWindow(hwnd);
    UnregisterClassW(CLASS_NAME, GetModuleHandle(nullptr));

    return 0;
}
