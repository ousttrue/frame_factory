use std::{cell::Cell, convert::TryInto, fs::File, io::Read};

use winapi::um::d3d11;

use super::model::Model;

pub enum LoadError {
    NotImpl,
    FileNotFound,
    CanNotOpen,
    CanNotRead,
    InvalidHeader,
    UnknownVersion,
    InvalidUtf8,
    UnknownChunkType,
}

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

    fn read_bytes(&self, len: usize) -> Result<&[u8], LoadError> {
        let pos = self.pos.get();
        let ret = &self.buf[pos..pos + len];
        self.pos.set(pos + len);
        Ok(ret)
    }

    fn read_str(&self, len: usize) -> Result<&str, LoadError> {
        let pos = self.pos.get();
        match std::str::from_utf8(&self.buf[pos..pos + len]) {
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

pub struct Loader {
    pub models: Vec<Model>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { models: Vec::new() }
    }

    pub fn load(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<(), LoadError> {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy();
            match ext.to_lowercase().as_str() {
                "glb" => return self.load_glb(d3d_device, path),
                _ => (),
            }
        }

        Err(LoadError::NotImpl)
    }

    ///
    /// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#glb-file-format-specification
    ///
    pub fn load_glb(
        &self,
        _d3d_device: &d3d11::ID3D11Device,
        path: &std::path::Path,
    ) -> Result<(), LoadError> {
        if !path.exists() {
            return Err(LoadError::FileNotFound);
        }
        let mut f = File::open(path).map_err(|_| LoadError::CanNotOpen)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).map_err(|_| LoadError::CanNotRead)?;

        let reader = BytesReader::new(&buf);

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
                    json =
                        Some(std::str::from_utf8(&chunk_data).map_err(|_| LoadError::InvalidUtf8)?);
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
                return self.load_gltf(json, bin);
            }
        }

        Err(LoadError::NotImpl)
    }

    pub fn load_gltf(&self, json: &str, bin: &[u8]) -> Result<(), LoadError> {
        let deserialized: gltf::glTF = serde_json::from_str(json).unwrap();

        for m in deserialized.meshes {

            // let m = Model::new();
        }

        Err(LoadError::NotImpl)
    }
}