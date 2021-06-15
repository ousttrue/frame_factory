use std::ffi::CStr;

use clang_sys::CXTranslationUnit;

pub struct CXString {
    data: clang_sys::CXString,
}

impl CXString {
    pub fn cursor_spelling(cursor: clang_sys::CXCursor) -> CXString {
        CXString {
            data: unsafe { clang_sys::clang_getCursorSpelling(cursor) },
        }
    }

    pub fn cursor_mangling(cursor: clang_sys::CXCursor) -> CXString {
        CXString {
            data: unsafe { clang_sys::clang_Cursor_getMangling(cursor) },
        }
    }

    pub fn from_file(file: clang_sys::CXFile) -> CXString {
        CXString {
            data: unsafe { clang_sys::clang_getFileName(file) },
        }
    }

    pub fn from_token(tu: CXTranslationUnit, token: clang_sys::CXToken) -> CXString {
        CXString {
            data: unsafe { clang_sys::clang_getTokenSpelling(tu, token) },
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let ptr = clang_sys::clang_getCString(self.data);
            if ptr.is_null() {
                "".to_owned()
            } else {
                let str = CStr::from_ptr(ptr).to_owned();
                str.to_string_lossy().into_owned()
            }
        }
    }
}

impl Drop for CXString {
    fn drop(&mut self) {
        unsafe { clang_sys::clang_disposeString(self.data) };
    }
}
