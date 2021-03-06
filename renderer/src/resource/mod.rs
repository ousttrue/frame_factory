use cgmath::One;
use std::collections::HashMap;
use std::ptr;
use std::{cell::RefCell, ops::Deref, rc::Rc};
use winapi::um::d3d11;

mod shader;
use shader::*;
mod buffer_array;
use buffer_array::*;
mod d3d_device;
mod render_target;
use render_target::*;
mod constant_buffer;
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
        texture: &scene::Texture,
    ) -> Option<Rc<Texture>> {
        let id = texture.get_id();
        if !self.textures.borrow().contains_key(&id) {
            // load png/jpeg
            if let Ok(buffer) = load_texture(d3d_device, &texture.image.bytes) {
                // create texture
                let texture = Texture::new(d3d_device, buffer, texture).unwrap();
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
        semantics: &[String],
    ) -> Rc<BufferArray> {
        if !self.buffer_arrays.borrow().contains_key(&mesh.get_id()) {
            let vertex_buffer = BufferArray::from(
                d3d_device,
                &mesh.index_buffer,
                &mesh.vertex_buffers,
                semantics,
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
        // imgui
        unsafe {
            if gen::imgui::Begin("imgui_raw\0".as_ptr() as *const i8, ptr::null_mut(), 0) {
                // for (auto &msg : m_messages)
                // {
                gen::imgui::Text("imgui_raw\0".as_ptr() as *const i8);
                // imgui_raw::TextUnformatted(msg.data(), msg.data() + msg.size());
                // }
                gen::imgui::End();
            }
        }

        // render target
        self.clear_render_target(d3d_device, d3d_context, target_texture);

        for root in &scene.roots {
            self.render_node(
                d3d_device,
                d3d_context,
                scene,
                root,
                &cgmath::Matrix4::one(),
            );
        }
    }

    fn render_node(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        scene: &scene::Scene,
        node: &Rc<scene::Node>,
        world_matrix: &cgmath::Matrix4<f32>,
    ) {
        let local_matrix = node.transform.matrix();
        let matrix = world_matrix * local_matrix;
        if let Some(mesh) = &node.mesh {
            self.render_mesh(d3d_device, d3d_context, scene, &matrix, mesh.as_ref());
        }

        for child in node.get_children().iter() {
            self.render_node(d3d_device, d3d_context, scene, child, &matrix);
        }
    }

    fn render_mesh(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        scene: &scene::Scene,
        matrix: &cgmath::Matrix4<f32>,
        mesh: &scene::Mesh,
    ) {
        for submesh in &mesh.submeshes {
            let material = self.get_or_create_material(d3d_device, &submesh.material);

            //
            // update constant buffer
            //
            material
                .shader
                .vs_constant_buffer
                .update("ProjectionMatrix", &scene.camera.projection);
            material
                .shader
                .vs_constant_buffer
                .update("ViewMatrix", &scene.camera.view);
            match scene.light {
                scene::Light::DirectionalLight(dir) => {
                    material
                        .shader
                        .vs_constant_buffer
                        .update("LightDirection", &dir);
                }
            }
            material
                .shader
                .vs_constant_buffer
                .update("ModelMatrix", matrix);
            match &submesh.material.data {
                scene::MaterialData::UnLight(unlit) => {
                    material
                        .shader
                        .vs_constant_buffer
                        .update("Color", &unlit.color);
                }
            };
            for slot in &material.shader.vs_constant_buffer.slots {
                slot.commit(d3d_context);
            }

            //
            // shader setup
            //
            material.shader.set(d3d_context);
            if let Some(texture) = &material.color_texture {
                let srvs = [texture.srv.as_ptr()];
                let samplers = [texture.sampler.as_ptr()];
                unsafe {
                    d3d_context.PSSetSamplers(0, 1, samplers.as_ptr());
                    d3d_context.PSSetShaderResources(0, 1, srvs.as_ptr());
                };
            }

            //
            // vertex buffer
            //
            let vertex_buffer =
                self.get_or_create_vertex_buffer(d3d_device, mesh, &material.shader.semantics);
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
