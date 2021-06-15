use crate::{
    asset_manager::ShaderSource,
    com_util::{ComCreate, ComError},
};
use com_ptr::ComPtr;
use std::{ffi::CStr, ptr, str};
use winapi::{ctypes::c_void, shared::ntdef::HRESULT, Interface};
use winapi::{
    shared::dxgiformat,
    um::{d3d11, d3dcommon, d3dcompiler, winnt::LPCSTR},
};
use winapi::{
    shared::minwindef::*,
    um::{d3d11shader, d3dcompiler::D3DReflect},
};

use super::constant_buffer::ConstantBufferShader;

unsafe fn to_string(blob: &ComPtr<d3dcommon::ID3DBlob>) -> String {
    let buf = blob.GetBufferPointer() as *mut u8;
    String::from_raw_parts(buf, blob.GetBufferSize(), blob.GetBufferSize())
}

unsafe fn compile_shader_from_source(
    filename: &str,
    source: &str,
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
        source.as_ptr() as LPCVOID,
        source.bytes().len(),
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
    d3d_device: &d3d11::ID3D11Device,
    compiled_vertex_shader: &ComPtr<d3dcommon::ID3DBlob>,
    reflection: &ComPtr<d3d11shader::ID3D11ShaderReflection>,
    semantics: &mut Vec<String>,
    input_layout: *mut *mut d3d11::ID3D11InputLayout,
) -> HRESULT {
    // Create InputLayout
    let mut shaderdesc = d3d11shader::D3D11_SHADER_DESC::default();
    reflection.GetDesc(&mut shaderdesc);

    let mut elements = vec![];
    for i in 0..shaderdesc.InputParameters {
        let mut param_desc = d3d11shader::D3D11_SIGNATURE_PARAMETER_DESC::default();
        reflection.GetInputParameterDesc(i, &mut param_desc);
        let format = to_dxgi_format(param_desc.ComponentType, param_desc.Mask);
        let element = d3d11::D3D11_INPUT_ELEMENT_DESC {
            SemanticName: param_desc.SemanticName,
            SemanticIndex: param_desc.SemanticIndex,
            Format: format,
            InputSlot: i,
            AlignedByteOffset: d3d11::D3D11_APPEND_ALIGNED_ELEMENT,
            InputSlotClass: d3d11::D3D11_INPUT_PER_VERTEX_DATA,
            InstanceDataStepRate: 0,
        };
        let semantic = String::from(CStr::from_ptr(element.SemanticName).to_str().unwrap());
        semantics.push(semantic);
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

impl ShaderSource {
    pub fn compile_vertex_shader(
        &self,
        d3d_device: &d3d11::ID3D11Device,
    ) -> Result<
        (
            ComPtr<d3d11::ID3D11VertexShader>,
            Vec<String>,
            ComPtr<d3d11::ID3D11InputLayout>,
            ConstantBufferShader,
        ),
        ComError,
    > {
        let compiled_vertex_shader = ComPtr::new(|| unsafe {
            compile_shader_from_source("vs\0", &self.source, &self.vs_main, "vs_4_0\0")
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

        let mut semantics: Vec<String> = vec![];
        let input_layout = ComPtr::create_if_success(|pp| unsafe {
            create_input_layout(
                d3d_device,
                &compiled_vertex_shader,
                &reflection,
                &mut semantics,
                pp,
            )
        })?;

        let constant_buffer = unsafe { ConstantBufferShader::new(d3d_device, &reflection)? };

        Ok((shader, semantics, input_layout, constant_buffer))
    }

    pub fn compile_pixel_shader(
        &self,
        d3d_device: &d3d11::ID3D11Device,
    ) -> Result<ComPtr<d3d11::ID3D11PixelShader>, ComError> {
        let compiled = ComPtr::new(|| unsafe {
            compile_shader_from_source("ps\0", &self.source, &self.ps_main, "ps_4_0\0")
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
}

pub struct Shader {
    pub vs: ComPtr<d3d11::ID3D11VertexShader>,
    pub vs_constant_buffer: ConstantBufferShader,
    pub ps: ComPtr<d3d11::ID3D11PixelShader>,
    pub semantics: Vec<String>,
    pub input_layout: ComPtr<d3d11::ID3D11InputLayout>,
}

impl Shader {
    pub fn new(
        vs: ComPtr<d3d11::ID3D11VertexShader>,
        vs_constant_buffer: ConstantBufferShader,
        ps: ComPtr<d3d11::ID3D11PixelShader>,
        semantics: Vec<String>,
        input_layout: ComPtr<d3d11::ID3D11InputLayout>,
    ) -> Shader {
        Shader {
            vs,
            vs_constant_buffer,
            ps,
            semantics,
            input_layout,
        }
    }

    pub fn compile(
        d3d_device: &d3d11::ID3D11Device,
        source: &ShaderSource,
    ) -> Result<Shader, ComError> {
        let (vs, semantics, input_layout, vs_constant_buffer) =
            source.compile_vertex_shader(&d3d_device)?;
        let ps = source.compile_pixel_shader(d3d_device)?;

        Ok(Shader::new(
            vs,
            vs_constant_buffer,
            ps,
            semantics,
            input_layout,
        ))
    }

    pub fn set(&self, d3d_context: &d3d11::ID3D11DeviceContext) {
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
