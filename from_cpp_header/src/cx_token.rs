use std::{os::raw::c_uint, ptr};

use clang_sys::*;

use crate::cx_string;

pub struct CXTokens {
    tu: CXTranslationUnit,
    num: c_uint,
    token: *mut clang_sys::CXToken,
}

impl Drop for CXTokens {
    fn drop(&mut self) {
        unsafe { clang_disposeTokens(self.tu, self.token, self.num) };
    }
}

impl CXTokens {
    pub fn from_cursor(cursor: CXCursor) -> CXTokens {
        let tu = unsafe { clang_Cursor_getTranslationUnit(cursor) };
        let range = unsafe { clang_getCursorExtent(cursor) };

        let mut token = CXTokens {
            tu,
            num: 0,
            token: ptr::null_mut(),
        };

        unsafe { clang_tokenize(tu, range, &mut token.token, &mut token.num) };

        token
    }

    pub fn get(&self) -> Option<String> {
        let mut tokens: Vec<String> = Vec::new();
        let mut p = self.token;
        let mut found = false;
        for _i in 0..self.num {
            let token = unsafe { *p };
            let token = cx_string::CXString::from_token(self.tu, token).to_string();
            if found {
                tokens.push(token);
            } else {
                if token == "=" {
                    found = true;
                }
            }
            p = unsafe { p.add(1) };
        }

        if tokens.len() == 0 {
            return None;
        }

        let mut value = String::new();
        for token in &tokens {
            value.push_str(token);
        }
        Some(value)
    }
}
