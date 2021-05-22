mod shader;
use std::{collections::HashMap, rc::Rc};

use cgmath::Matrix;
pub use shader::*;
mod vertex_buffer;
pub use vertex_buffer::*;
mod d3d_device;
pub use d3d_device::*;
mod render_target;
pub use render_target::*;
mod frame;
use winapi::um::d3d11;

use crate::{asset_manager, scene};

pub struct ResourceManager {
    render_target: Option<RenderTarget>,
    // textures: HashMap<u32, Texture>,
    shaders: HashMap<String, Rc<Shader>>,
    materials: HashMap<u32, Material>,
    vertex_buffers: HashMap<u32, Rc<VertexBuffer>>,
}

fn shader_asset_path(shader: &scene::MaterialData) -> &'static str {
    match shader {
        scene::MaterialData::UnLight(_) => "shaders/mvp.hlsl",
    }
}

impl ResourceManager {
    pub fn get_or_create_rtv(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        texture: *mut d3d11::ID3D11Texture2D,
    ) -> &RenderTarget {
        let mut create = true;
        if let Some(render_target) = self.render_target.as_ref() {
            if render_target.texture == texture {
                create = false;
            }
        }

        if create {
            self.render_target = RenderTarget::create(d3d_device, texture).ok();
        }

        self.render_target.as_ref().unwrap()
    }

    pub fn get_or_create_shader(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        data: &scene::MaterialData,
    ) -> Rc<Shader> {
        let key = shader_asset_path(&data);
        if !self.shaders.contains_key(key) {
            if let Some(asset_manager) = asset_manager::get() {
                let source = asset_manager.get_shader_source(key).unwrap();
                let shader = Shader::compile(d3d_device, source).unwrap();
                self.shaders.insert(String::from(key), Rc::new(shader));
            }
        }

        self.shaders[key].clone()
    }

    pub fn get_or_create_material(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        material: &scene::Material,
    ) -> &Material {
        let id = material.get_id();
        if !self.materials.contains_key(&id) {
            let material = Material {
                name: material.name.clone(),
                color_texture: None,
                shader: self.get_or_create_shader(d3d_device, &material.data),
            };
            self.materials.insert(id, material);
        }

        &self.materials[&material.get_id()]
    }

    pub fn get_or_create_vertex_buffer(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        mesh: &scene::Mesh,
    ) -> Rc<VertexBuffer> {
        if !self.vertex_buffers.contains_key(&mesh.get_id()) {
            let vertex_buffer =
                VertexBuffer::from(d3d_device, &mesh.positions, &mesh.indices).unwrap();
            self.vertex_buffers
                .insert(mesh.get_id(), Rc::new(vertex_buffer));
        }

        self.vertex_buffers[&mesh.get_id()].clone()
    }

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        target_texture: *mut d3d11::ID3D11Texture2D,
        scene: &scene::Scene,
    ) {
        // render target
        let render_target = self.get_or_create_rtv(d3d_device, target_texture);
        render_target.set_and_clear(d3d_context);

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
        let vertex_buffer = self.get_or_create_vertex_buffer(d3d_device, mesh);
        vertex_buffer.prepare(d3d_context);

        for submesh in &mesh.submeshes {
            let material = self.get_or_create_material(d3d_device, &submesh.material);

            material
                .shader
                .vs_constant_buffer
                .update(d3d_context, 0, frame);

            material
                .shader
                .vs_constant_buffer
                .update(d3d_context, 1, &mesh.transform);

            material.shader.set(d3d_context);
            vertex_buffer.draw(d3d_context, submesh.index_count, submesh.offset)
        }
    }
}

static mut G_MANAGER: Option<Box<ResourceManager>> = None;

pub fn get() -> Option<&'static mut Box<ResourceManager>> {
    unsafe {
        if G_MANAGER.is_none() {
            G_MANAGER = Some(Box::new(ResourceManager {
                render_target: None,
                shaders: HashMap::new(),
                materials: HashMap::new(),
                vertex_buffers: HashMap::new(),
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
