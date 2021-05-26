#pragma once
#include <Windows.h>
#include <d3d11.h>
#include <stdint.h>

extern "C"
{
    // message
    auto FRAME_FACTORY_message_peek(uint32_t index) -> const uint8_t *;

    auto FRAME_FACTORY_message_clear() -> void;

    // asset
    auto FRAME_FACTORY_asset_path(const char *path) -> void;

    // new scene
    auto FRAME_FACTORY_scene_load(const char *path) -> uint32_t;

    // shutdown
    auto FRAME_FACTORY_scene_destroy() -> void;
    auto FRAME_FACTORY_shutdown() -> void;

    // render to texture
    auto FRAME_FACTORY_scene_render(uint32_t scene, ID3D11Device *device,
                                    ID3D11DeviceContext *context,
                                    ID3D11Texture2D *render_target,
                                    const void *state) -> bool;
}
