use std::rc::Rc;

use clang_sys::*;

use crate::{Type, TypeMap};

pub struct Typedef {
    base_type: Rc<Type>,
}

impl Typedef {
    pub fn parse(cursor: CXCursor, type_map: &TypeMap) -> Typedef {
        let underlying = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
        let base_type = type_map.type_from_cx_type(underlying, cursor);

        Typedef { base_type }
    }
}
