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
