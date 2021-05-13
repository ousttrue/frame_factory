#include "dx11.h"
#include <d3d11.h>

std::shared_ptr<DX11> DX11::create(HWND hwnd)
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

auto DX11::new_frame(UINT width, UINT height, const float clearColor[4]) -> bool
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

        if (FAILED(m_device->CreateRenderTargetView(backBuffer.Get(), nullptr,
                                                    &m_rtv)))
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

void DX11::flush()
{
    m_swapchain->Present(0, 0);
}

struct RenderTarget
{
    int width;
    int height;
    Microsoft::WRL::ComPtr<ID3D11Texture2D> texture;
    Microsoft::WRL::ComPtr<ID3D11RenderTargetView> rtv;
    Microsoft::WRL::ComPtr<ID3D11ShaderResourceView> srv;

    static std::shared_ptr<RenderTarget> create(ID3D11Device *device, int w,
                                                int h)
    {
        if (w <= 0 || h <= 0)
        {
            return nullptr;
        }

        auto p = std::make_shared<RenderTarget>();
        p->width = w;
        p->height = h;

        D3D11_TEXTURE2D_DESC desc = {0};
        desc.Width = w;
        desc.Height = h;
        desc.MipLevels = 1;
        desc.ArraySize = 1;
        desc.Format = DXGI_FORMAT_B8G8R8X8_UNORM;
        desc.SampleDesc.Count = 1;
        desc.SampleDesc.Quality = 0;
        desc.Usage = D3D11_USAGE_DEFAULT;
        desc.BindFlags = D3D11_BIND_SHADER_RESOURCE | D3D11_BIND_RENDER_TARGET;
        if (FAILED(device->CreateTexture2D(&desc, nullptr, &p->texture)))
        {
            return nullptr;
        }

        // create RTV
        if (FAILED(device->CreateRenderTargetView(p->texture.Get(), nullptr,
                                                  &p->rtv)))
        {
            return nullptr;
        }

        // create SRV
        if (FAILED(device->CreateShaderResourceView(p->texture.Get(), nullptr,
                                                    &p->srv)))
        {
            return nullptr;
        }

        return p;
    }

    void set_and_clear_rtv(ID3D11DeviceContext *context)
    {
        if (rtv)
        {
            float clear[] = {0.2f, 0.2f, 0.2f, 1.0f};
            context->ClearRenderTargetView(rtv.Get(), clear);
        }

        // set backbuffer & depthbuffer
        ID3D11RenderTargetView *rtv_list[] = {rtv.Get()};
        context->OMSetRenderTargets(1, rtv_list, nullptr);
        D3D11_VIEWPORT viewports[] = {
            {
                .TopLeftX = 0,
                .TopLeftY = 0,
                .Width = (float)width,
                .Height = (float)height,
                .MinDepth = 0,
                .MaxDepth = 1.0f,
            },
        };
        context->RSSetViewports(_countof(viewports), viewports);
    }
};

ID3D11ShaderResourceView *DX11::setup_render_target(int w, int h)
{
    if (m_render_target &&
        (m_render_target->width != w || m_render_target->height != h))
    {
        // clear
        m_render_target = nullptr;
    }

    if (!m_render_target)
    {
        m_render_target = RenderTarget::create(m_device.Get(), w, h);
        if (!m_render_target)
        {
            return nullptr;
        }
    }

    m_render_target->set_and_clear_rtv(m_context.Get());
    return m_render_target->srv.Get();
}
