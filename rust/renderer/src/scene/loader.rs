use crate::{
    asset_manager,
    error::{Error, LoadError},
    resource, scene,
};
use std::{cell::Cell, fs::File, io::Read};
use winapi::um::d3d11;

use super::model::Model;

struct BytesReader<'a> {
    pos: Cell<usize>,
    buf: &'a [u8],
}

impl<'a> BytesReader<'a> {
    fn new(buf: &[u8]) -> BytesReader {
        {
            BytesReader {
                pos: Cell::new(0),
                buf,
            }
        }
    }

    fn read_bytes(&self, len: usize) -> &[u8] {
        let pos = self.pos.get();
        let ret = &self.buf[pos..pos + len];
        self.pos.set(pos + len);
        ret
    }

    fn read_str(&self, len: usize) -> Result<&str, std::str::Utf8Error> {
        let pos = self.pos.get();
        let ret = std::str::from_utf8(&self.buf[pos..pos + len])?;
        self.pos.set(pos + len);
        Ok(ret)
    }

    fn read_u32(&self) -> u32 {
        let pos = self.pos.get();
        let mut bytes4: [u8; 4] = Default::default();
        bytes4.copy_from_slice(&self.buf[pos..pos + 4]);
        self.pos.set(pos + 4);
        u32::from_le_bytes(bytes4)
    }
}

pub struct Loader {
    pub models: Vec<Model>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { models: Vec::new() }
    }

    pub fn load(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<(), Error> {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy();
            match ext.to_lowercase().as_str() {
                "glb" => return self.load_glb(d3d_device, path),
                _ => (),
            }
        }

        Err(Error::NotImpl)
    }

    ///
    /// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#glb-file-format-specification
    ///
    pub fn load_glb(
        &mut self,
        _d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<(), Error> {
        let mut f = File::open(path).map_err(|e| Error::IOError(e))?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|e| Error::IOError(e))?;

        let reader = BytesReader::new(&buf);

        if reader.read_str(4).map_err(|e| Error::Utf8Error(e))? != "glTF" {
            return Err(Error::LoadError(LoadError::InvalidHeader));
        }

        if reader.read_u32() != 2 {
            return Err(Error::LoadError(LoadError::UnknownVersion));
        }

        let length = reader.read_u32() as usize;

        let mut json: Option<&str> = None;
        let mut bin: Option<&[u8]> = None;

        while reader.pos.get() < length {
            let chunk_length = reader.read_u32() as usize;
            let chunk_type = reader.read_str(4).map_err(|e| Error::Utf8Error(e))?;
            let chunk_data = reader.read_bytes(chunk_length);

            match &chunk_type {
                &"JSON" => {
                    json = Some(std::str::from_utf8(&chunk_data).map_err(|e| Error::Utf8Error(e))?);
                }
                &"BIN\0" => {
                    bin = Some(&chunk_data);
                }
                _ => {
                    return Err(Error::LoadError(LoadError::UnknownChunkType));
                }
            }
        }

        if let Some(json) = json {
            if let Some(bin) = bin {
                return self.load_gltf(_d3d_device, json, bin);
            }
        }

        Err(Error::NotImpl)
    }

    pub fn load_gltf(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        json: &str,
        bin: &[u8],
    ) -> Result<(), Error> {
        let gltf: gltf2::glTF = serde_json::from_str(json).unwrap();

        let asset_manager = asset_manager::get().unwrap();
        let source = asset_manager
            .get_shader_source("shaders/mvp.hlsl")
            .map_err(|e| Error::IOError(e))?;

        for m in &gltf.meshes {
            for prim in &m.primitives {
                let accessor_index = prim.attributes.get("POSITION").unwrap().clone();

                let (positions, stride, _) = gltf.get_accessor_bytes(bin, accessor_index).unwrap();
                let v = resource::VertexBuffer::create_vertices(d3d_device, positions)
                    .map_err(|e| Error::ComError(e))?;

                let (indices, index_stride, index_count) =
                    gltf.get_accessor_bytes(bin, prim.indices.unwrap()).unwrap();
                let i = resource::VertexBuffer::create_indices(d3d_device, indices)
                    .map_err(|e| Error::ComError(e))?;

                let vertex_buffer = resource::VertexBuffer::new(v, stride, i, index_stride, index_count);
                let shader = resource::Shader::compile(d3d_device, source)
                    .map_err(|e| Error::ComError(e))?;
                let model = scene::Model::new(vertex_buffer, shader);

                self.models.push(model);
            }
        }

        Ok(())
    }
}
