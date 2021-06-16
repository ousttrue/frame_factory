use std::path::PathBuf;

use clang_sys::*;

use crate::{cx_source_location, cx_token};

pub struct Define {
    pub tokens: Vec<String>,
    pub path: PathBuf,
}

impl Define {
    pub fn parse(cursor: CXCursor) -> Option<Define> {
        if unsafe { clang_Cursor_isMacroFunctionLike(cursor) } != 0 {
            return None;
        }

        let token = cx_token::CXTokens::from_cursor(cursor);
        if token.len() < 2 {
            return None;
        }

        let (path, _) = cx_source_location::CXSourceLocation::from_cursor(cursor).get_path();
        let def = Define {
            tokens: token.get(),
            path,
        };
        Some(def)
    }
}
