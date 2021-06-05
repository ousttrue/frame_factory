use std::{ffi::{c_void, CString, NulError}, path::Path, ptr};

extern crate clang_sys;

#[derive(Debug)]
pub enum Error {
    StaticMessage(&'static str),
    CString(NulError),
    IOError(std::io::Error),
}

pub struct Index {
    index: *mut c_void,
}

impl Drop for Index {
    fn drop(&mut self) {
        unsafe { clang_sys::clang_disposeIndex(self.index) };
    }
}

impl Index {
    pub fn new() -> Result<Index, Error> {
        let index = unsafe { clang_sys::clang_createIndex(0, 1) };
        if index.is_null() {
            return Err(Error::StaticMessage("fail to clang_sys::clang_createIndex"));
        }

        Ok(Index { index })
    }
}

pub struct TranslationUnit {
    index: Index,
    tu: *mut c_void,
}

impl TranslationUnit {
    pub fn parse(path: &Path) -> Result<TranslationUnit, Error> {
        let index = Index::new()?;

        let arguments = [
            "-x",
            "c++",
            "-target",
            "x86_64-windows-msvc",
            "-fms-compatibility-version=18",
            "-fdeclspec",
            "-fms-compatibility",
        ];
        let mut arguments: Vec<CString> = arguments
            .iter()
            .map(|arg| CString::new(*arg).unwrap())
            .collect();
        {
            let tmp = format!("-I{}", path.to_string_lossy());
            arguments.push(CString::new(tmp.as_str()).unwrap());
        }

        let path = CString::new(path.to_string_lossy().to_string()).map_err(|e| Error::CString(e))?;

        let pp: Vec<*const i8> = arguments.iter().map(|arg| arg.as_ptr()).collect();

        let tu = unsafe {
            clang_sys::clang_parseTranslationUnit(
                index.index,
                path.as_ptr(),
                pp.as_ptr(),
                pp.len() as i32,
                ptr::null_mut(),
                0,
                clang_sys::CXTranslationUnit_DetailedPreprocessingRecord as i32,
            )
        };
        if tu.is_null() {
            return Err(Error::StaticMessage(
                "clang_sys::clang_parseTranslationUnit",
            ));
        }

        Ok(TranslationUnit { index, tu })
    }

    pub fn get_cursor(&self) -> clang_sys::CXCursor {
        unsafe{clang_sys::clang_getTranslationUnitCursor(self.tu)}
    }
}
