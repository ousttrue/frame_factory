use std::fmt::Debug;
use std::rc::Rc;

use clang_sys::*;

use crate::{Type, TypeMap};

pub struct Typedef {
    base_type: Rc<Type>,
}

impl Debug for Typedef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Typedef {
    pub fn parse(cursor: CXCursor, type_map: &TypeMap) -> Typedef {
        let underlying = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
        let base_type = type_map.type_from_cx_type(underlying, cursor);

        Typedef { base_type }
    }
}
