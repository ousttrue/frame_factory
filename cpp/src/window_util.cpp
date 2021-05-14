#include "window_util.h"
#include <imgui.h>
#include <windowsx.h>
#include "save_windowplacement.h"

const auto JSON_FILE = L"cpp_sample.json";

static screenstate::ScreenState g_state = {};
static UINT g_startTime = 0;
static UINT g_lastTime = 0;

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
    case WM_DESTROY:
        windowplacement::Save(hWnd, JSON_FILE);
        ::PostQuitMessage(0);
        return 0;

    case WM_SIZE:
        if (wParam != SIZE_MINIMIZED)
        {
            g_state.Width = LOWORD(lParam);
            g_state.Height = HIWORD(lParam);
        }
        break;

        // TODO:
        // case WM_KEYDOWN:
        //     if (pSample)
        //     {
        //         pSample->OnKeyDown(static_cast<UINT8>(wParam));
        //     }
        //     return 0;

        // case WM_KEYUP:
        //     if (pSample)
        //     {
        //         pSample->OnKeyUp(static_cast<UINT8>(wParam));
        //     }
        //     return 0;

    case WM_ERASEBKGND:
        return 1;

    case WM_MOUSEMOVE:
        g_state.MouseX = GET_X_LPARAM(lParam);
        g_state.MouseY = GET_Y_LPARAM(lParam);
        return 0;

    case WM_LBUTTONDOWN:
        if (!g_state.HasCapture())
        {
            SetCapture(hWnd);
        }
        g_state.Set(screenstate::MouseButtonFlags::LeftDown);
        return 0;

    case WM_LBUTTONUP:
        g_state.Unset(screenstate::MouseButtonFlags::LeftDown);
        if (!g_state.HasCapture())
        {
            ReleaseCapture();
        }
        return 0;

    case WM_RBUTTONDOWN:
        if (!g_state.HasCapture())
        {
            SetCapture(hWnd);
        }
        g_state.Set(screenstate::MouseButtonFlags::RightDown);
        return 0;

    case WM_RBUTTONUP:
        g_state.Unset(screenstate::MouseButtonFlags::RightDown);
        if (!g_state.HasCapture())
        {
            ReleaseCapture();
        }
        return 0;

    case WM_MBUTTONDOWN:
        if (!g_state.HasCapture())
        {
            SetCapture(hWnd);
        }
        g_state.Set(screenstate::MouseButtonFlags::MiddleDown);
        return 0;

    case WM_MBUTTONUP:
        g_state.Unset(screenstate::MouseButtonFlags::MiddleDown);
        if (!g_state.HasCapture())
        {
            ReleaseCapture();
        }
        return 0;

    case WM_MOUSEWHEEL:
    {
        auto zDelta = GET_WHEEL_DELTA_WPARAM(wParam);
        if (zDelta < 0)
        {
            g_state.Set(screenstate::MouseButtonFlags::WheelMinus);
        }
        else if (zDelta > 0)
        {
            g_state.Set(screenstate::MouseButtonFlags::WheelPlus);
        }
        return 0;
    }

    case WM_SYSCOMMAND:
        if ((wParam & 0xfff0) == SC_KEYMENU) // Disable ALT application menu
            return 0;
        break;

    // case WM_SETCURSOR:
    //     if (!g_enableSetCursor)
    //     {
    //         if (LOWORD(lParam) == HTCLIENT)
    //         {
    //             g_state.Set(screenstate::MouseButtonFlags::CursorUpdate);
    //             return 1;
    //         }
    //     }
    //     break;
    // }
        
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

    g_startTime = timeGetTime();
    return std::shared_ptr<SampleWindow>(
        new SampleWindow(window_register, hwnd));
}

auto SampleWindow::main_loop(screenstate::ScreenState *state) -> bool
{
    // WindowState state{};
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

    auto now = timeGetTime();
    if (g_startTime)
    {
        now -= g_startTime;
    }
    else
    {
        now = 0;
    }
    if (now > g_lastTime)
    {
        g_state.ElapsedSeconds = now * 0.001f;
        auto delta = now - g_lastTime;
        g_state.DeltaSeconds = delta * 0.001f;
    }
    else
    {
        // work around
        g_startTime = now;
        g_state.DeltaSeconds = 0.016f;
        g_state.ElapsedSeconds = 0.016f;
    }
    g_lastTime = now;

    *state = g_state;
    g_state.Clear();

    return true;
}
