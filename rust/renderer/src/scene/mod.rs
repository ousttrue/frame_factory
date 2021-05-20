use cgmath::Matrix;
use winapi::um::d3d11;

use crate::resource::RenderTarget;

pub mod scene_manager;
pub use scene_manager::*;

pub mod camera;
pub use camera::*;

pub mod model;
pub use model::*;

pub mod frame;
pub use frame::*;

pub mod loader;

pub mod screen_state;
pub use screen_state::*;

pub struct Scene {
    render_target: Option<RenderTarget>,
    camera: Camera,
    pub models: Vec<Model>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            render_target: None,
            camera: Camera::new(),
            models: Vec::new(),
        }
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

            for model in &self.models {
                model.render(d3d_context, &frame);
            }
        }
    }
}
