#pragma once
#include <Windows.h>
#include <stdint.h>
#include <memory>
#include "SDL_video.h"
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

#include <SDL.h>

class SDLWindow
{
    SDL_Window *m_window;
    SDLWindow(SDL_Window *window);

public:
    ~SDLWindow();
    static std::shared_ptr<SDLWindow> create(const char *title);

    SDL_Window *window() const
    {
        return m_window;
    }
    HWND handle() const;
    static std::shared_ptr<SampleWindow> create(const wchar_t *class_name,
                                                const wchar_t *window_name);
    auto main_loop(screenstate::ScreenState *state) -> bool;
};
