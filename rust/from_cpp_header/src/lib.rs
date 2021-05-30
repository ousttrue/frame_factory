mod translation_unit;
use std::{collections::HashMap, ffi::c_void, ptr};

pub use translation_unit::*;

struct Data {
    map: HashMap<u32, u32>,
}

impl Data {
    fn new() -> Data {
        Data {
            map: HashMap::new(),
        }
    }

    fn push_cursor_value(&mut self, cursor: clang_sys::CXCursor, value: u32) {
        let hash = unsafe { clang_sys::clang_hashCursor(cursor) };
        self.map.insert(hash, value);
    }

    fn get_cursor_value(&self, cursor: clang_sys::CXCursor) -> Option<u32> {
        let hash = unsafe { clang_sys::clang_hashCursor(cursor) };
        if let Some(value) = self.map.get(&hash) {
            Some(value.clone())
        } else {
            None
        }
    }
}

fn get_indent(level: u32) -> String {
    let mut indent = String::new();

    for _ in 0..level {
        indent.push_str("  ");
    }

    indent
}

extern "C" fn visitor(
    cursor: clang_sys::CXCursor,
    parent: clang_sys::CXCursor,
    data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    let mut data: Box<Data> = unsafe { Box::from_raw(data as *mut Data) };

    if unsafe { clang_sys::clang_Cursor_isNull(parent) } == 0 {
        let value = data.get_cursor_value(parent).unwrap();
        data.push_cursor_value(cursor, value + 1);
        let indent = get_indent(value);
        println!("{}{:?}", indent, cursor);
    } else {
        println!("{:?}", cursor);
        data.push_cursor_value(cursor, 0)
    }

    // avoid drop
    Box::into_raw(data);
    clang_sys::CXChildVisit_Recurse
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let tu = TranslationUnit::parse(args[0].as_str())?;

    let mut data = Box::new(Data::new());

    // traverse(&tu.get_cursor());
    let root = tu.get_cursor();
    data.push_cursor_value(root, 0);

    let p = Box::into_raw(data);
    unsafe { clang_sys::clang_visitChildren(root, visitor, p as *mut c_void) };

    Ok(())
}
