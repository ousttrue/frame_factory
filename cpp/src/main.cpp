#include "window_util.h"
#include "extern_renderer.h"
#include <Windows.h>
#include <vector>
#include <fstream>
#include <functional>
#include <wrl/client.h>
#include <imgui.h>
#include <wrl/client.h>
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

class DX11
{
    Microsoft::WRL::ComPtr<ID3D11Device> m_device;
    Microsoft::WRL::ComPtr<ID3D11DeviceContext> m_context;
    Microsoft::WRL::ComPtr<IDXGISwapChain> m_swapchain;
    Microsoft::WRL::ComPtr<ID3D11RenderTargetView> m_rtv;
    UINT m_width = 0;
    UINT m_height = 0;

    DX11()
    {
    }

public:
    ~DX11()
    {
    }

    static std::shared_ptr<DX11> create(HWND hwnd)
    {
        DXGI_SWAP_CHAIN_DESC swapChainDesc = {0};
        swapChainDesc.OutputWindow = hwnd;
        swapChainDesc.Windowed = TRUE;
        swapChainDesc.SwapEffect = DXGI_SWAP_EFFECT_FLIP_DISCARD;
        swapChainDesc.BufferCount = 2;
        swapChainDesc.BufferUsage = DXGI_USAGE_RENDER_TARGET_OUTPUT;
        swapChainDesc.BufferDesc.Format = DXGI_FORMAT_R8G8B8A8_UNORM;
        swapChainDesc.BufferDesc.RefreshRate.Numerator = 60;
        swapChainDesc.BufferDesc.RefreshRate.Denominator = 1;
        swapChainDesc.Flags = DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH;
        swapChainDesc.SampleDesc.Count = 1;

        UINT createDeviceFlags = 0;
#if DEBUG
        createDeviceFlags |= D3D11_CREATE_DEVICE_DEBUG;
#endif

        auto dx11 = std::shared_ptr<DX11>(new DX11);

        D3D_FEATURE_LEVEL featureLevel;
        const D3D_FEATURE_LEVEL featureLevelArray[] = {
            D3D_FEATURE_LEVEL_11_0,
        };
        if (FAILED(D3D11CreateDeviceAndSwapChain(
                nullptr, D3D_DRIVER_TYPE_HARDWARE, nullptr, createDeviceFlags,
                featureLevelArray, 1, D3D11_SDK_VERSION, &swapChainDesc,
                &dx11->m_swapchain, &dx11->m_device, &featureLevel,
                &dx11->m_context)))
        {
            // LOGE << "Error initializing DirectX";
            return nullptr;
        }

        DXGI_SWAP_CHAIN_DESC desc;
        dx11->m_swapchain->GetDesc(&desc);
        dx11->m_width = desc.BufferDesc.Width;
        dx11->m_height = desc.BufferDesc.Height;

        return dx11;
    }

    const Microsoft::WRL::ComPtr<ID3D11Device> &device() const
    {
        return m_device;
    }

    const Microsoft::WRL::ComPtr<ID3D11DeviceContext> &context() const
    {
        return m_context;
    }

    auto new_frame(UINT width, UINT height, const float clearColor[4])
    {
        if (width != m_width || height != m_height)
        {
            // clear
            m_rtv = nullptr;
            m_context->OMSetRenderTargets(0, nullptr, nullptr);

            // resize swapchain
            DXGI_SWAP_CHAIN_DESC desc;
            m_swapchain->GetDesc(&desc);
            m_swapchain->ResizeBuffers(desc.BufferCount, width, height,
                                       desc.BufferDesc.Format, desc.Flags);

            m_width = width;
            m_height = height;
        }

        if (!m_rtv)
        {
            // get backbuffer
            Microsoft::WRL::ComPtr<ID3D11Texture2D> backBuffer;
            if (FAILED(m_swapchain->GetBuffer(0, IID_PPV_ARGS(&backBuffer))))
            {
                // fatal
                // LOGE << "fail to get backbuffer";
                return false;
            }

            if (FAILED(m_device->CreateRenderTargetView(backBuffer.Get(),
                                                        nullptr, &m_rtv)))
            {
                return false;
            }
        }

        // clear
        m_context->ClearRenderTargetView(m_rtv.Get(), clearColor);

        // set render target
        ID3D11RenderTargetView *targets[1] = {m_rtv.Get()};
        m_context->OMSetRenderTargets(1, targets, nullptr);

        // viewport
        D3D11_VIEWPORT viewport = {
            .Width = (float)width,
            .Height = (float)height,
        };
        m_context->RSSetViewports(1, &viewport);

        return true;
    }

    void flush()
    {
        m_swapchain->Present(0, 0);
    }
};

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

    ID3D11ShaderResourceView *render(ID3D11Device *device,
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
                    return rust.render(d3d->device().Get(),
                                       d3d->context().Get(), w, h);
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
