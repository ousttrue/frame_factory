use std::ffi::c_void;

use clang_sys::*;

use crate::TypeMap;

pub trait Visitor {
    type Result;

    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap) -> bool;

    fn result(&mut self, type_map: &mut TypeMap) -> Self::Result;
}

struct Data<'a, T: Visitor> {
    visitor: T,
    type_map: &'a mut TypeMap,
}

extern "C" fn visitor<T: Visitor>(
    cursor: CXCursor,
    _parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    let t = data as *mut Data<T>;
    let mut data = unsafe { &mut *t };
    if data.visitor.on_visit(cursor, data.type_map) {
        CXChildVisit_Continue
    } else {
        CXChildVisit_Break
    }
}

fn _visit_children<T: Visitor>(cursor: CXCursor, ptr: *mut Data<T>) {
    unsafe { clang_visitChildren(cursor, visitor::<T>, ptr as *mut c_void) };
}

pub fn visit_children_with<T: Visitor, F: FnOnce() -> T>(
    cursor: CXCursor,
    type_map: &mut TypeMap,
    f: F,
) -> T::Result {
    let visitor = f();
    let mut data = Data { visitor, type_map };
    let ptr = &mut data as *mut Data<T>;
    _visit_children(cursor, ptr);
    let data = unsafe { &mut *ptr };
    data.visitor.result(data.type_map)
}
