#include <Windows.h>
#include <vector>
#include <stdint.h>
#include <fstream>
#include <d3d11.h>
#include <wrl/client.h>
#include <imgui.h>
#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx11.h>

extern "C"
{
    auto FRAME_FACTORY_initialize(HWND hwnd) -> void *;

    auto FRAME_FACTORY_sample_scene(const char *source, size_t source_size,
                                    const char *vs_main, const char *ps_main)
        -> bool;

    auto FRAME_FACTORY_sample_render() -> void;
    auto FRAME_FACTORY_flush() -> void;
}

// Forward declare message handler from imgui_impl_win32.cpp
extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd,
                                                             UINT msg,
                                                             WPARAM wParam,
                                                             LPARAM lParam);

// Win32 message handler
LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam)
{
    if (ImGui_ImplWin32_WndProcHandler(hWnd, msg, wParam, lParam))
        return true;

    switch (msg)
    {
    case WM_SIZE:
        // if (g_pd3dDevice != NULL && wParam != SIZE_MINIMIZED)
        // {
        //     CleanupRenderTarget();
        //     g_pSwapChain->ResizeBuffers(0, (UINT)LOWORD(lParam),
        //                                 (UINT)HIWORD(lParam),
        //                                 DXGI_FORMAT_UNKNOWN, 0);
        //     CreateRenderTarget();
        // }
        return 0;
    case WM_SYSCOMMAND:
        if ((wParam & 0xfff0) == SC_KEYMENU) // Disable ALT application menu
            return 0;
        break;
    case WM_DESTROY:
        ::PostQuitMessage(0);
        return 0;
    }
    return ::DefWindowProc(hWnd, msg, wParam, lParam);
}

auto create_window(const wchar_t *class_name, const wchar_t *window_name)
    -> HWND
{
    WNDCLASSEXW wcex = {};
    wcex.cbSize = sizeof(WNDCLASSEXW);
    wcex.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wcex.lpfnWndProc = WndProc;
    wcex.cbClsExtra = 0;
    wcex.cbWndExtra = 0;
    wcex.hInstance = GetModuleHandleW(nullptr);
    wcex.hbrBackground = nullptr;
    wcex.lpszMenuName = nullptr;
    wcex.hIcon = nullptr;
    wcex.hCursor = nullptr;
    wcex.lpszClassName = class_name;
    wcex.hIconSm = nullptr;
    if (RegisterClassExW(&wcex) == 0)
    {
        return nullptr;
    }

    auto hwnd = CreateWindowExW(WS_EX_APPWINDOW, class_name, window_name,
                                WS_OVERLAPPEDWINDOW, CW_USEDEFAULT,
                                CW_USEDEFAULT, 1280, 720, nullptr, nullptr,
                                GetModuleHandleW(nullptr), nullptr);
    if (!hwnd)
    {
        return nullptr;
    }

    ShowWindow(hwnd, SW_SHOW);
    UpdateWindow(hwnd);
    return hwnd;
}

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

auto main_loop(HWND hwnd) -> bool
{
    MSG msg;
    while (PeekMessageW(&msg, nullptr, 0, 0, PM_REMOVE) > 0)
    {
        if (msg.message == WM_QUIT)
        {
            return false;
        }
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    return true;
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine, int nCmdShow)
{
    auto hwnd = create_window(L"CLASS_NAME", L"WINDOW_NAME");
    if (!hwnd)
    {
        return 1;
    }

    auto d3d = (ID3D11Device *)FRAME_FACTORY_initialize(hwnd);
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
        // error
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

    Microsoft::WRL::ComPtr<ID3D11DeviceContext> context;
    d3d->GetImmediateContext(&context);

    ImGui_ImplDX11_Init(d3d, context.Get());

    bool show_demo_window = true;
    while (main_loop(hwnd))
    {
        {
            // Start the Dear ImGui frame
            ImGui_ImplDX11_NewFrame();
            ImGui_ImplWin32_NewFrame();
            ImGui::NewFrame();
            ImGui::ShowDemoWindow(&show_demo_window);
            // Rendering
            ImGui::Render();
        }

        // const float clear_color_with_alpha[4] = { clear_color.x *
        // clear_color.w, clear_color.y * clear_color.w, clear_color.z *
        // clear_color.w, clear_color.w };
        // g_pd3dDeviceContext->OMSetRenderTargets(1, &g_mainRenderTargetView,
        // NULL);
        // g_pd3dDeviceContext->ClearRenderTargetView(g_mainRenderTargetView,
        // clear_color_with_alpha);
        FRAME_FACTORY_sample_render();
        ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
        FRAME_FACTORY_flush();
    }

    // Cleanup
    ImGui_ImplDX11_Shutdown();
    ImGui_ImplWin32_Shutdown();
    ImGui::DestroyContext();

    // CleanupDeviceD3D();
    ::DestroyWindow(hwnd);
    // ::UnregisterClass(wc.lpszClassName, wc.hInstance);

    return 0;
}
