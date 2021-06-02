use clang_sys::*;

use crate::TypeMap;

pub struct Function {}

impl Function {
    pub fn parse(cursor: CXCursor, type_map: &TypeMap) -> Function {

        let result = unsafe { clang_getCursorResultType(cursor) };

        Function {}
    }
}
