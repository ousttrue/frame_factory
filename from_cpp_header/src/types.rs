use std::{
    cell::{Ref, RefCell},
    path::PathBuf,
    rc::Rc,
};

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
    name: RefCell<String>,
    // location
    pub file: PathBuf,
    pub line: u32,
    count: RefCell<u32>,
    decl: RefCell<Decl>,
}

impl UserType {
    pub fn new(hash: u32, name: String, file: PathBuf, line: u32) -> UserType {
        UserType {
            hash,
            name: RefCell::new(name),
            file,
            line,
            count: RefCell::new(0),
            decl: RefCell::new(Decl::None),
        }
    }

    pub fn get_name(&self) -> Ref<String> {
        self.name.borrow()
    }

    pub fn set_name(&self, name: String) {
        self.name.replace(name);
    }

    pub fn increment(&self) {
        self.count.replace_with(|&mut old| old + 1);
    }

    pub fn set_decl(&self, d: Decl) {
        self.decl.replace(d);
    }

    pub fn get_decl(&self) -> Ref<Decl> {
        self.decl.borrow()
    }

    pub fn set_function(&self, f: Function) {
        self.set_decl(Decl::Function(f));
    }
    pub fn set_typedef(&self, d: Typedef) {
        if let Decl::None = &*self.decl.borrow() {
        } else {
            // avoid function pointer and typedef has same cursor hash
            // typedef void (NAME*)();
            return;
        }
        self.set_decl(Decl::Typedef(d));
    }
    pub fn set_struct(&self, s: Struct) {
        self.set_decl(Decl::Struct(s));
    }
    pub fn set_enum(&self, e: Enum) {
        self.set_decl(Decl::Enum(e));
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
