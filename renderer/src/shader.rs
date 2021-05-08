use std::ptr;

use com_ptr::ComPtr;
use winapi::{ctypes::c_void, Interface};
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

unsafe fn CompileShaderFromSource(
    filename: &str,
    source: &str,
    entry_point: &str,
    shader_model: &str,
) -> Result<ComPtr<d3dcommon::ID3D10Blob>, String> {
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
        source.len(),
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
        return Err(message);
    }

    Ok(ComPtr::from_raw(blob))
}

pub struct Shader {
    pub vertex: ComPtr<d3d11::ID3D11VertexShader>,
    pub pixel: ComPtr<d3d11::ID3D11PixelShader>,
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

unsafe fn create_input_layout(
    d3d_device: &ComPtr<d3d11::ID3D11Device>,
    compiled_vertex_shader: &ComPtr<d3dcommon::ID3DBlob>,
) -> Result<ComPtr<d3d11::ID3D11InputLayout>, String> {
    // vertex shader reflection
    let mut reflection: *mut d3d11shader::ID3D11ShaderReflection = ptr::null_mut();
    let p: *mut *mut d3d11shader::ID3D11ShaderReflection = &mut reflection;
    let hr = D3DReflect(
        compiled_vertex_shader.GetBufferPointer(),
        compiled_vertex_shader.GetBufferSize(),
        &d3d11shader::ID3D11ShaderReflection::uuidof(),
        p as *mut *mut c_void,
    );
    if hr != 0 {
        return Err(String::from("D3DReflect"));
    }
    let reflection = ComPtr::from_raw(reflection);

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
        return Err(String::from("no elements"));
    }

    let mut input_layout: *mut d3d11::ID3D11InputLayout = ptr::null_mut();
    let hr = d3d_device.CreateInputLayout(
        elements.as_ptr(),
        elements.len() as UINT,
        compiled_vertex_shader.GetBufferPointer(),
        compiled_vertex_shader.GetBufferSize(),
        &mut input_layout,
    );
    if hr != 0 {
        return Err(String::from("CreateInputLayout"));
    }

    Ok(ComPtr::from_raw(input_layout))
}

impl Shader {
    pub fn compile_vertex_shader(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        source: &str,
        entry_point: &str,
    ) -> Result<
        (
            ComPtr<d3d11::ID3D11VertexShader>,
            ComPtr<d3d11::ID3D11InputLayout>,
        ),
        String,
    > {
        let compiled_vertex_shader =
            unsafe { CompileShaderFromSource("vs\0", source, entry_point, "vs_4_0\0")? };

        let mut shader: *mut d3d11::ID3D11VertexShader = ptr::null_mut();
        let hr = unsafe {
            d3d_device.CreateVertexShader(
                compiled_vertex_shader.GetBufferPointer(),
                compiled_vertex_shader.GetBufferSize(),
                ptr::null_mut(),
                &mut shader,
            )
        };
        if hr != 0 {
            return Err(String::from("fail to CreateVertexShader"));
        }
        let shader = unsafe { ComPtr::from_raw(shader) };

        let input_layout = unsafe { create_input_layout(d3d_device, &compiled_vertex_shader)? };

        Ok((shader, input_layout))
    }

    pub fn compile_pixel_shader(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        source: &str,
        entry_point: &str,
    ) -> Result<ComPtr<d3d11::ID3D11PixelShader>, String> {
        let compiled = unsafe { CompileShaderFromSource("ps\0", source, entry_point, "ps_4_0\0")? };

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
            return Err(String::from("fail to CreatePixelShader"));
        }

        unsafe { Ok(ComPtr::from_raw(shader)) }
    }

    pub fn new(
        vertex: ComPtr<d3d11::ID3D11VertexShader>,
        pixel: ComPtr<d3d11::ID3D11PixelShader>,
    ) -> Shader {
        Shader { vertex, pixel }
    }
}
