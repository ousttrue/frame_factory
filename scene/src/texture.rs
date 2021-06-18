use std::rc::Rc;

use crate::Image;

pub enum MipMap {
    None,
    Use,
    Blend,
}

pub enum Filter {
    Point,
    Linear,
}

pub enum Wrap {
    Clamp,
    Repeat,
    Mirror,
}

pub struct Texture {
    id: u32,
    pub image: Rc<Image>,

    pub mipmap: MipMap,
    pub filter: Filter,
    pub wrap_u: Wrap,
    pub wrap_v: Wrap,
}

static mut G_NEXT_ID: u32 = 1;

fn next_id() -> u32 {
    unsafe {
        let id = G_NEXT_ID;
        G_NEXT_ID += 1;
        id
    }
}

impl Texture {
    pub fn new(
        image: Rc<Image>,
        mipmap: MipMap,
        filter: Filter,
        wrap_u: Wrap,
        wrap_v: Wrap,
    ) -> Texture {
        Texture {
            id: next_id(),
            image,
            mipmap,
            filter,
            wrap_u,
            wrap_v,
        }
    }

    pub fn default(image: Rc<Image>) -> Texture {
        Self::new(
            image,
            MipMap::Use,
            Filter::Linear,
            Wrap::Repeat,
            Wrap::Repeat,
        )
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
