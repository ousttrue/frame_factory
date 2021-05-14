#pragma once
#include <Windows.h>
#include <d3d11.h>
#include <stdint.h>

extern "C"
{
    auto FRAME_FACTORY_sample_create(ID3D11Device *device, const char *source,
                                     size_t source_size, const char *vs_main,
                                     const char *ps_main) -> uint32_t;
    auto FRAME_FACTORY_sample_render(uint32_t scene, ID3D11Device *device,
                                     ID3D11DeviceContext *context,
                                     ID3D11Texture2D *render_target) -> bool;
    auto FRAME_FACTORY_sample_destroy() -> void;
}
