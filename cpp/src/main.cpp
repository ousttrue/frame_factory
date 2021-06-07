#include "imgui_app.h"
#include "dx11.h"

class FPS
{
    UINT m_beginTime = 0;
    UINT m_frameTime = 0;

public:
    FPS(int frame_rate)
    {
        m_frameTime = 1000 / frame_rate;
    }

    void begin()
    {
        m_beginTime = timeGetTime();
    }

    void wait()
    {
        auto now = timeGetTime();
        auto delta = now - m_beginTime;
        if (delta < m_frameTime)
        {
            Sleep(m_frameTime - delta);
        }
    }
};

#if 0
int main(int argc, char **argv)
#else
auto argc = __argc;
auto argv = __argv;
int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine, int nCmdShow)
#endif
{

    // auto window = SampleWindow::create(CLASS_NAME, L"CPP_SAMPLE");
    auto window = SDLWindow::create("CPP_SAMPLE");
    if (!window)
    {
        return 1;
    }

    auto d3d = DX11::create(window->handle());
    if (!d3d)
    {
        return 2;
    }

    ImGuiApp gui(window->window(), d3d->device().Get(), d3d->context().Get());
    gui.load(d3d->device().Get(), argc, argv);

    FPS fps(30);
    while (true)
    {
        fps.begin();

        screenstate::ScreenState state;
        if (!window->main_loop(&state))
        {
            break;
        }

        // update imgui
        gui.update(d3d->device().Get(), d3d->context().Get(), state);

        // render d3d
        if (!d3d->new_frame(state.Width, state.Height, gui.m_clearColor))
        {
            return 4;
        }
        gui.render();

        // flush
        fps.wait();
        d3d->flush();
    }

    return 0;
}
