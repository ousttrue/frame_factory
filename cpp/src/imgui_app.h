#pragma once
#include "window_util.h"
#include <memory>
#include <list>
#include <d3d11.h>

struct ImGuiApp
{
    bool m_show_demo_window = true;
    float m_clearColor[4] = {0.0f, 0.2f, 0.4f, 1.0f};
    std::list<std::unique_ptr<class RustRenderer>> m_scenes;
    HWND m_hwnd;

public:
    ImGuiApp(HWND hwnd, ID3D11Device *device, ID3D11DeviceContext *context);
    ~ImGuiApp();
    void load(ID3D11Device *device, int argc, char **argv);
    void update(ID3D11Device *device, ID3D11DeviceContext *context,
                const screenstate::ScreenState &state);
    void gui(ID3D11Device *device, ID3D11DeviceContext *context,
             const screenstate::ScreenState &state);
    void render();
};
