use std::{collections::HashMap, ffi::CStr};

use crate::com_util::ComCreate;
use crate::scene::AccessorBytes;
use com_ptr::ComPtr;
use winapi::{
    ctypes::c_void,
    shared::dxgiformat,
    um::{d3d11, d3dcommon},
};

use crate::com_util::ComError;

pub struct BufferArray {
    vertex_buffers: Vec<ComPtr<d3d11::ID3D11Buffer>>,
    vertex_strides: Vec<u32>,
    vertex_offsets: Vec<u32>,
    index_buffer: ComPtr<d3d11::ID3D11Buffer>,
    index_format: dxgiformat::DXGI_FORMAT,
    _index_count: u32,
}

impl BufferArray {
    fn stride_to_format(stride: u32) -> dxgiformat::DXGI_FORMAT {
        match stride {
            2 => dxgiformat::DXGI_FORMAT_R16_UINT,
            4 => dxgiformat::DXGI_FORMAT_R32_UINT,
            _ => panic!(),
        }
    }

    pub fn new(
        index_buffer: ComPtr<d3d11::ID3D11Buffer>,
        index_stride: u32,
        index_count: u32,
    ) -> BufferArray {
        BufferArray {
            vertex_buffers: Vec::new(),
            vertex_strides: Vec::new(),
            vertex_offsets: Vec::new(),
            index_buffer,
            index_format: Self::stride_to_format(index_stride),
            _index_count: index_count,
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
        indices: &AccessorBytes,
        vertices: &HashMap<String, AccessorBytes>,
        elements: &[d3d11::D3D11_INPUT_ELEMENT_DESC],
    ) -> Result<BufferArray, ComError> {
        let index_buffer = Self::create_indices(d3d_device, &indices.bytes)?;

        let mut buffer_array = Self::new(
            index_buffer,
            Self::stride_to_format(indices.stride),
            indices.count,
        );

        for element in elements {
            let v = &vertices[unsafe { CStr::from_ptr(element.SemanticName).to_str().unwrap() }];
            let vertex_buffer = Self::create_vertices(d3d_device, &v.bytes)?;
            buffer_array.vertex_buffers.push(vertex_buffer);
            buffer_array.vertex_strides.push(v.stride);
            buffer_array.vertex_offsets.push(0);
        }

        Ok(buffer_array)
    }

    pub fn prepare(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
        let buffers: Vec<*mut d3d11::ID3D11Buffer> =
            self.vertex_buffers.iter().map(|p| p.as_ptr()).collect();

        unsafe {
            d3d_context.IASetVertexBuffers(
                0,
                self.vertex_buffers.len() as u32,
                buffers.as_ptr(),
                self.vertex_strides.as_ptr(),
                self.vertex_offsets.as_ptr(),
            );

            d3d_context.IASetIndexBuffer(self.index_buffer.as_ptr(), self.index_format, 0);

            d3d_context.IASetPrimitiveTopology(d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        }
    }

    pub fn draw(&self, d3d_context: &d3d11::ID3D11DeviceContext, index_count: u32, offset: u32) {
        unsafe { d3d_context.DrawIndexed(index_count, offset, 0) };
    }
}
