use crate::{
    asset_manager,
    error::{Error, LoadError},
    resource, scene,
};
use std::{cell::Cell, fs::File, io::Read};
use winapi::um::d3d11;

use super::{mesh::Mesh, AccessorBytes, RGBA};

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
    pub models: Vec<Mesh>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader { models: Vec::new() }
    }

    pub fn load(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), Error> {
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy();
            match ext.to_lowercase().as_str() {
                "glb" => return self.load_glb(path),
                _ => (),
            }
        }

        Err(Error::NotImpl)
    }

    ///
    /// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#glb-file-format-specification
    ///
    pub fn load_glb(&mut self, path: &std::path::Path) -> Result<(), Error> {
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
                return self.load_gltf(json, bin);
            }
        }

        Err(Error::NotImpl)
    }

    fn load_material(&mut self, gltf_material: &gltf2::Material) -> scene::Material {
        let unlit = scene::UnLightMaterial {
            color: RGBA::white(),
            color_texture: None,
        };
        scene::Material {
            name: gltf_material.name.clone(),
            material_type: scene::MaterialTypes::UnLight(unlit),
        }
    }

    pub fn load_gltf(&mut self, json: &str, bin: &[u8]) -> Result<(), Error> {
        let gltf: gltf2::glTF = serde_json::from_str(json).unwrap();

        for m in &gltf.meshes {
            let mut vertex_buffer: Option<scene::AccessorBytes> = None;
            let mut index_buffer: Option<scene::AccessorBytes> = None;
            let mut index_count_list = Vec::new();

            for prim in &m.primitives {
                let accessor_index = prim.attributes.get("POSITION").unwrap().clone();
                let (positions, position_stride, position_count) =
                    gltf.get_accessor_bytes(bin, accessor_index).unwrap();
                if let Some(vertices) = &mut vertex_buffer {
                    vertices.extends(positions, position_stride as u32, position_count as u32);
                } else {
                    vertex_buffer = Some(AccessorBytes::new(
                        Vec::from(positions),
                        position_stride as u32,
                        position_count as u32,
                    ));
                }

                let (indices, indices_stride, indices_count) =
                    gltf.get_accessor_bytes(bin, prim.indices.unwrap()).unwrap();
                index_count_list.push(indices_count as u32);

                if let Some(index_buffer) = &mut index_buffer {
                    index_buffer.extends(indices, indices_stride as u32, indices_count as u32);
                } else {
                    index_buffer = Some(AccessorBytes::new(
                        Vec::from(indices),
                        indices_stride as u32,
                        indices_count as u32,
                    ));
                }
            }

            if let Some(vertex_buffer) = vertex_buffer {
                if let Some(index_buffer) = index_buffer {
                    let mut model = scene::Mesh::new(vertex_buffer, index_buffer);

                    let mut offset = 0;
                    for (i, prim) in m.primitives.iter().enumerate() {
                        let material =
                            self.load_material(&gltf.materials[prim.material.unwrap() as usize]);

                        let index_count = index_count_list[i];
                        model.submeshes.push(scene::Submesh {
                            offset: offset,
                            index_count: index_count,
                            material,
                        });
                        offset += index_count;
                    }

                    self.models.push(model);
                }
            }
        }

        Ok(())
    }
}
