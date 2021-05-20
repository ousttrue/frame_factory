#include "window_util.h"
#include "extern_renderer.h"
#include "dx11.h"
#include <Windows.h>
#include <vector>
#include <list>
#include <fstream>
#include <memory>
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

class RustRenderer
{
    uint32_t m_scene = 0;

    int m_width = 0;
    int m_height = 0;
    Microsoft::WRL::ComPtr<ID3D11Texture2D> m_texture;
    Microsoft::WRL::ComPtr<ID3D11ShaderResourceView> m_srv;

    RustRenderer(uint32_t scene)
    {
        m_scene = scene;
    }

public:
    ~RustRenderer()
    {
        FRAME_FACTORY_shutdown();
    }

    static std::unique_ptr<RustRenderer> load_sample(ID3D11Device *device)
    {
        auto scene = FRAME_FACTORY_scene_sample(device, "");
        if (!scene)
        {
            return nullptr;
        }
        return std::unique_ptr<RustRenderer>(new RustRenderer(scene));
    }

    static std::unique_ptr<RustRenderer> load_gltf(ID3D11Device *device,
                                                   const char *path)
    {
        auto scene = FRAME_FACTORY_scene_load(device, path);
        if (!scene)
        {
            return nullptr;
        }
        return std::unique_ptr<RustRenderer>(new RustRenderer(scene));
    }

    ID3D11ShaderResourceView *render(ID3D11Device *device,
                                     ID3D11DeviceContext *context,
                                     const screenstate::ScreenState &state)
    {
        if (m_width != state.Width || m_height != state.Height)
        {
            m_texture = nullptr;
            m_srv = nullptr;
        }

        if (!m_texture)
        {
            if (state.Width == 0 || state.Height == 0)
            {
                return nullptr;
            }
            m_width = state.Width;
            m_height = state.Height;

            D3D11_TEXTURE2D_DESC desc = {0};
            desc.Width = state.Width;
            desc.Height = state.Height;
            desc.MipLevels = 1;
            desc.ArraySize = 1;
            desc.Format = DXGI_FORMAT_B8G8R8X8_UNORM;
            desc.SampleDesc.Count = 1;
            desc.SampleDesc.Quality = 0;
            desc.Usage = D3D11_USAGE_DEFAULT;
            desc.BindFlags =
                D3D11_BIND_SHADER_RESOURCE | D3D11_BIND_RENDER_TARGET;
            if (FAILED(device->CreateTexture2D(&desc, nullptr, &m_texture)))
            {
                return nullptr;
            }

            // create SRV
            if (FAILED(device->CreateShaderResourceView(m_texture.Get(),
                                                        nullptr, &m_srv)))
            {
                return nullptr;
            }
        }

        if (!FRAME_FACTORY_scene_render(m_scene, device, context,
                                        m_texture.Get(), &state))
        {
            return nullptr;
        }

        return m_srv.Get();
    }
};

struct ImGuiApp
{
    bool m_show_demo_window = true;
    float m_clearColor[4] = {0.0f, 0.2f, 0.4f, 1.0f};
    std::list<std::unique_ptr<RustRenderer>> m_scenes;

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
        // Check japanese (meiryo UI) font is exist on system.
        {
            // Create glyphs.
            io.Fonts->AddFontFromFileTTF("C:\\Windows\\Fonts\\meiryo.ttc",
                                         18.0f, nullptr,
                                         io.Fonts->GetGlyphRangesJapanese());
        }

        ImGui_ImplWin32_Init(hwnd);
        ImGui_ImplDX11_Init(device, context);

        // docking
        io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
    }

    ~ImGuiApp()
    {
        // Cleanup
        ImGui_ImplDX11_Shutdown();
        ImGui_ImplWin32_Shutdown();
        ImGui::DestroyContext();
    }

    void load(ID3D11Device *device, int argc, char **argv)
    {
        if (argc < 2)
        {
            if (auto scene = RustRenderer::load_sample(device))
            {
                m_scenes.push_back(std::move(scene));
            }
        }
        else
        {
            for (int i = 1; i < argc; ++i)
            {
                if (auto scene = RustRenderer::load_gltf(device, argv[i]))
                {
                    m_scenes.push_back(std::move(scene));
                }
            }
        }
    }

    void update(ID3D11Device *device, ID3D11DeviceContext *context,
                const screenstate::ScreenState &state)
    {
        // Start the Dear ImGui frame
        ImGui_ImplDX11_NewFrame();
        ImGui_ImplWin32_NewFrame();
        ImGui::NewFrame();

        gui(device, context, state);

        // end
        ImGui::Render();
    }

    void gui(ID3D11Device *device, ID3D11DeviceContext *context,
             const screenstate::ScreenState &state)
    {
        ImGuiWindowFlags window_flags =
            ImGuiWindowFlags_MenuBar | ImGuiWindowFlags_NoDocking;
        {
            const ImGuiViewport *viewport = ImGui::GetMainViewport();
            ImGui::SetNextWindowPos(viewport->WorkPos);
            ImGui::SetNextWindowSize(viewport->WorkSize);
            ImGui::SetNextWindowViewport(viewport->ID);
            ImGui::PushStyleVar(ImGuiStyleVar_WindowRounding, 0.0f);
            ImGui::PushStyleVar(ImGuiStyleVar_WindowBorderSize, 0.0f);
            window_flags |= ImGuiWindowFlags_NoTitleBar |
                            ImGuiWindowFlags_NoCollapse |
                            ImGuiWindowFlags_NoResize | ImGuiWindowFlags_NoMove;
            window_flags |= ImGuiWindowFlags_NoBringToFrontOnFocus |
                            ImGuiWindowFlags_NoNavFocus;
        }
        if (ImGui::Begin("Master Window", nullptr, window_flags))
        {
            // Declare Central dockspace
            ImGuiID dockspace_id = ImGui::GetID("MyDockSpace");
            ImGui::DockSpace(dockspace_id, ImVec2(0.0f, 0.0f),
                             ImGuiDockNodeFlags_None);
        }
        ImGui::End();
        ImGui::PopStyleVar(2);

        // demo
        if (m_show_demo_window)
        {
            ImGui::ShowDemoWindow(&m_show_demo_window);
        }

        // 3d view
        for (auto &scene : m_scenes)
        {
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
                auto crop = state.Crop(static_cast<int>(pos.x),
                                       static_cast<int>(pos.y + frameHeight),
                                       static_cast<int>(size.x),
                                       static_cast<int>(size.y));
                auto srv = scene->render(device, context, crop);
                if (srv)
                {
                    ImGui::ImageButton((ImTextureID)srv, size,
                                       ImVec2(0.0f, 0.0f), ImVec2(1.0f, 1.0f),
                                       0);
                }
            }
            ImGui::End();
            ImGui::PopStyleVar();
        }
    }

    void render()
    {
        ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
    }
};

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
    auto window = SampleWindow::create(CLASS_NAME, L"CPP_SAMPLE");
    if (!window)
    {
        return 1;
    }

    auto d3d = DX11::create(window->handle());
    if (!d3d)
    {
        return 2;
    }

    ImGuiApp gui(window->handle(), d3d->device().Get(), d3d->context().Get());
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
