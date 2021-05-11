#include "window_util.h"
#include <imgui.h>

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

auto main_loop(HWND hwnd) -> WindowState
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
    GetClientRect(hwnd, &rect);
    state.width = rect.right;
    state.height = rect.bottom;
    return state;
}
