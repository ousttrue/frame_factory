use std::{ffi::{c_void}, path::{Path, PathBuf}, ptr};

use crate::{cx_string};

pub struct CXSourceLocation {
    data: clang_sys::CXSourceLocation,
}

impl CXSourceLocation {
    pub fn from_cursor(cursor: clang_sys::CXCursor) -> CXSourceLocation {
        let location = unsafe { clang_sys::clang_getCursorLocation(cursor) };

        CXSourceLocation { data: location }
    }

    pub fn is_null(&self) -> bool {
        unsafe {
            clang_sys::clang_equalLocations(self.data, clang_sys::clang_getNullLocation()) != 0
        }
    }

    pub fn get_path(&self) -> PathBuf {
        let mut file: *mut c_void = ptr::null_mut();
        let mut line: u32 = 0;
        let mut column: u32 = 0;
        let mut offset: u32 = 0;

        unsafe {
            clang_sys::clang_getSpellingLocation(
                self.data,
                &mut file,
                &mut line,
                &mut column,
                &mut offset,
            )
        }

        let file = cx_string::CXString::from_file(file);

        Path::new(&file.to_string()).to_owned()
    }

    // let extent = unsafe { clang_sys::clang_getCursorExtent(cursor) };
    // let begin = unsafe { clang_sys::clang_getRangeStart(extent) };

    // unsafe {
    //     clang_sys::clang_getInstantiationLocation(
    //         location,
    //         &mut file,
    //         &mut line,
    //         &mut column,
    //         &mut offset,
    //     )
    // };

    // Some(CXSourceLocation {
    //     file: file.to_string_lossy().to_string(),
    //     line,
    //     column,
    //     offset,
    // })
}
