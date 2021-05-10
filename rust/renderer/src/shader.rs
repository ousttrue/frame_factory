use crate::com_util::{ComCreate, ComError};
use com_ptr::ComPtr;
use std::{ffi::CStr, ptr};
use winapi::{ctypes::c_void, shared::ntdef::HRESULT, Interface};
use winapi::{
    shared::dxgiformat,
    um::{d3d11, d3dcommon, d3dcompiler, winnt::LPCSTR},
};
use winapi::{
    shared::minwindef::*,
    um::{d3d11shader, d3dcompiler::D3DReflect},
};

unsafe fn to_string(blob: &ComPtr<d3dcommon::ID3DBlob>) -> String {
    let buf = blob.GetBufferPointer() as *mut u8;
    String::from_raw_parts(buf, blob.GetBufferSize(), blob.GetBufferSize())
}

unsafe fn compile_shader_from_source(
    filename: &str,
    source: *const u8,
    source_size: usize,
    entry_point: &str,
    shader_model: &str,
) -> Result<*mut d3dcommon::ID3DBlob, ComError> {
    let shader_flags = d3dcompiler::D3DCOMPILE_ENABLE_STRICTNESS;

    // #if defined(DEBUG) || defined(_DEBUG)
    // 	dwShaderFlags |= D3DCOMPILE_DEBUG;
    // #endif//defiend(DEBUG) || defined(_DEBUG)

    // #if defined(NDEBUG) || defined(_NDEBUG)
    // 	dwShaderFlags |= D3DCOMPILE_OPTIMIZATION_LEVEL3;
    // #endif//defined(NDEBUG) || defined(_NDEBUG)

    let mut blob: *mut d3dcommon::ID3DBlob = ptr::null_mut();
    let mut error: *mut d3dcommon::ID3DBlob = ptr::null_mut();
    let hr = d3dcompiler::D3DCompile(
        source as LPCVOID,
        source_size,
        filename.as_ptr() as LPCSTR,
        ptr::null(),
        d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE,
        entry_point.as_ptr() as LPCSTR,
        shader_model.as_ptr() as LPCSTR,
        shader_flags,
        0,
        &mut blob,
        &mut error,
    );
    if hr != 0 {
        let error = ComPtr::from_raw(error);
        let message = to_string(&error);
        return Err(ComError::Message(message));
    }

    Ok(blob)
}

fn to_dxgi_format(
    value_type: d3dcommon::D3D_REGISTER_COMPONENT_TYPE,
    mask: u8,
) -> dxgiformat::DXGI_FORMAT {
    if value_type == d3dcommon::D3D_REGISTER_COMPONENT_FLOAT32 {
        if (mask & 0x0F) == 0x0F {
            return dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT;
        } else if (mask & 0x07) == 0x07 {
            return dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT;
        } else if (mask & 0x03) == 0x03 {
            return dxgiformat::DXGI_FORMAT_R32G32_FLOAT;
        } else if (mask & 0x1) == 0x1 {
            return dxgiformat::DXGI_FORMAT_R32_FLOAT;
        }
    }

    dxgiformat::DXGI_FORMAT_UNKNOWN
}

unsafe fn create_reflection(
    compiled_vertex_shader: &ComPtr<d3dcommon::ID3DBlob>,
    reflection: *mut *mut d3d11shader::ID3D11ShaderReflection,
) -> HRESULT {
    D3DReflect(
        compiled_vertex_shader.GetBufferPointer(),
        compiled_vertex_shader.GetBufferSize(),
        &d3d11shader::ID3D11ShaderReflection::uuidof(),
        reflection as *mut *mut c_void,
    )
}

