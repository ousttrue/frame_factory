#include "window_util.h"
#include "extern_renderer.h"
#include "dx11.h"
#include <Windows.h>
#include <vector>
#include <fstream>
#include <functional>
#include <wrl/client.h>
#include <imgui.h>
#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx11.h>

const auto CLASS_NAME = L"CPP_SAMPLE_CLASS";

static std::vector<char> ReadAllBytes(char const *filename)
{
    std::ifstream ifs(filename, std::ios::binary | std::ios::ate);
    if (!ifs)
    {
        return {};
    }
    auto pos = ifs.tellg();
    std::vector<char> buffer(pos);
    ifs.seekg(0, std::ios::beg);
    ifs.read(buffer.data(), pos);
    return buffer;
}

using ViewFunc = std::function<ID3D11ShaderResourceView *(int, int, int, int)>;

class ImGuiApp
{
    bool show_demo_window = true;

public:
    ImGuiApp(HWND hwnd, ID3D11Device *device, ID3D11DeviceContext *context)
    {
        // Setup Dear ImGui context
        IMGUI_CHECKVERSION();
        ImGui::CreateContext();
        ImGuiIO &io = ImGui::GetIO();
        (void)io;
        // io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable
        // Keyboard Controls io.ConfigFlags |=
        // ImGuiConfigFlags_NavEnableGamepad; // Enable Gamepad Controls

        // Setup Dear ImGui style
        ImGui::StyleColorsDark();
        // ImGui::StyleColorsClassic();

        // Setup Platform/Renderer backends

        ImGui_ImplWin32_Init(hwnd);
        ImGui_ImplDX11_Init(device, context);
    }

    ~ImGuiApp()
    {
        // Cleanup
        ImGui_ImplDX11_Shutdown();
        ImGui_ImplWin32_Shutdown();
        ImGui::DestroyContext();
    }

    void update(const ViewFunc &render)
    {
        // Start the Dear ImGui frame
        ImGui_ImplDX11_NewFrame();
        ImGui_ImplWin32_NewFrame();
        ImGui::NewFrame();

        gui(render);

        // end
        ImGui::Render();
    }

    void gui(const ViewFunc &render)
    {
        // demo
        if (show_demo_window)
        {
            ImGui::ShowDemoWindow(&show_demo_window);
        }

        // 3d view
        ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0, 0));
        if (ImGui::Begin("render target", nullptr,
                         ImGuiWindowFlags_NoScrollbar |
                             ImGuiWindowFlags_NoScrollWithMouse))
        {
            auto size = ImGui::GetContentRegionAvail();
            auto pos = ImGui::GetWindowPos();
            auto frameHeight = ImGui::GetFrameHeight();
            auto mouseXY = ImGui::GetMousePos();

            // render
            auto x = mouseXY.x - pos.x;
            auto y = mouseXY.y - pos.y - frameHeight;

            auto renderTarget =
                render((int)x, (int)y, (int)size.x, (int)size.y);

            ImGui::ImageButton((ImTextureID)renderTarget, size,
                               ImVec2(0.0f, 0.0f), ImVec2(1.0f, 1.0f), 0);
        }
        ImGui::End();
        ImGui::PopStyleVar();
    }

    void render()
    {
        ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
    }
};

class RustRenderer
{
    bool m_initialized;

public:
    RustRenderer(ID3D11Device *device, const char *data, size_t len)
    {
        m_initialized =
            FRAME_FACTORY_sample_create(device, data, len, "vsMain", "psMain");
    }

    ~RustRenderer()
    {
        FRAME_FACTORY_sample_destroy();
    }

    bool render(ID3D11Device *device,
                                     ID3D11DeviceContext *context, int w, int h)
    {
        return FRAME_FACTORY_sample_render(device, context, w, h);
    }
};

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance,
                   LPSTR lpCmdLine, int nCmdShow)
{
    auto window = SampleWindow::create(CLASS_NAME, L"CPP_SAMPLE");
    if (!window)
    {
        return 1;
    }

    {
        auto d3d = DX11::create(window->handle());
        if (!d3d)
        {
            return 2;
        }

        auto source = ReadAllBytes("../shaders/mvp.hlsl");
        if (source.empty())
        {
            return 3;
        }

        float clearColor[] = {0.0f, 0.2f, 0.4f, 1.0f};
        {
            RustRenderer rust(d3d->device().Get(), source.data(),
                              source.size());
            ImGuiApp gui(window->handle(), d3d->device().Get(),
                         d3d->context().Get());
            for (WindowState state = {}; !state.closed;
                 state = window->main_loop())
            {
                // update imgui
                auto func = [&rust, &d3d](int x, int y, int w, int h) {
                    auto srv =
                        d3d->setup_render_target(w, h);
                    if (!rust.render(d3d->device().Get(), d3d->context().Get(),
                                     w, h))
                    {
                        return (ID3D11ShaderResourceView*)nullptr;
                    }
                    return srv;
                };
                gui.update(func);

                // render d3d
                if (!d3d->new_frame(state.width, state.height, clearColor))
                {
                    return 4;
                }
                gui.render();
                d3d->flush();
            }
        }
    }

    return 0;
}
