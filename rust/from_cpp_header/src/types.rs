use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{Enum, Function, Struct, Typedef};

#[derive(Debug)]
pub enum Decl {
    None,
    Enum(Enum),
    Struct(Struct),
    Function(Function),
    Typedef(Typedef),
}

#[derive(Debug)]
pub struct UserType {
    hash: u32,
    pub name: String,
    // location
    pub file: PathBuf,
    pub line: u32,
    count: RefCell<u32>,
    pub decl: RefCell<Decl>,
}

impl UserType {
    pub fn new(hash: u32, name: String, file: PathBuf, line: u32) -> UserType {
        UserType {
            hash,
            name,
            file,
            line,
            count: RefCell::new(0),
            decl: RefCell::new(Decl::None),
        }
    }

    pub fn increment(&self) {
        self.count.replace_with(|&mut old| old + 1);
    }
}

#[derive(Debug)]
pub enum Primitives {
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    F80,
}

#[derive(Debug)]
pub enum Type {
    UserType(UserType),
    Pointer(Rc<Type>),
    Array(Rc<Type>, usize),
    Primitive(Primitives),
}
