#include "window_util.h"
#include "extern_renderer.h"
#include "dx11.h"
#include <Windows.h>
#include <vector>
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

public:
    RustRenderer(ID3D11Device *device, const char *data, size_t len)
    {
        m_scene =
            FRAME_FACTORY_sample_create(device, data, len, "vsMain", "psMain");
    }

    ~RustRenderer()
    {
        FRAME_FACTORY_sample_destroy();
    }

    ID3D11ShaderResourceView *render(ID3D11Device *device,
                                     ID3D11DeviceContext *context, int w, int h)
    {
        if (m_width != w || m_height != h)
        {
            m_texture = nullptr;
            m_srv = nullptr;
        }

        if (!m_texture)
        {
            if (w == 0 || h == 0)
            {
                return nullptr;
            }
            m_width = w;
            m_height = h;

            D3D11_TEXTURE2D_DESC desc = {0};
            desc.Width = w;
            desc.Height = h;
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

        if (!FRAME_FACTORY_sample_render(m_scene, device, context,
                                         m_texture.Get()))
        {
            return nullptr;
        }

        return m_srv.Get();
    }
};

class ImGuiApp
{
    bool m_show_demo_window = true;
    // 3D View
    std::unique_ptr<RustRenderer> m_scene;

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

        auto source = ReadAllBytes("../shaders/mvp.hlsl");
        if (!source.empty())
        {
            m_scene.reset(
                new RustRenderer(device, source.data(), source.size()));
        }
    }

    ~ImGuiApp()
    {
        // Cleanup
        ImGui_ImplDX11_Shutdown();
        ImGui_ImplWin32_Shutdown();
        ImGui::DestroyContext();
    }

    void update(ID3D11Device *device, ID3D11DeviceContext *context)
    {
        // Start the Dear ImGui frame
        ImGui_ImplDX11_NewFrame();
        ImGui_ImplWin32_NewFrame();
        ImGui::NewFrame();

        gui(device, context);

        // end
        ImGui::Render();
    }

    void gui(ID3D11Device *device, ID3D11DeviceContext *context)
    {
        // demo
        if (m_show_demo_window)
        {
            ImGui::ShowDemoWindow(&m_show_demo_window);
        }

        // 3d view
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
                auto x = mouseXY.x - pos.x;
                auto y = mouseXY.y - pos.y - frameHeight;

                auto srv =
                    m_scene->render(device, context, (int)size.x, (int)size.y);
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
    UINT m_lastTime = 0;
    UINT m_frameTime = 0;

public:
    FPS(int frame_rate)
    {
        m_frameTime = 1000 / frame_rate;
    }

    void wait()
    {
        auto now = timeGetTime();
        if (m_lastTime)
        {
            auto delta = now - m_lastTime;
            if (delta < m_frameTime)
            {
                Sleep(m_frameTime - delta);
            }
        }
        m_lastTime = now;
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

        FPS fps(60);

        float clearColor[] = {0.0f, 0.2f, 0.4f, 1.0f};
        {
            ImGuiApp gui(window->handle(), d3d->device().Get(),
                         d3d->context().Get());
            for (WindowState state = {}; !state.closed;
                 state = window->main_loop())
            {
                // update imgui
                gui.update(d3d->device().Get(), d3d->context().Get());

                // render d3d
                if (!d3d->new_frame(state.width, state.height, clearColor))
                {
                    return 4;
                }
                gui.render();

                fps.wait();

                d3d->flush();
            }
        }
    }

    return 0;
}
