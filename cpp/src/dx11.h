#pragma once
#include <wrl/client.h>
#include <memory>

class DX11
{
    Microsoft::WRL::ComPtr<struct ID3D11Device> m_device;
    Microsoft::WRL::ComPtr<struct ID3D11DeviceContext> m_context;
    Microsoft::WRL::ComPtr<struct IDXGISwapChain> m_swapchain;
    Microsoft::WRL::ComPtr<struct ID3D11RenderTargetView> m_rtv;
    UINT m_width = 0;
    UINT m_height = 0;

    DX11()
    {
    }

public:
    ~DX11()
    {
    }

    static std::shared_ptr<DX11> create(HWND hwnd);

    const Microsoft::WRL::ComPtr<ID3D11Device> &device() const
    {
        return m_device;
    }

    const Microsoft::WRL::ComPtr<ID3D11DeviceContext> &context() const
    {
        return m_context;
    }

    auto new_frame(UINT width, UINT height, const float clearColor[4]) -> bool;

    void flush();
};
