use std::{cell::Cell, fs::File, io::Read, rc::Rc, usize};

use crate::*;

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

pub fn load(path: &std::path::Path) -> Result<Loader, Error> {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy();
        match ext.to_lowercase().as_str() {
            "glb" => return load_glb(path),
            _ => (),
        }
    }

    Err(Error::NotImpl)
}

///
/// https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#glb-file-format-specification
///
pub fn load_glb(path: &std::path::Path) -> Result<Loader, Error> {
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
            let gltf: gltf2::glTF =
                serde_json::from_str(json).map_err(|e| Error::StaticMessage("serde_json"))?;

            let mut loader = Loader {
                gltf,
                bin: Vec::from(bin),
                textures: Vec::new(),
                materials: Vec::new(),
                meshes: Vec::new(),
                nodes: Vec::new(),
            };

            loader.load()?;

            return Ok(loader);
        }
    }

    Err(Error::NotImpl)
}

pub struct Loader {
    gltf: gltf2::glTF,
    bin: Vec<u8>,
    pub textures: Vec<Rc<Image>>,
    pub materials: Vec<Rc<Material>>,
    pub meshes: Vec<Rc<Mesh>>,
    pub nodes: Vec<Rc<Node>>,
}

impl Loader {
    pub fn load(&mut self) -> Result<(), Error> {
        self.load_textures()?;
        self.load_materials()?;
        self.load_meshes()?;
        self.load_nodes()?;

        Ok(())
    }

    pub fn get_accessor_bytes(&self, accessor_index: i32) -> Option<(&[u8], i32, i32)> {
        let accessor = &self.gltf.accessors[accessor_index as usize];
        let buffer_view = &self.gltf.bufferViews[accessor.bufferView.unwrap() as usize];

        if buffer_view.buffer? != 0 {
            return None;
        }

        let start = buffer_view.byteOffset.unwrap_or(0) as usize;
        let end = start + buffer_view.byteLength.unwrap() as usize;
        let bytes = &self.bin[start..end];

        let start = accessor.byteOffset.unwrap_or(0) as usize;
        let stride = accessor.stride();
        let count = accessor.count?;
        let end = (start + count as usize * stride as usize) as usize;

        Some((&bytes[start..end], stride, count))
    }

    pub fn get_image_bytes_from_texture(&self, texture: &gltf2::Texture) -> Image {
        let image = &self.gltf.images[texture.source.unwrap() as usize];

        if image.uri.len() > 0 {
            todo!()
        }

        let buffer_view = &self.gltf.bufferViews[image.bufferView.unwrap() as usize];
        let start = buffer_view.byteOffset.unwrap() as usize;
        let end = start + buffer_view.byteLength.unwrap() as usize;
        let bytes = &self.bin[start..end];

        Image {
            bytes: Vec::from(bytes),
            name: String::from(&image.name),
            mime: String::from(&image.mimeType),
        }
    }

    pub fn load_textures(&mut self) -> Result<(), Error> {
        for t in &self.gltf.textures {
            let image = self.get_image_bytes_from_texture(t);
            self.textures.push(Rc::new(image));
        }

        Ok(())
    }

    fn get_material_color_texture(&self, material: &gltf2::Material) -> Option<Rc<Image>> {
        if let Some(texture_index) = material
            .pbrMetallicRoughness
            .as_ref()?
            .baseColorTexture
            .as_ref()?
            .index
        {
            let texture = &self.textures[texture_index as usize];
            Some(texture.clone())
        } else {
            None
        }
    }

    pub fn load_materials(&mut self) -> Result<(), Error> {
        for m in &self.gltf.materials {
            let texture = self.get_material_color_texture(m);
            let unlit = UnLightMaterial {
                color: RGBA::white(),
                color_texture: texture,
            };
            let material = Material {
                name: m.name.clone(),
                data: MaterialData::UnLight(unlit),
            };
            self.materials.push(Rc::new(material));
        }

        Ok(())
    }

    pub fn load_meshes(&mut self) -> Result<(), Error> {
        for m in &self.gltf.meshes {
            let mut vertex_buffer: Option<AccessorBytes> = None;
            let mut index_buffer: Option<AccessorBytes> = None;
            let mut index_count_list = Vec::new();

            for prim in &m.primitives {
                let accessor_index = prim.attributes.get("POSITION").unwrap().clone();
                let (positions, position_stride, position_count) =
                    self.get_accessor_bytes(accessor_index).unwrap();
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
                    self.get_accessor_bytes(prim.indices.unwrap()).unwrap();
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
                    let mut mesh = Mesh::new(vertex_buffer, index_buffer);

                    let mut offset = 0;
                    for (i, prim) in m.primitives.iter().enumerate() {
                        let material = &self.materials[prim.material.unwrap() as usize];

                        let index_count = index_count_list[i];
                        mesh.submeshes.push(Submesh {
                            offset: offset,
                            index_count: index_count,
                            material: material.clone(),
                        });
                        offset += index_count;
                    }

                    self.meshes.push(Rc::new(mesh));
                }
            }
        }

        Ok(())
    }

    pub fn load_nodes(&mut self) -> Result<(), Error> {
        for n in &self.gltf.nodes {
            let mut node = Node::new(&n.name);

            if let Some(mesh_index) = n.mesh {
                let mesh = &self.meshes[mesh_index as usize];
                node.mesh = Some(mesh.clone());
            }

            self.nodes.push(Rc::new(node));
        }

        // build tree
        for (i, n) in self.gltf.nodes.iter().enumerate() {
            let node = &self.nodes[i];
            for c in &n.children {
                let child = &self.nodes[*c as usize];
                Node::add_child(node, child);
            }
        }

        Ok(())
    }
}
