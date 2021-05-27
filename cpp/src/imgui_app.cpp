#include "imgui_app.h"
#include "extern_renderer.h"
#include <stdint.h>
#include <wrl/client.h>
#include <string>
#include <sstream>
#include <string_view>
#include <filesystem>
#include <imgui.h>
#include <backends/imgui_impl_win32.h>
#include <backends/imgui_impl_dx11.h>

static std::string get_file_name(std::string_view src)
{
    std::filesystem::path p = src;
    auto name = p.filename().string();
    return name;
}

class RustRenderer
{
    uint32_t m_scene = 0;

    int m_width = 0;
    int m_height = 0;
    Microsoft::WRL::ComPtr<ID3D11Texture2D> m_texture;
    Microsoft::WRL::ComPtr<ID3D11ShaderResourceView> m_srv;
    std::string m_name;

    static int s_next_id;

    RustRenderer(uint32_t scene, const std::string_view &name)
        : m_scene(scene), m_name(name)
    {
    }

public:
    ~RustRenderer()
    {
        FRAME_FACTORY_shutdown();
    }

    static std::unique_ptr<RustRenderer> load_gltf(const char *path)
    {
        auto scene = FRAME_FACTORY_scene_load(path);
        if (!scene)
        {
            return nullptr;
        }

        std::stringstream ss;

        ss << get_file_name(path) << "##view:" << s_next_id++;

        return std::unique_ptr<RustRenderer>(new RustRenderer(scene, ss.str()));
    }

    const std::string &name() const
    {
        return m_name;
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

int RustRenderer::s_next_id = 1;

ImGuiApp::ImGuiApp(HWND hwnd, ID3D11Device *device,
                   ID3D11DeviceContext *context)
    : m_hwnd(hwnd)
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
        io.Fonts->AddFontFromFileTTF("C:\\Windows\\Fonts\\meiryo.ttc", 18.0f,
                                     nullptr,
                                     io.Fonts->GetGlyphRangesJapanese());
    }

    ImGui_ImplWin32_Init(hwnd);
    ImGui_ImplDX11_Init(device, context);

    // docking
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;
}

ImGuiApp::~ImGuiApp()
{
    // Cleanup
    ImGui_ImplDX11_Shutdown();
    ImGui_ImplWin32_Shutdown();
    ImGui::DestroyContext();
}

// 0: exe
// 1: assets path
// 2..: scenes
void ImGuiApp::load(ID3D11Device *device, int argc, char **argv)
{
    FRAME_FACTORY_asset_path(argv[1]);

    if (argc >= 3)
    {
        for (int i = 2; i < argc; ++i)
        {
            if (auto scene = RustRenderer::load_gltf(argv[i]))
            {
                m_scenes.push_back(std::move(scene));
            }
        }
    }
}

void ImGuiApp::update(ID3D11Device *device, ID3D11DeviceContext *context,
                      const screenstate::ScreenState &state)
{
    // log message
    for (int i = 0; true; ++i)
    {
        uint32_t size;
        auto msg = FRAME_FACTORY_message_peek(i, &size);
        if (!msg)
        {
            FRAME_FACTORY_message_clear();
            break;
        }
        m_messages.push_back(std::string(msg, msg + size));
    }

    // Start the Dear ImGui frame
    ImGui_ImplDX11_NewFrame();
    ImGui_ImplWin32_NewFrame();
    ImGui::NewFrame();

    gui(device, context, state);

    // end
    ImGui::Render();
}

void ImGuiApp::gui(ID3D11Device *device, ID3D11DeviceContext *context,
                   const screenstate::ScreenState &state)
{
    // docking space
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

        if (ImGui::BeginMenuBar())
        {
            if (ImGui::BeginMenu("File"))
            {
                if (ImGui::MenuItem("Open", "Ctrl+O"))
                {
                    char strCustom[256] = {0};
                    char strFile[MAX_PATH] = {0};
                    OPENFILENAMEA ofn = {0};
                    ofn.lStructSize = sizeof(OPENFILENAME);
                    ofn.hwndOwner = m_hwnd;
                    ofn.lpstrFilter = "glb files {*.glb}\0*.glb\0"
                                      "All files {*.*}\0*.*\0"
                                      "\0";
                    ofn.lpstrCustomFilter = strCustom;
                    ofn.nMaxCustFilter = sizeof(strCustom);
                    ofn.nFilterIndex = 0;
                    ofn.lpstrFile = strFile;
                    ofn.nMaxFile = MAX_PATH;
                    ofn.Flags = OFN_FILEMUSTEXIST;
                    if (GetOpenFileNameA(&ofn))
                    {
                        if (auto scene = RustRenderer::load_gltf(ofn.lpstrFile))
                        {
                            m_scenes.push_back(std::move(scene));
                        }
                    }
                }
                ImGui::EndMenu();
            }
            ImGui::EndMenuBar();
        }
        ImGui::End();
        ImGui::PopStyleVar(2);
    }

    // demo
    if (m_show_demo_window)
    {
        ImGui::ShowDemoWindow(&m_show_demo_window);
    }

    // log
    if (ImGui::Begin("log messages"))
    {
        for (auto &msg : m_messages)
        {
            ImGui::TextUnformatted(msg.data(), msg.data() + msg.size());
        }
        ImGui::End();
    }

    // 3d view
    for (auto &scene : m_scenes)
    {
        ImGui::PushStyleVar(ImGuiStyleVar_WindowPadding, ImVec2(0, 0));
        ImGui::SetNextWindowSize(ImVec2(512, 512), ImGuiCond_Once);
        if (ImGui::Begin(scene->name().c_str(), nullptr,
                         ImGuiWindowFlags_NoScrollbar |
                             ImGuiWindowFlags_NoScrollWithMouse))
        {
            auto size = ImGui::GetContentRegionAvail();
            auto pos = ImGui::GetWindowPos();
            auto frameHeight = ImGui::GetFrameHeight();

            // render
            auto crop = state.Crop(
                static_cast<int>(pos.x), static_cast<int>(pos.y + frameHeight),
                static_cast<int>(size.x), static_cast<int>(size.y));
            auto srv = scene->render(device, context, crop);
            if (srv)
            {
                ImGui::ImageButton((ImTextureID)srv, size, ImVec2(0.0f, 0.0f),
                                   ImVec2(1.0f, 1.0f), 0);
            }
        }
        ImGui::End();
        ImGui::PopStyleVar();
    }
}

void ImGuiApp::render()
{
    ImGui_ImplDX11_RenderDrawData(ImGui::GetDrawData());
}
