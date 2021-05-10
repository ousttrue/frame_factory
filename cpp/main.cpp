#include <Windows.h>
#include <vector>
#include <stdint.h>
#include <fstream>

extern "C"
{
    auto FRAME_FACTORY_initialize(HWND hwnd) -> bool;

    auto FRAME_FACTORY_sample_scene(const char *source, size_t source_size,
                                    const char *vs_main, const char *ps_main)
        -> bool;

    auto FRAME_FACTORY_sample_render() -> void;
}

LRESULT CALLBACK wndproc(HWND hwnd, UINT msg, WPARAM wparam, LPARAM lparam)
{
    switch (msg)
    {
    case WM_DESTROY:
    {
        PostQuitMessage(0);
        return 0;
    }
    }

    return DefWindowProcW(hwnd, msg, wparam, lparam);
}

auto create_window(const wchar_t *class_name, const wchar_t *window_name)
    -> HWND
{
    WNDCLASSEXW wcex = {};
    wcex.cbSize = sizeof(WNDCLASSEXW);
    wcex.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wcex.lpfnWndProc = wndproc;
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
    if(!ifs)
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

int main(int argc, char **argv)
{
    auto hwnd = create_window(L"CLASS_NAME", L"WINDOW_NAME");
    if (!hwnd)
    {
        return 1;
    }

    if (!FRAME_FACTORY_initialize(hwnd))
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

    while (main_loop(hwnd))
    {
        FRAME_FACTORY_sample_render();
    }

    return 0;
}
