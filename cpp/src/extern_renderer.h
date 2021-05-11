#pragma once
#include <Windows.h>
#include <d3d11.h>

extern "C"
{
    auto FRAME_FACTORY_initialize(HWND hwnd) -> ID3D11Device *;
    auto FRAME_FACTORY_destory()
    {
        FRAME_FACTORY_initialize(nullptr);
    }

    auto FRAME_FACTORY_sample_scene(const char *source, size_t source_size,
                                    const char *vs_main, const char *ps_main)
        -> bool;

    auto FRAME_FACTORY_new_frame(uint32_t width, uint32_t height) -> void;
    auto FRAME_FACTORY_sample_render() -> void;
    auto FRAME_FACTORY_flush() -> void;
}
