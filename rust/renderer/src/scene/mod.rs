use std::{
    cell::Cell,
    convert::TryInto,
    ffi::CStr,
    fs::File,
    io::{Bytes, Read},
    str::from_utf8,
};

use cgmath::{Matrix, One};
use winapi::um::{d3d11, winuser::DefMDIChildProcW};

use super::rendertarget::RenderTarget;
use crate::{com_util::ComError, shader::Shader, vertex_buffer::VertexBuffer};

pub mod camera;
use self::{camera::Camera, screenstate::ScreenState};
pub mod screenstate;

#[repr(C)]
struct c0 {
    view: [f32; 16],
    projection: [f32; 16],
}

#[repr(C)]
struct c1 {
    model: [f32; 16],
}

pub enum LoadError {
    NotImpl,
    FileNotFound,
    CanNotOpen,
    CanNotRead,
    InvalidHeader,
    UnknownVersion,
    Fail,
    InvalidUtf8,
    UnknownChunkType,
}

pub struct Scene {
    shader: Shader,
    model: cgmath::Matrix4<f32>,
    vertex_buffer: VertexBuffer,

    render_target: Option<RenderTarget>,
    camera: Camera,
}

struct BytesReader<'a> {
    pos: Cell<usize>,
    buf: &'a [u8],
}

impl<'a> BytesReader<'a> {
    fn new(buf: &[u8]) -> BytesReader {
        {
            BytesReader { pos: Cell::new(0), buf }
        }
    }

    fn read_bytes(&self, len: usize) -> Result<&[u8], LoadError> {
        let pos = self.pos.get();
        let ret = &self.buf[pos..pos + len];
        self.pos.set(pos + len);
        Ok(ret)
    }

    fn read_str(&self, len: usize) -> Result<&str, LoadError> {
        let pos = self.pos.get();
        match from_utf8(&self.buf[pos..pos + len]) {
            Ok(ret) => {
                self.pos.set(pos + len);
                Ok(ret)
            }
            Err(_) => Err(LoadError::InvalidUtf8),
        }
    }

    fn read_u32(&self) -> Result<u32, LoadError> {
        let pos = self.pos.get();
        let bytes4: [u8; 4] = self.buf[pos..pos + 4]
            .try_into()
            .map_err(|_| LoadError::InvalidHeader)?;
        self.pos.set(pos + 4);
        Ok(u32::from_le_bytes(bytes4))
    }
}

impl Scene {
    pub fn create(
        d3d_device: &d3d11::ID3D11Device,
        source: *const u8,
        source_size: usize,
        vs_main: *const u8,
        ps_main: *const u8,
    ) -> Result<Scene, ComError> {
        let (vs, input_layout, vs_constant_buffer) =
            Shader::compile_vertex_shader(&d3d_device, source, source_size, vs_main)?;
        let ps = Shader::compile_pixel_shader(d3d_device, source, source_size, ps_main)?;
        let shader = Shader {
            vs,
            ps,
            input_layout,
            vs_constant_buffer,
        };
        let vertex_buffer = VertexBuffer::create_triangle(d3d_device)?;

        let model: cgmath::Matrix4<f32> = cgmath::Matrix4::one();

        Ok(Scene {
            shader,
            model,
            vertex_buffer,
            render_target: None,
            camera: Camera::new(),
        })
    }

    pub fn load(
        d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<Scene, LoadError> {
        if let Some(ext) = path.extension() {
            if let ext = ext.to_string_lossy() {
                match ext.to_lowercase().as_str() {
                    "glb" => return Self::load_glb(d3d_device, path),
                    _ => (),
                }
            }
        }

        Err(LoadError::NotImpl)
    }

    ///
    /// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#glb-file-format-specification
    ///
    pub fn load_glb(
        d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<Scene, LoadError> {
        if !path.exists() {
            return Err(LoadError::FileNotFound);
        }
        let mut f = File::open(path).map_err(|_| LoadError::CanNotOpen)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|_| LoadError::CanNotRead)?;

        let mut reader = BytesReader::new(&buf);

        if reader.read_str(4)? != "glTF" {
            return Err(LoadError::InvalidHeader);
        }

        if reader.read_u32()? != 2 {
            return Err(LoadError::UnknownVersion);
        }

        let length = reader.read_u32()? as usize;

        let mut json: Option<&str> = None;
        let mut bin: Option<&[u8]> = None;

        while reader.pos.get() < length {
            let chunk_length = reader.read_u32()? as usize;
            let chunk_type = reader.read_str(4)?;
            let chunk_data = reader.read_bytes(chunk_length)?;

            match &chunk_type {
                &"JSON" => {
                    json = Some(from_utf8(&chunk_data).map_err(|_| LoadError::InvalidUtf8)?);
                }
                &"BIN\0" => {
                    bin = Some(&chunk_data);
                }
                _ => {
                    return Err(LoadError::UnknownChunkType);
                }
            }
        }

        if let Some(json) = json {
            if let Some(bin) = bin {
                return Self::load_gltf(json, bin);
            }
        }

        Err(LoadError::NotImpl)
    }

    pub fn load_gltf(json: &str, bin: &[u8]) -> Result<Scene, LoadError> {
        Err(LoadError::NotImpl)
    }

    pub fn get_or_create_rtv(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        texture: *mut d3d11::ID3D11Texture2D,
    ) {
        if let Some(render_target) = &self.render_target {
            if render_target.texture == texture {
                return;
            }
        }

        self.render_target = RenderTarget::create(d3d_device, texture).ok();
    }

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
        state: &ScreenState,
    ) {
        // update camera
        self.camera.update(state);

        // render
        self.get_or_create_rtv(d3d_device, texture);
        if let Some(render_target) = &self.render_target {
            render_target.set_and_clear(d3d_context);

            // update constant buffer
            let frame = c0 {
                view: Default::default(),
                projection: Default::default(),
            };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.camera.view.as_ptr() as *const u8,
                    frame.view.as_ptr() as *mut u8,
                    64,
                );
                std::ptr::copy_nonoverlapping(
                    self.camera.projection.as_ptr() as *const u8,
                    frame.projection.as_ptr() as *mut u8,
                    64,
                );
            }
            self.shader
                .vs_constant_buffer
                .update(d3d_context, 0, &frame);

            self.shader
                .vs_constant_buffer
                .update(d3d_context, 1, &self.model);

            // model
            self.shader.set(d3d_context);
            self.vertex_buffer.draw(d3d_context);
        }
    }
}
