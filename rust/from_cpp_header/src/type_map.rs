use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use clang_sys::*;

use crate::Function;

pub enum Decl {
    Function(Function),
}

pub struct Type {
    hash: u32,
    count: u32,
    pub decl: Option<Decl>,
}

impl Type {
    pub fn new(hash: u32) -> Type {
        Type {
            hash,
            count: 0,
            decl: None,
        }
    }
}

pub struct TypeMap {
    map: RefCell<HashMap<u32, Type>>,
}

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap {
            map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_create<T: FnOnce(&mut Type)>(&self, cursor: CXCursor, f: T) {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(t) = self.map.borrow_mut().get_mut(&hash) {
            // 前方参照で hash は登録済み
            // この型がTypedefなどから参照されている回数
            t.count += 1;
            f(t);
            return;
        }

        let mut t = Type::new(hash);
        f(&mut t);
        self.map.borrow_mut().insert(hash, t);
    }
}