unsafe fn create_input_layout(
    d3d_device: &ComPtr<d3d11::ID3D11Device>,
    compiled_vertex_shader: &ComPtr<d3dcommon::ID3DBlob>,
    reflection: &ComPtr<d3d11shader::ID3D11ShaderReflection>,
    input_layout: *mut *mut d3d11::ID3D11InputLayout,
) -> HRESULT {
    // Create InputLayout
    let mut shaderdesc = d3d11shader::D3D11_SHADER_DESC::default();
    reflection.GetDesc(&mut shaderdesc);

    let mut elements: Vec<d3d11::D3D11_INPUT_ELEMENT_DESC> = vec![];
    for i in 0..shaderdesc.InputParameters {
        let mut param_desc = d3d11shader::D3D11_SIGNATURE_PARAMETER_DESC::default();
        reflection.GetInputParameterDesc(i, &mut param_desc);
        let format = to_dxgi_format(param_desc.ComponentType, param_desc.Mask);
        let element = d3d11::D3D11_INPUT_ELEMENT_DESC {
            SemanticName: param_desc.SemanticName,
            SemanticIndex: param_desc.SemanticIndex,
            Format: format,
            InputSlot: 0,
            AlignedByteOffset: d3d11::D3D11_APPEND_ALIGNED_ELEMENT,
            InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        };
        elements.push(element);
    }

    if elements.len() == 0 {
        return -1;
    }

    d3d_device.CreateInputLayout(
        elements.as_ptr(),
        elements.len() as UINT,
        compiled_vertex_shader.GetBufferPointer(),
        compiled_vertex_shader.GetBufferSize(),
        input_layout,
    )
}

pub struct ConstantBufferSlot {
    pub name: String,
    pub buffer: ComPtr<d3d11::ID3D11Buffer>,
}

impl ConstantBufferSlot {
    pub fn new(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        shader_desc: d3d11shader::D3D11_SHADER_BUFFER_DESC,
    ) -> Result<ConstantBufferSlot, ComError> {
        let desc = d3d11::D3D11_BUFFER_DESC {
            ByteWidth: shader_desc.Size,
            Usage: d3d11::D3D11_USAGE_DEFAULT,
            BindFlags: d3d11::D3D11_BIND_CONSTANT_BUFFER,
            ..Default::default()
        };

        let buffer = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateBuffer(&desc, ptr::null_mut(), pp)
        })?;

        Ok(ConstantBufferSlot {
            name: unsafe { String::from(CStr::from_ptr(shader_desc.Name).to_str().unwrap()) },
            buffer,
        })
    }
}

pub struct ConstantBufferShader {
    pub slots: Vec<Box<ConstantBufferSlot>>,
}

impl ConstantBufferShader {
    unsafe fn new(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        reflection: &ComPtr<d3d11shader::ID3D11ShaderReflection>,
    ) -> Result<ConstantBufferShader, ComError> {
        let mut shader_desc = d3d11shader::D3D11_SHADER_DESC::default();
        reflection.GetDesc(&mut shader_desc);

        let mut constant_buffer = ConstantBufferShader { slots: vec![] };

        // analize constant buffer
        for i in 0..shader_desc.ConstantBuffers {
            let cb = reflection.GetConstantBufferByIndex(i).as_ref().unwrap();
            let mut desc = d3d11shader::D3D11_SHADER_BUFFER_DESC::default();
            cb.GetDesc(&mut desc);
            println!(
                "[{}: {}] {} bytes",
                i,
                CStr::from_ptr(desc.Name).to_str().unwrap(),
                desc.Size
            );

            let buffer = Box::new(ConstantBufferSlot::new(d3d_device, desc)?);
            constant_buffer.slots.push(buffer);

            // impl->AddCBSlot(pDevice, desc.Size);
            for j in 0..desc.Variables {
                let v = cb.GetVariableByIndex(j).as_ref().unwrap();
                let mut var = d3d11shader::D3D11_SHADER_VARIABLE_DESC::default();
                v.GetDesc(&mut var);
                println!(
                    "({}) {} {} {} bytes",
                    j,
                    CStr::from_ptr(var.Name).to_str().unwrap(),
                    var.StartOffset,
                    var.Size
                );
                // impl->AddCBVariable(i, vdesc);
            }
        }

        Ok(constant_buffer)
    }

