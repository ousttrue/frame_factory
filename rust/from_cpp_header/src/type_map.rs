use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

use clang_sys::*;

use crate::Function;

pub enum Decl {
    Function(Function),
}

pub struct UserType {
    hash: u32,
    count: u32,
    decl: Decl,
}

impl UserType {
    pub fn new(hash: u32, decl: Decl) -> UserType {
        UserType {
            hash,
            count: 0,
            decl,
        }
    }
}

pub enum Primitives {
    Void,
}

pub enum Type {
    UserType(UserType),
    Primitive(Primitives),
}

static VOID: Type = Type::Primitive(Primitives::Void);

pub struct TypeMap {
    map: RefCell<HashMap<u32, Type>>,
}

impl TypeMap {
    pub fn new() -> TypeMap {
        TypeMap {
            map: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_or_create_user_type<T: FnOnce(u32) -> UserType>(&self, cursor: CXCursor, f: T) {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(Type::UserType(t)) = self.map.borrow_mut().get_mut(&hash) {
            // 前方参照で hash は登録済み
            // この型がTypedefなどから参照されている回数
            t.count += 1;
            return;
        }

        let t = Type::UserType(f(hash));
        // match &mut t {
        //     Type::UserType(d) => {
        //         ;
        //     }
        //     _ => panic!(),
        // };
        self.map.borrow_mut().insert(hash, t);
    }

    pub fn from_cx_type(&self, cx_type: CXType, cursor: CXCursor) -> &Type {
        let isConst = unsafe { clang_isConstQualifiedType(cx_type) } != 0;

        match cx_type.kind {
            CXType_Void => &VOID,
            // CXType_Bool => { dst = BoolType.Instance; return true;

            // // Int
            // CXType_Char_S => {
            // CXType_SChar => {
            //     dst = Int8Type.Instance;
            //     return true;
            // CXType_Short => {
            //     dst = Int16Type.Instance;
            //     return true;
            // CXType_Int => {
            // CXType_Long => {
            //     dst = Int32Type.Instance;
            //     return true;
            // CXType_LongLong => {
            //     dst = Int64Type.Instance;
            //     return true;

            // // UInt
            // CXType_Char_U => {
            // CXType_UChar => {
            //     dst = UInt8Type.Instance;
            //     return true;
            // CXType_UShort => {
            // CXType_WChar => {
            //     dst = UInt16Type.Instance;
            //     return true;
            // CXType_UInt => {
            // CXType_ULong => {
            //     dst = UInt32Type.Instance;
            //     return true;
            // CXType_ULongLong => {
            //     dst = UInt64Type.Instance;
            //     return true;

            // // Float
            // CXType_Float => {
            //     dst = Float32Type.Instance;
            //     return true;
            // CXType_Double => {
            //     dst = Float64Type.Instance;
            //     return true;
            // CXType_LongDouble => {
            //     dst = Float80Type.Instance;
            //     return true;
            _ => panic!(),
        }
    }
}
