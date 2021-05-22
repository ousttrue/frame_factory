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
use winapi::um::d3d11;

use crate::{asset_manager, scene};

pub struct ResourceManager {
    render_target: Option<RenderTarget>,
    unlit: Option<Shader>,
    vertex_buffer: HashMap<u32, Rc<VertexBuffer>>,
}

impl ResourceManager {
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

    pub fn get_or_create_shader(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        shader: &scene::Material,
    ) -> &Shader {
        if self.unlit.is_none() {
            if let Some(asset_manager) = asset_manager::get() {
                let source = asset_manager.get_shader_source("shaders/mvp.hlsl").unwrap();
                let shader = Shader::compile(d3d_device, source).unwrap();
                self.unlit = Some(shader);
            }
        }

        &self.unlit.as_ref().unwrap()
    }

    pub fn get_or_create_vertex_buffer(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        model: &scene::Mesh,
    ) -> Rc<VertexBuffer> {
        if !self.vertex_buffer.contains_key(&model.get_id()) {
            let vertex_buffer =
                VertexBuffer::from(d3d_device, &model.positions, &model.indices).unwrap();
            self.vertex_buffer
                .insert(model.get_id(), Rc::new(vertex_buffer));
        }

        self.vertex_buffer.get(&model.get_id()).unwrap().clone()
    }

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
        scene: &scene::Scene,
    ) {
        // render
        self.get_or_create_rtv(d3d_device, texture);
        if let Some(render_target) = &self.render_target {
            render_target.set_and_clear(d3d_context);

            // update constant buffer
            let frame = scene::c0 {
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
    }

    fn render_node(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        frame: &scene::c0,
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
        frame: &scene::c0,
        mesh: &scene::Mesh,
    ) {
        let vertex_buffer = self.get_or_create_vertex_buffer(d3d_device, mesh);
        vertex_buffer.prepare(d3d_context);

        for submesh in &mesh.submeshes {
            let shader = self.get_or_create_shader(d3d_device, &submesh.material);

            shader.vs_constant_buffer.update(d3d_context, 0, frame);

            shader
                .vs_constant_buffer
                .update(d3d_context, 1, &mesh.transform);

            shader.set(d3d_context);
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
                unlit: None,
                vertex_buffer: HashMap::new(),
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
