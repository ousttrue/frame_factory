use std::ffi::c_void;

use clang_sys::*;

pub trait OnVisit {
    type Result;

    fn on_visit(&mut self, cursor: CXCursor) -> bool;

    fn result(&mut self) -> Self::Result;
}

extern "C" fn visitor<T: OnVisit>(
    cursor: CXCursor,
    _parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    let t = data as *mut T;
    let t = unsafe { & mut *t };
    if t.on_visit(cursor) {
        CXChildVisit_Continue
    } else {
        CXChildVisit_Break
    }
}

pub fn visit_children<T: OnVisit>(cursor: CXCursor, ptr: *mut T) {
    unsafe { clang_visitChildren(cursor, visitor::<T>, ptr as *mut c_void) };
}

pub fn visit_children_with<T: OnVisit, F: FnOnce() -> T>(cursor: CXCursor, f: F) -> T::Result {
    let visitor = Box::new(f());
    let ptr = Box::into_raw(visitor);
    visit_children(cursor, ptr);
    // for drop
    let mut visitor = unsafe { Box::from_raw(ptr) };
    visitor.result()
}
