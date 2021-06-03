use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use clang_sys::*;

use crate::Function;

pub enum Decl {
    Function(Function),
}

pub struct UserType {
    hash: u32,
    count: RefCell<u32>,
    decl: Decl,
}

impl UserType {
    pub fn new(hash: u32, decl: Decl) -> UserType {
        UserType {
            hash,
            count: RefCell::new(0),
            decl,
        }
    }

    pub fn increment(&self) {
        self.count.replace_with(|&mut old| old + 1);
    }
}

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

pub enum Type {
    UserType(UserType),
    Primitive(Primitives),
}

pub struct TypeMap {
    map: RefCell<HashMap<u32, Rc<Type>>>,
    VOID: Rc<Type>,
    BOOL: Rc<Type>,
    I8: Rc<Type>,
    I16: Rc<Type>,
    I32: Rc<Type>,
    I64: Rc<Type>,
    U8: Rc<Type>,
    U16: Rc<Type>,
    U32: Rc<Type>,
    U64: Rc<Type>,
    F32: Rc<Type>,
    F64: Rc<Type>,
    F80: Rc<Type>,
}

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap {
            map: RefCell::new(HashMap::new()),
            VOID: Rc::new(Type::Primitive(Primitives::Void)),
            BOOL: Rc::new(Type::Primitive(Primitives::Bool)),
            I8: Rc::new(Type::Primitive(Primitives::I8)),
            I16: Rc::new(Type::Primitive(Primitives::I16)),
            I32: Rc::new(Type::Primitive(Primitives::I32)),
            I64: Rc::new(Type::Primitive(Primitives::I64)),
            U8: Rc::new(Type::Primitive(Primitives::U8)),
            U16: Rc::new(Type::Primitive(Primitives::U16)),
            U32: Rc::new(Type::Primitive(Primitives::U32)),
            U64: Rc::new(Type::Primitive(Primitives::U64)),
            F32: Rc::new(Type::Primitive(Primitives::F32)),
            F64: Rc::new(Type::Primitive(Primitives::F64)),
            F80: Rc::new(Type::Primitive(Primitives::F80)),
        }
    }

    pub fn get_or_create_user_type<T: FnOnce(u32) -> UserType>(&self, cursor: CXCursor, f: T) {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(t) = self.map.borrow_mut().get_mut(&hash) {
            // 前方参照で hash は登録済み
            // この型がTypedefなどから参照されている回数
            let t = t.clone();
            if let Type::UserType(t) = &*t {
                t.increment();
            }
            return;
        }

        let t = Type::UserType(f(hash));
        // match &mut t {
        //     Type::UserType(d) => {
        //         ;
        //     }
        //     _ => panic!(),
        // };
        self.map.borrow_mut().insert(hash, Rc::new(t));
    }

    pub fn type_from_cx_type(&self, cx_type: CXType, cursor: CXCursor) -> Rc<Type> {
        let isConst = unsafe { clang_isConstQualifiedType(cx_type) } != 0;

        match cx_type.kind {
            CXType_Void => self.VOID.clone(),
            CXType_Bool => self.BOOL.clone(),

            // Int
            CXType_Char_S | CXType_SChar => self.I8.clone(),
            CXType_Short => self.I16.clone(),
            CXType_Int | CXType_Long => self.I32.clone(),
            CXType_LongLong => self.I64.clone(),

            // UInt
            CXType_Char_U | CXType_UChar => self.U8.clone(),
            CXType_UShort | CXType_WChar => self.U16.clone(),
            CXType_UInt | CXType_ULong => self.U32.clone(),
            CXType_ULongLong => self.U64.clone(),

            // Float
            CXType_Float => self.F32.clone(),
            CXType_Double => self.F64.clone(),
            CXType_LongDouble => self.F80.clone(),
            _ => panic!(),
        }
    }

    pub fn type_from_cx_cursor(&self, cursor: CXCursor) -> Rc<Type> {
        let t = unsafe { clang_getCursorResultType(cursor) };
        self.type_from_cx_type(t, cursor)
    }
}
