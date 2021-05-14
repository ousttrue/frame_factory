#pragma once
#include <Windows.h>
#include <stdint.h>
#include <memory>
#include "ScreenState.h"

class SampleWindow
{
    ATOM m_window_register;
    HWND m_hwnd;

    SampleWindow(ATOM window_register, HWND hwnd);

public:
    ~SampleWindow();

    HWND handle() const
    {
        return m_hwnd;
    }

    static std::shared_ptr<SampleWindow> create(const wchar_t *class_name,
                                                const wchar_t *window_name);
    auto main_loop(screenstate::ScreenState *state) -> bool;
};
