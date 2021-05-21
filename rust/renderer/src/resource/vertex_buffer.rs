use crate::{
    com_util::ComCreate,
    scene::{AccessorBytes, Shader},
};
use com_ptr::ComPtr;
use winapi::{
    ctypes::c_void,
    shared::{dxgiformat, minwindef::UINT},
    um::{d3d11, d3dcommon},
};

use crate::com_util::ComError;

pub struct VertexBuffer {
    vertex_buffer: ComPtr<d3d11::ID3D11Buffer>,
    stride: u32,
    index_buffer: ComPtr<d3d11::ID3D11Buffer>,
    index_format: dxgiformat::DXGI_FORMAT,
    index_count: u32,
}

impl VertexBuffer {
    fn stride_to_format(stride: u32) -> dxgiformat::DXGI_FORMAT {
        match stride {
            2 => dxgiformat::DXGI_FORMAT_R16_UINT,
            4 => dxgiformat::DXGI_FORMAT_R32_UINT,
            _ => panic!(),
        }
    }

    pub fn new(
        vertex_buffer: ComPtr<d3d11::ID3D11Buffer>,
        stride: u32,
        index_buffer: ComPtr<d3d11::ID3D11Buffer>,
        index_stride: u32,
        index_count: u32,
    ) -> VertexBuffer {
        VertexBuffer {
            vertex_buffer,
            stride,
            index_buffer,
            index_format: Self::stride_to_format(index_stride),
            index_count,
        }
    }

    pub fn create_vertices<T>(
        d3d_device: &d3d11::ID3D11Device,
        vertices: &[T],
    ) -> Result<ComPtr<d3d11::ID3D11Buffer>, ComError> {
        let mut desc = d3d11::D3D11_BUFFER_DESC::default();
        desc.ByteWidth = std::mem::size_of_val(vertices) as u32;
        desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
        desc.BindFlags = d3d11::D3D11_BIND_VERTEX_BUFFER;
        desc.CPUAccessFlags = 0;
        let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
        data.pSysMem = vertices.as_ptr() as *mut c_void;
        ComPtr::create_if_success(|pp| unsafe { d3d_device.CreateBuffer(&desc, &data, pp) })
    }

    pub fn create_indices<T>(
        d3d_device: &d3d11::ID3D11Device,
        indices: &[T],
    ) -> Result<ComPtr<d3d11::ID3D11Buffer>, ComError> {
        let mut desc = d3d11::D3D11_BUFFER_DESC::default();
        desc.ByteWidth = std::mem::size_of_val(indices) as u32;
        desc.Usage = d3d11::D3D11_USAGE_DEFAULT;
        desc.BindFlags = d3d11::D3D11_BIND_INDEX_BUFFER;
        desc.CPUAccessFlags = 0;
        let mut data = d3d11::D3D11_SUBRESOURCE_DATA::default();
        data.pSysMem = indices.as_ptr() as *mut c_void;
        ComPtr::create_if_success(|pp| unsafe { d3d_device.CreateBuffer(&desc, &data, pp) })
    }

    pub fn from(
        d3d_device: &d3d11::ID3D11Device,
        positions: &AccessorBytes,
        indices: &AccessorBytes,
    ) -> Result<VertexBuffer, ComError> {
        let vertex_buffer = Self::create_vertices(d3d_device, &positions.bytes)?;
        let index_buffer = Self::create_indices(d3d_device, &indices.bytes)?;

        Ok(VertexBuffer {
            vertex_buffer,
            stride: positions.stride,
            index_buffer,
            index_format: Self::stride_to_format(indices.stride),
            index_count: indices.count,
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

            d3d_context.IASetIndexBuffer(self.index_buffer.as_ptr(), self.index_format, 0);

            d3d_context.IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            d3d_context.DrawIndexed(self.index_count as UINT, 0, 0);
        }
    }
}
