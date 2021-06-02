use std::{cell::Ref, collections::HashMap};

use clang_sys::*;

pub struct Type {
    hash: u32,
    count: u32,
}

impl Type {
    pub fn new(hash: u32) -> Type {
        Type { hash, count: 0 }
    }
}

pub struct TypeMap {
    map: HashMap<u32, Type>,
}

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap {
            map: HashMap::new(),
        }
    }

    fn get_or_create(&mut self, cursor: CXCursor) -> &Type {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(t) = self.map.get_mut(&hash) {
            // この型がTypedefなどから参照されている回数
            t.count += 1;
        } else {
            let t = Type::new(hash);
            self.map.insert(hash, t);
        }

        self.map.get(&hash).unwrap()
    }

    pub fn parse_function(&mut self, cursor: CXCursor) {
        let t = self.get_or_create(cursor);

        let result = unsafe { clang_getCursorResultType(cursor) };

       
    }
}
