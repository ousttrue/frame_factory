use std::rc::Rc;

use crate::Image;

pub struct Texture {
    id: u32,
    pub image: Rc<Image>,
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
    pub fn new(image: Rc<Image>) -> Texture {
        Texture {
            id: next_id(),
            image,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
