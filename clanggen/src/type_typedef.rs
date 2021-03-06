use std::fmt::Debug;
use std::rc::Rc;

use clang_sys::*;

use crate::{Type, TypeMap};

pub struct Typedef {
    pub base_type: Rc<Type>,
}

impl Debug for Typedef {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Typedef {
    pub fn parse(cursor: CXCursor, type_map: &mut TypeMap) -> Typedef {
        let underlying = unsafe { clang_getTypedefDeclUnderlyingType(cursor) };
        let (base_type, _is_const) = type_map.type_from_cx_type(underlying, cursor);

        Typedef { base_type }
    }

    pub fn base_type_is_anonymous(&self) -> Option<Rc<Type>> {
        if let Type::UserType(u) = &*self.base_type {
            if u.get_name().len() == 0 {
                return Some(self.base_type.clone());
            }
        }

        None
    }
}
