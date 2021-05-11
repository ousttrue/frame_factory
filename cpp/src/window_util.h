#pragma once
#include <Windows.h>

// Win32 message handler
LRESULT WINAPI WndProc(HWND hWnd, UINT msg, WPARAM wParam, LPARAM lParam);

auto create_window(const wchar_t *class_name, const wchar_t *window_name)
    -> HWND;

auto main_loop(HWND hwnd) -> bool;
