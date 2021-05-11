#pragma once
#include <Windows.h>
#include <stdint.h>

// Win32 message handler
LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

auto create_window(const wchar_t *class_name, const wchar_t *window_name)
    -> HWND;

struct WindowState
{
    bool closed;
    uint16_t width;
    uint16_t height;
};

auto main_loop(HWND hwnd) -> WindowState;
