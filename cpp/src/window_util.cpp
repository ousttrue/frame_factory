#include "window_util.h"
#include <imgui.h>
#include "save_windowplacement.h"

const auto JSON_FILE = L"cpp_sample.json";

// Forward declare message handler from imgui_impl_win32.cpp
extern IMGUI_IMPL_API LRESULT ImGui_ImplWin32_WndProcHandler(HWND hWnd,
                                                             UINT msg,
                                                             WPARAM wParam,
                                                             LPARAM lParam);

// Win32 message handler
static LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam)
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
        windowplacement::Save(hWnd, JSON_FILE);
        ::PostQuitMessage(0);
        return 0;
    }
    return ::DefWindowProc(hWnd, msg, wParam, lParam);
}

SampleWindow::SampleWindow(ATOM window_register, HWND hwnd)
    : m_window_register(window_register), m_hwnd(hwnd)
{
}

SampleWindow::~SampleWindow()
{
    DestroyWindow(m_hwnd);
    UnregisterClassW((const wchar_t *)m_window_register,
                     GetModuleHandle(nullptr));
}

std::shared_ptr<SampleWindow> SampleWindow::create(const wchar_t *class_name,
                                                   const wchar_t *window_name)
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
    auto window_register = RegisterClassExW(&wcex);
    if (window_register == 0)
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
    // ShowWindow(hwnd, SW_SHOW);
    windowplacement::Restore(hwnd, SW_SHOW, JSON_FILE);
    UpdateWindow(hwnd);
    if (!hwnd)
    {
        return nullptr;
    }

    return std::shared_ptr<SampleWindow>(
        new SampleWindow(window_register, hwnd));
}

auto SampleWindow::main_loop() -> WindowState
{
    WindowState state{};
    MSG msg;
    while (PeekMessageW(&msg, nullptr, 0, 0, PM_REMOVE) > 0)
    {
        if (msg.message == WM_QUIT)
        {
            state.closed = true;
            break;
        }
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    RECT rect;
    GetClientRect(m_hwnd, &rect);
    state.width = (uint16_t)rect.right;
    state.height = (uint16_t)rect.bottom;
    return state;
}
