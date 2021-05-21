mod shader;
use cgmath::Matrix;
pub use shader::*;
mod vertex_buffer;
pub use vertex_buffer::*;
mod d3d_device;
pub use d3d_device::*;
mod render_target;
pub use render_target::*;
use winapi::um::d3d11;

use crate::scene::{Model, Scene, c0};

pub struct ResourceManager {
    render_target: Option<RenderTarget>,
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

    pub fn render(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        texture: *mut d3d11::ID3D11Texture2D,
        scene: &Scene,
    ) {
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

            for model in &scene.models {
                self.render_model(d3d_device, d3d_context, &frame, model);
            }
        }
    }

    fn render_model(
        &mut self,
        d3d_device: &d3d11::ID3D11Device,
        d3d_context: &d3d11::ID3D11DeviceContext,
        frame: &c0,
        model: &Model,
    ) {
        model.shader
            .vs_constant_buffer
            .update(d3d_context, 0, frame);

        model.shader
            .vs_constant_buffer
            .update(d3d_context, 1, &model.model);

        // model
        model.shader.set(d3d_context);
        model.vertex_buffer.draw(d3d_context);        
    }
}

static mut G_MANAGER: Option<Box<ResourceManager>> = None;

pub fn get() -> Option<&'static mut Box<ResourceManager>> {
    unsafe {
        if G_MANAGER.is_none() {
            G_MANAGER = Some(Box::new(ResourceManager {
                render_target: None,
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
