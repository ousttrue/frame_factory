use std::rc::Rc;

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

pub struct Image {
    pub bytes: Vec<u8>,
    pub name: String,
    pub mime: String,
}

pub struct UnLightMaterial {
    pub color: RGBA,
    pub color_texture: Option<Rc<Image>>,
}

pub enum MaterialData {
    UnLight(UnLightMaterial),
}

pub struct Material {
    pub name: String,
    pub data: MaterialData,
}
