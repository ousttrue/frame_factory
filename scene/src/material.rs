use std::rc::Rc;

use crate::Texture;

#[repr(C)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl RGBA {
    pub fn white() -> RGBA {
        RGBA {
            r: 1f32,
            g: 1f32,
            b: 1f32,
            a: 1f32,
        }
    }
}

pub struct UnLightMaterial {
    pub color: RGBA,
    pub color_texture: Option<Rc<Texture>>,
}

pub enum MaterialData {
    UnLight(UnLightMaterial),
}

pub struct Material {
    id: u32,
    pub name: String,
    pub data: MaterialData,
}

static mut G_NEXT_ID: u32 = 1;

fn next_id() -> u32 {
    unsafe {
        let id = G_NEXT_ID;
        G_NEXT_ID += 1;
        id
    }
}

impl Material {
    pub fn new(name: String, data: MaterialData) -> Material {
        let id = next_id();
        Material { id, name, data }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