    pub fn update<T>(
        &self,
        d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>,
        slot_index: usize,
        data: *const T,
    ) {
        if let Some(slot) = self.slots.get(slot_index) {
            let slot = slot.as_ref();
            let p: *mut d3d11::ID3D11Buffer = slot.buffer.as_ptr();
            unsafe {
                d3d_context.UpdateSubresource(
                    p as *mut d3d11::ID3D11Resource,
                    0,
                    ptr::null_mut(),
                    data as *const c_void,
                    0,
                    0,
                )
            };
        }
    }

    pub fn set_vs(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
        for (i, slot) in self.slots.iter().enumerate() {
            unsafe {
                d3d_context.VSSetConstantBuffers(i as u32, 1, &mut slot.buffer.as_ptr());
            };
        }
    }
}

pub struct Shader {
    pub vs: ComPtr<d3d11::ID3D11VertexShader>,
    pub vs_constant_buffer: ConstantBufferShader,
    pub ps: ComPtr<d3d11::ID3D11PixelShader>,
    pub input_layout: ComPtr<d3d11::ID3D11InputLayout>,
}

impl Shader {
    pub fn compile_vertex_shader(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        source: *const u8,
        source_size: usize,
        entry_point: &str,
    ) -> Result<
        (
            ComPtr<d3d11::ID3D11VertexShader>,
            ComPtr<d3d11::ID3D11InputLayout>,
            ConstantBufferShader,
        ),
        ComError,
    > {
        let compiled_vertex_shader = ComPtr::new(|| unsafe {
            compile_shader_from_source("vs\0", source, source_size, entry_point, "vs_4_0\0")
        })?;

        let shader = ComPtr::create_if_success(|pp| unsafe {
            d3d_device.CreateVertexShader(
                compiled_vertex_shader.GetBufferPointer(),
                compiled_vertex_shader.GetBufferSize(),
                ptr::null_mut(),
                pp,
            )
        })?;

        let reflection = ComPtr::create_if_success(|pp| unsafe {
            create_reflection(&compiled_vertex_shader, pp)
        })?;

        let input_layout = ComPtr::create_if_success(|pp| unsafe {
            create_input_layout(d3d_device, &compiled_vertex_shader, &reflection, pp)
        })?;

        let constant_buffer = unsafe { ConstantBufferShader::new(d3d_device, &reflection)? };

        Ok((shader, input_layout, constant_buffer))
    }

    pub fn compile_pixel_shader(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        source: *const u8,
        source_size: usize,
        entry_point: &str,
    ) -> Result<ComPtr<d3d11::ID3D11PixelShader>, ComError> {
        let compiled = ComPtr::new(|| unsafe {
            compile_shader_from_source("ps\0", source, source_size, entry_point, "ps_4_0\0")
        })?;

        let mut shader: *mut d3d11::ID3D11PixelShader = ptr::null_mut();
        let hr = unsafe {
            d3d_device.CreatePixelShader(
                compiled.GetBufferPointer(),
                compiled.GetBufferSize(),
                ptr::null_mut(),
                &mut shader,
            )
        };
        if hr != 0 {
            return Err(ComError::StaticMessage("fail to CreatePixelShader"));
        }

        unsafe { Ok(ComPtr::from_raw(shader)) }
    }

    pub fn set(&self, d3d_context: &ComPtr<d3d11::ID3D11DeviceContext>) {
        unsafe {
            // VS
            d3d_context.VSSetShader(self.vs.as_ptr(), ptr::null(), 0);
            self.vs_constant_buffer.set_vs(d3d_context);

            // PS
            d3d_context.PSSetShader(self.ps.as_ptr(), ptr::null(), 0);

            // IA InputLayout
            d3d_context.IASetInputLayout(self.input_layout.as_ptr());
        }
    }
}
