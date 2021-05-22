use cgmath::Matrix;
use com_ptr::ComPtr;
use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    collections::HashMap,
    ops::Deref,
    rc::Rc,
};
use winapi::um::d3d11;

mod shader;
use shader::*;
mod buffer_array;
use buffer_array::*;
mod d3d_device;
mod render_target;
use render_target::*;
mod frame;
mod material;
use material::*;

use crate::{asset_manager, scene};

pub struct ResourceManager {
    render_target: RefCell<Option<RenderTarget>>,
    textures: RefCell<HashMap<u32, Rc<Texture>>>,
    shaders: RefCell<HashMap<String, Rc<Shader>>>,
    materials: RefCell<HashMap<u32, Rc<Material>>>,
    buffer_arrays: RefCell<HashMap<u32, Rc<BufferArray>>>,
}

fn shader_asset_path(shader: &scene::MaterialData) -> &'static str {
    match shader {
        scene::MaterialData::UnLight(_) => "shaders/mvp.hlsl",
    }
}

impl ResourceManager {
    pub fn clear_render_target(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
    ) {
        let mut create = true;
        if let Some(render_target) = self.render_target.borrow().as_ref() {
            if render_target.texture == texture {
                create = false;
            }
        }

        if create {
            self.render_target
                .replace(RenderTarget::create(d3d_device, texture).ok());
        }

        if let Some(render_target) = self.render_target.borrow().deref() {
            render_target.set_and_clear(d3d_context);
        }
    }

    pub fn get_or_create_shader(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        data: &scene::MaterialData,
    ) -> Rc<Shader> {
        let key = shader_asset_path(&data);
        if !self.shaders.borrow().contains_key(key) {
            if let Some(asset_manager) = asset_manager::get() {
                let source = asset_manager.get_shader_source(key).unwrap();
                let shader = Shader::compile(d3d_device, source).unwrap();
                self.shaders
                    .borrow_mut()
                    .insert(String::from(key), Rc::new(shader));
            }
        }

        self.shaders.borrow()[key].clone()
    }

    pub fn get_or_create_texture(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        texture: &scene::Image,
    ) -> Option<Rc<Texture>> {
        let id = texture.get_id();
        if !self.textures.borrow().contains_key(&id) {
            // load png/jpeg
            if let Ok(buffer) = load_texture(d3d_device, &texture.bytes) {
                // create texture
                let texture = Texture::new(d3d_device, buffer).unwrap();
                self.textures.borrow_mut().insert(id, Rc::new(texture));
            }
        }

        if let Some(texture) = self.textures.borrow().get(&id) {
            Some(texture.clone())
        } else {
            None
        }
    }

    pub fn get_or_create_material(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        src: &scene::Material,
    ) -> Rc<Material> {
        let id = src.get_id();
        if !self.materials.borrow().contains_key(&id) {
            let material = match &src.data {
                scene::MaterialData::UnLight(unlit) => {
                    let mut material = Material {
                        name: src.name.clone(),
                        color_texture: None,
                        shader: self.get_or_create_shader(d3d_device, &src.data),
                    };
                    if let Some(texture) = &unlit.color_texture {
                        material.color_texture =
                            self.get_or_create_texture(d3d_device, texture.as_ref());
                    }
                    material
                }
            };
            self.materials.borrow_mut().insert(id, Rc::new(material));
        }

        self.materials.borrow()[&src.get_id()].clone()
    }

    pub fn get_or_create_vertex_buffer(
        &self,
        d3d_device: &d3d11::ID3D11Device,
        mesh: &scene::Mesh,
        elements: &[d3d11::D3D11_INPUT_ELEMENT_DESC],
    ) -> Rc<BufferArray> {
        if !self.buffer_arrays.borrow().contains_key(&mesh.get_id()) {
            let vertex_buffer = BufferArray::from(
                d3d_device,
                &mesh.index_buffer,
                &mesh.vertex_buffers,
                elements,
            )
            .unwrap();
            self.buffer_arrays
                .borrow_mut()
                .insert(mesh.get_id(), Rc::new(vertex_buffer));
        }

        self.buffer_arrays.borrow()[&mesh.get_id()].clone()
    }

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        target_texture: *mut d3d11::ID3D11Texture2D,
        scene: &scene::Scene,
    ) {
        // render target
        self.clear_render_target(d3d_device, d3d_context, target_texture);

        // update constant buffer
        let frame = frame::c0 {
            view: Default::default(),
            projection: Default::default(),
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                scene.camera.view.as_ptr() as *const u8,
                frame.view.as_ptr() as *mut u8,
                64,
            );
            std::ptr::copy_nonoverlapping(
                scene.camera.projection.as_ptr() as *const u8,
                frame.projection.as_ptr() as *mut u8,
                64,
            );
        }

        for root in &scene.roots {
            self.render_node(d3d_device, d3d_context, &frame, root);
        }
    }

    fn render_node(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        frame: &frame::c0,
        node: &Rc<scene::Node>,
    ) {
        if let Some(mesh) = &node.mesh {
            self.render_mesh(d3d_device, d3d_context, frame, mesh.as_ref());
        }

        for child in node.get_children().iter() {
            self.render_node(d3d_device, d3d_context, frame, child);
        }
    }

    fn render_mesh(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        frame: &frame::c0,
        mesh: &scene::Mesh,
    ) {
        for submesh in &mesh.submeshes {
            let material = self.get_or_create_material(d3d_device, &submesh.material);

            // update constant buffer
            material
                .shader
                .vs_constant_buffer
                .update(d3d_context, 0, frame);

            material
                .shader
                .vs_constant_buffer
                .update(d3d_context, 1, &mesh.transform);

            material.shader.set(d3d_context);

            if let Some(texture) = &material.color_texture {
                let srvs = [texture.srv.as_ptr()];
                unsafe {
                    d3d_context.PSSetShaderResources(0, 1, srvs.as_ptr());
                };
            }

            let vertex_buffer =
                self.get_or_create_vertex_buffer(d3d_device, mesh, &material.shader.elements);
            vertex_buffer.prepare(d3d_context);
            vertex_buffer.draw(d3d_context, submesh.index_count, submesh.offset)
        }
    }
}

static mut G_MANAGER: Option<Box<ResourceManager>> = None;

pub fn get() -> Option<&'static mut Box<ResourceManager>> {
    unsafe {
        if G_MANAGER.is_none() {
            G_MANAGER = Some(Box::new(ResourceManager {
                render_target: RefCell::new(None),
                shaders: RefCell::new(HashMap::new()),
                textures: RefCell::new(HashMap::new()),
                materials: RefCell::new(HashMap::new()),
                buffer_arrays: RefCell::new(HashMap::new()),
            }))
        }
    }

    if let Some(asset_manager) = unsafe { &mut G_MANAGER } {
        Some(asset_manager)
    } else {
        None
    }
}

pub fn shutdown() {
    unsafe { G_MANAGER = None };
}
