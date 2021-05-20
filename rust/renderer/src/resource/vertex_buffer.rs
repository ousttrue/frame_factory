use std::ptr;

use com_ptr::ComPtr;
use winapi::{
    ctypes::c_void,
    shared::{dxgiformat, minwindef::UINT},
    um::{d3d11, d3dcommon},
};

use crate::com_util::ComError;

pub struct VertexBuffer {
    vertex_buffer: ComPtr<d3d11::ID3D11Buffer>,
    stride: i32,
    index_buffer: ComPtr<d3d11::ID3D11Buffer>,
    index_count: i32,
}

impl VertexBuffer {
    /// clock wise
    ///    2
    ///   1 0
    pub fn create_triangle(d3d_device: &d3d11::ID3D11Device) -> Result<VertexBuffer, ComError> {
        // vertices
        let size = 0.5f32;
        let positions = [
            size, -size, 0.0f32, -size, -size, 0.0f32, 0.0f32, size, 0.0f32,
        ];
        let mut desc = d3d11::D3D11_BUFFER_DESC::default();
        desc.ByteWidth = std::mem::size_of_val(&positions) as u32;
        desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
        desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
        desc.CPUAccessFlags = 0;
        let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
        data.pSysMem = positions.as_ptr() as *mut c_void;
        let mut vertex_buffer: *mut d3d11::ID3D11Buffer = ptr::null_mut();
        let hr = unsafe { d3d_device.CreateBuffer(&desc, &data, &mut vertex_buffer) };
        if hr != 0 {
            return Err(ComError::StaticMessage("CreateBuffer vertex_buffer"));
        }
        let vertex_buffer = unsafe { ComPtr::from_raw(vertex_buffer) };

        // indices
        let indices = [0, 1, 2];
        let mut desc = d3d11::D3D11_BUFFER_DESC::default();
        desc.ByteWidth = std::mem::size_of_val(&indices) as u32;
        desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
        desc.BindFlags = d3d11::D3D11_BIND_INDEX_BUFFER;
        desc.CPUAccessFlags = 0;
        let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
        data.pSysMem = indices.as_ptr() as *mut c_void;
        let mut index_buffer: *mut d3d11::ID3D11Buffer = ptr::null_mut();
        let hr = unsafe { d3d_device.CreateBuffer(&desc, &data, &mut index_buffer) };
        if hr != 0 {
            return Err(ComError::StaticMessage("CreateBuffer index_buffer"));
        }
        let index_buffer = unsafe { ComPtr::from_raw(index_buffer) };

        Ok(VertexBuffer {
            vertex_buffer,
            stride: 12,
            index_buffer,
            index_count: 3,
        })
    }

    pub fn draw(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        let buffers = [self.vertex_buffer.as_ptr()];
        let strides = [self.stride as UINT];
        let offsets = [0 as UINT];

        unsafe {
            d3d_context.IASetVertexBuffers(
                0,
                1,
                buffers.as_ptr(),
                strides.as_ptr(),
                offsets.as_ptr(),
            );

            d3d_context.IASetIndexBuffer(
                self.index_buffer.as_ptr(),
                dxgiformat::DXGI_FORMAT_R32_UINT,
                0,
            );

            d3d_context.IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            d3d_context.DrawIndexed(self.index_count as UINT, 0, 0);
        }
    }
}
