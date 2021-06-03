use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
};

use clang_sys::*;

pub struct NoDrop<T> {
    data: Option<Box<T>>,
}

impl<T> Drop for NoDrop<T> {
    fn drop(&mut self) {
        if let Some(box_data) = self.data.take() {
            Box::into_raw(box_data);
        }
    }
}

impl<T> Deref for NoDrop<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().unwrap()
    }
}

impl<T> DerefMut for NoDrop<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()
    }
}

impl<T> NoDrop<T> {
    pub fn new(data: *mut T) -> NoDrop<T> {
        NoDrop::<T> {
            data: Some(unsafe { Box::from_raw(data) }),
        }
    }
}

pub trait OnVisit<T> {
    type Result;

    fn on_visit(&mut self, ptr: *mut T, cursor: CXCursor, parent: CXCursor) -> bool;

    fn result(&mut self) -> Self::Result;
}

extern "C" fn visitor<T: OnVisit<T>>(
    cursor: CXCursor,
    parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    let t = data as *mut T;
    let mut no_drop = NoDrop::new(t);

    if no_drop.on_visit(t, cursor, parent) {
        CXChildVisit_Continue
    } else {
        CXChildVisit_Break
    }
}

pub fn visit_children<T: OnVisit<T>>(cursor: CXCursor, ptr: *mut T) {
    unsafe { clang_visitChildren(cursor, visitor::<T>, ptr as *mut c_void) };
}

pub fn visit_children_with<T: OnVisit<T>, F: FnOnce() -> T>(cursor: CXCursor, f: F) -> T::Result {
    let visitor = Box::new(f());
    let ptr = Box::into_raw(visitor);
    visit_children(cursor, ptr);
    // for drop
    let mut visitor = unsafe { Box::from_raw(ptr) };
    visitor.result()
}
