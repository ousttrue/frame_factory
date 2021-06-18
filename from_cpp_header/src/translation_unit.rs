use std::{
    ffi::{c_void, CString, NulError},
    path::Path,
};

use clang_sys::CXUnsavedFile;

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
    _index: Index,
    tu: *mut c_void,
}

impl TranslationUnit {
    pub fn parse(
        path: &Path,
        unsaved_content: &str,
        args: &[String],
    ) -> Result<TranslationUnit, Error> {
        let index = Index::new()?;

        let path = path.to_string_lossy().to_string();

        let base_args = [
            "-x",
            "c++",
            "-target",
            "x86_64-windows-msvc",
            "-fms-compatibility-version=18",
            "-fdeclspec",
            "-fms-compatibility",
        ];
        let mut arguments: Vec<CString> = base_args
            .iter()
            .map(|arg| CString::new(*arg).unwrap())
            .collect();
        {
            arguments.push(CString::new(format!("-I{}", path).as_str()).unwrap());
        }
        for arg in args {
            arguments.push(CString::new(arg.as_str()).unwrap());
        }

        let path = CString::new(path).map_err(|e| Error::CString(e))?;

        let pp: Vec<*const i8> = arguments.iter().map(|arg| arg.as_ptr()).collect();

        let mut unsaved = CXUnsavedFile::default();
        let mut n_unsaved = 0;

        if unsaved_content.len() > 0 {
            n_unsaved = 1;
            unsaved.Filename = path.as_ptr();
            unsaved.Contents = unsaved_content.as_ptr() as *const i8;
            unsaved.Length = unsaved_content.len() as u32;
        }

        let tu = unsafe {
            clang_sys::clang_parseTranslationUnit(
                index.index,
                path.as_ptr(),
                pp.as_ptr(),
                pp.len() as i32,
                &mut unsaved as *mut CXUnsavedFile,
                n_unsaved,
                clang_sys::CXTranslationUnit_DetailedPreprocessingRecord as i32,
            )
        };
        if tu.is_null() {
            return Err(Error::StaticMessage(
                "clang_sys::clang_parseTranslationUnit",
            ));
        }

        Ok(TranslationUnit { _index: index, tu })
    }

    pub fn get_cursor(&self) -> clang_sys::CXCursor {
        unsafe { clang_sys::clang_getTranslationUnitCursor(self.tu) }
    }
}
