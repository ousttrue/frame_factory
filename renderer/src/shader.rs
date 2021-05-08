use std::ptr;

use com_ptr::ComPtr;
use winapi::um::{d3d11, d3dcommon, d3dcompiler, winnt::LPCSTR};
use winapi::{ctypes::c_void, Interface};
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

impl Shader {
    pub fn compile_vertex_shader(
        d3d_device: &ComPtr<d3d11::ID3D11Device>,
        source: &str,
        entry_point: &str,
    ) -> Result<ComPtr<d3d11::ID3D11VertexShader>, String> {
        let compiled = unsafe { CompileShaderFromSource("vs\0", source, entry_point, "vs_4_0\0")? };

        let mut shader: *mut d3d11::ID3D11VertexShader = ptr::null_mut();
        let hr = unsafe {
            d3d_device.CreateVertexShader(
                compiled.GetBufferPointer(),
                compiled.GetBufferSize(),
                ptr::null_mut(),
                &mut shader,
            )
        };
        if hr != 0 {
            return Err(String::from("fail to CreateVertexShader"));
        }

        // vertex shader reflection
        let mut reflection: *mut d3d11shader::ID3D11ShaderReflection = ptr::null_mut();
        let p: *mut *mut d3d11shader::ID3D11ShaderReflection = &mut reflection;
        let hr = unsafe {
            D3DReflect(
                compiled.GetBufferPointer(),
                compiled.GetBufferSize(),
                &d3d11shader::ID3D11ShaderReflection::uuidof(),
                p as *mut *mut c_void,
            )
        };
        if hr != 0 {
            return Err(String::from("D3DReflect"));
        }

        unsafe { Ok(ComPtr::from_raw(shader)) }
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
