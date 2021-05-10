#include <Windows.h>

extern "C"
{
    auto FRAME_FACTORY_initialize(HWND hwnd) -> bool;

    auto FRAME_FACTORY_sample_scene(const char *source, size_t source_size,
                                    const char *vs_main, const char *ps_main)
        -> bool;

    auto FRAME_FACTORY_sample_render() -> void;
}

int main(int argc, char **argv)
{
    FRAME_FACTORY_initialize(nullptr);

    return 0;
}
