use std::{collections::HashMap, rc::Rc};

use clang_sys::*;

use crate::{
    cx_source_location, cx_string, visit_children_with, Function, Primitives, Type, UserType,
    Visitor,
};

#[allow(non_upper_case_globals, non_snake_case)]
pub struct TypeMap {
    pub map: HashMap<u32, Rc<Type>>,

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

    NULL_PTR: Rc<Type>,
}

struct ReferenceVisitor {
    reference: Option<Rc<Type>>,
}

#[allow(non_upper_case_globals)]
impl Visitor for ReferenceVisitor {
    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap) -> bool {
        match cursor.kind {
            CXCursor_TypeRef => {
                let referenced = unsafe { clang_getCursorReferenced(cursor) };
                self.reference = Some(type_map.get_or_create_user_type(referenced));
                false
            }

            _ => true,
        }
    }

    type Result = Rc<Type>;

    fn result(&mut self, _type_map: &mut TypeMap) -> Self::Result {
        self.reference.take().unwrap()
    }
}

struct ElaboratedVisitor {
    reference: Option<Rc<Type>>,
}

#[allow(non_upper_case_globals)]
impl Visitor for ElaboratedVisitor {
    type Result = Rc<Type>;

    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap) -> bool {
        match cursor.kind {
            CXCursor_StructDecl | CXCursor_UnionDecl => {
                let t = type_map.get_or_create_user_type(cursor);

                // var structType = reference.Type as StructType;
                // if (structType is null)
                // {
                //     throw new NotImplementedException();
                // }

                // // if (!StructType.IsForwardDeclaration(child))
                // // {
                // //     structType.ParseFields(child, this);
                // // }

                self.reference = Some(t);

                false
            }

            CXCursor_EnumDecl => {
                let t = type_map.get_or_create_user_type(cursor);

                // if (reference.Type is null)
                // {
                //     throw new NotImplementedException();
                // }

                self.reference = Some(t);

                false
            }

            CXCursor_TypeRef => {
                let referenced = unsafe { clang_getCursorReferenced(cursor) };
                let t = type_map.get_or_create_user_type(referenced);
                self.reference = Some(t);
                false
            }

            _ => true,
        }
    }

    fn result(&mut self, _type_map: &mut TypeMap) -> Self::Result {
        self.reference.take().unwrap()
    }
}

#[allow(non_upper_case_globals)]
impl TypeMap {
    pub fn new() -> TypeMap {
        let void = Rc::new(Type::Primitive(Primitives::Void));
        TypeMap {
            map: HashMap::new(),
            VOID: void.clone(),
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
            NULL_PTR: Rc::new(Type::Pointer(void)),
        }
    }

    pub fn get_or_create_user_type(&mut self, cursor: CXCursor) -> Rc<Type> {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(t) = self.map.get_mut(&hash) {
            // 前方参照で hash は登録済み
            // この型がTypedefなどから参照されている回数
            let t = t.clone();
            if let Type::UserType(t) = &*t {
                t.increment();
            }
            return t;
        }

        let name = cx_string::CXString::cursor_spelling(cursor).to_string();
        let location = cx_source_location::CXSourceLocation::from_cursor(cursor);
        let (file, line) = location.get_path();
        let t = Rc::new(Type::UserType(UserType::new(hash, name, file, line)));
        self.map.insert(hash, t.clone());
        t
    }

    pub fn type_from_cx_type(&mut self, cx_type: CXType, cursor: CXCursor) -> (Rc<Type>, bool) {
        let mut is_const = unsafe { clang_isConstQualifiedType(cx_type) } != 0;

        let t = match cx_type.kind {
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

            // null_ptr ?
            CXType_Unexposed => self.NULL_PTR.clone(),

            CXType_Pointer | CXType_LValueReference => {
                let pointee = unsafe { clang_getPointeeType(cx_type) };
                let (pointee_type, _is_const) = self.type_from_cx_type(pointee, cursor);
                if _is_const {
                    is_const = true;
                }
                let t = Type::Pointer(pointee_type);
                Rc::new(t)
            }

            CXType_Typedef | CXType_Record => {
                // find reference from child cursors
                visit_children_with(cursor, self, || ReferenceVisitor { reference: None })
            }

            CXType_Elaborated => {
                visit_children_with(cursor, self, || ElaboratedVisitor { reference: None })
            }

            CXType_FunctionProto => {
                let t = self.get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t {
                    let result_type = unsafe { clang_getResultType(cx_type) };
                    let function = Function::parse(cursor, self, result_type);
                    t.set_function(function);
                }
                t
            }

            CXType_ConstantArray => {
                let array_size = unsafe { clang_getArraySize(cx_type) } as usize;
                let element_type = unsafe { clang_getArrayElementType(cx_type) };
                let (element_type, _is_const) = self.type_from_cx_type(element_type, cursor);
                let t = Type::Array(element_type, array_size);
                Rc::new(t)
            }

            CXType_IncompleteArray => {
                let element_type = unsafe { clang_getArrayElementType(cx_type) };
                let (element_type, _is_const) = self.type_from_cx_type(element_type, cursor);
                let t = Type::Pointer(element_type);
                Rc::new(t)
            }

            _ => {
                let _spelling = cx_string::CXString::cursor_spelling(cursor).to_string();
                let _location = cx_source_location::CXSourceLocation::from_cursor(cursor);
                let _file = _location.get_path();
                todo!()
            }
        };

        (t, is_const)
    }

    pub fn type_from_cx_cursor(&mut self, cursor: CXCursor) -> Rc<Type> {
        let cx_type = unsafe { clang_getCursorResultType(cursor) };
        let (t, _is_const) = self.type_from_cx_type(cx_type, cursor);
        t
    }
}
