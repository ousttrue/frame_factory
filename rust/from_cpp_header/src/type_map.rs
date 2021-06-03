use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

use clang_sys::*;

use crate::{Function, OnVisit, Typedef, cx_string, visit_children_with};

pub enum Decl {
    None,
    Function(Function),
    Typedef(Typedef),
}

pub struct UserType {
    hash: u32,
    pub name: String,
    count: RefCell<u32>,
    pub decl: RefCell<Decl>,
}

impl UserType {
    pub fn new(hash: u32, name: String) -> UserType {
        UserType {
            hash,
            name,
            count: RefCell::new(0),
            decl: RefCell::new(Decl::None),
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
    Pointer(Rc<Type>),
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

struct ReferenceVisitor<'a> {
    type_map: &'a TypeMap,
    reference: Option<Rc<Type>>,
}

#[allow(non_upper_case_globals)]
impl<'a> OnVisit<ReferenceVisitor<'a>> for ReferenceVisitor<'a> {
    fn on_visit(
        &mut self,
        _ptr: *mut ReferenceVisitor,
        cursor: CXCursor,
        _parent: CXCursor,
    ) -> bool {
        match cursor.kind {
            CXCursor_TypeRef => {
                let referenced = unsafe { clang_getCursorReferenced(cursor) };
                self.reference = Some(self.type_map.get_or_create_user_type(referenced));
                false
            }

            _ => true,
        }
    }

    type Result = Rc<Type>;

    fn result(&mut self) -> Self::Result {
        self.reference.take().unwrap()
    }
}

#[allow(non_upper_case_globals)]
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

    pub fn get_or_create_user_type(&self, cursor: CXCursor) -> Rc<Type> {
        let hash = unsafe { clang_hashCursor(cursor) };

        if let Some(t) = self.map.borrow_mut().get_mut(&hash) {
            // 前方参照で hash は登録済み
            // この型がTypedefなどから参照されている回数
            let t = t.clone();
            if let Type::UserType(t) = &*t {
                t.increment();
            }
            return t;
        }

        let name = cx_string::CXString::cursor_spelling(cursor).to_string();
        let t = Rc::new(Type::UserType(UserType::new(hash, name)));
        self.map.borrow_mut().insert(hash, t.clone());
        t
    }

    pub fn type_from_cx_type(&self, cx_type: CXType, cursor: CXCursor) -> Rc<Type> {
        // let isConst = unsafe { clang_isConstQualifiedType(cx_type) } != 0;

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

            CXType_Pointer | CXType_LValueReference => {
                let pointee = unsafe { clang_getPointeeType(cx_type) };
                let pointee_type = self.type_from_cx_type(pointee, cursor);
                let t = Type::Pointer(pointee_type);
                Rc::new(t)
            }

            CXType_Typedef | CXType_Record => {
                // find reference from child cursors
                visit_children_with(cursor, || ReferenceVisitor {
                    type_map: self,
                    reference: None,
                })
            }
            _ => todo!(),
        }

        // if (cxType.kind == CXTypeKind._Unexposed)
        // {
        //     // nullptr_t
        //     return TypeReference.FromPointer(new PointerType(TypeReference.FromPrimitive(VoidType.Instance)));
        // }

        // if (cxType.kind == CXTypeKind._LValueReference)
        // {
        //     return TypeReference.FromPointer(new PointerType(CxTypeToType(libclang.clang_getPointeeType(cxType), cursor)));
        // }

        // if (cxType.kind == CXTypeKind._IncompleteArray)
        // {
        //     return TypeReference.FromPointer(new PointerType(CxTypeToType(libclang.clang_getArrayElementType(cxType), cursor)));
        // }

        // if (cxType.kind == CXTypeKind._ConstantArray)
        // {
        //     var arraySize = (int)libclang.clang_getArraySize(cxType);
        //     var elementType = CxTypeToType(libclang.clang_getArrayElementType(cxType), cursor);
        //     return TypeReference.FromArray(new ArrayType(elementType, arraySize));
        // }

        // if (cxType.kind == CXTypeKind._Elaborated)
        // {
        //     // typedef struct {} Hoge;
        //     TypeReference reference = default;
        //     ClangVisitor.ProcessChildren(cursor, (in CXCursor child) =>
        //     {
        //         switch (child.kind)
        //         {
        //             case CXCursorKind._StructDecl:
        //             case CXCursorKind._UnionDecl:
        //                 {
        //                     reference = GetOrCreate(child);
        //                     var structType = reference.Type as StructType;
        //                     if (structType is null)
        //                     {
        //                         throw new NotImplementedException();
        //                     }

        //                     // if (!StructType.IsForwardDeclaration(child))
        //                     // {
        //                     //     structType.ParseFields(child, this);
        //                     // }

        //                     return CXChildVisitResult._Break;
        //                 }

        //             case CXCursorKind._EnumDecl:
        //                 {
        //                     reference = GetOrCreate(child);
        //                     if (reference.Type is null)
        //                     {
        //                         throw new NotImplementedException();
        //                     }
        //                     return CXChildVisitResult._Break;
        //                 }

        //             case CXCursorKind._TypeRef:
        //                 {
        //                     var referenced = libclang.clang_getCursorReferenced(child);
        //                     reference = GetOrCreate(referenced);
        //                     return CXChildVisitResult._Break;
        //                 }

        //             default:
        //                 return CXChildVisitResult._Continue;
        //         }
        //     });
        //     if (reference is null)
        //     {
        //         var children = cursor.Children();
        //         throw new NotImplementedException("Elaborated not found");
        //     }
        //     return reference;
        // }

        // if (cxType.kind == CXTypeKind._FunctionProto)
        // {
        //     var resultType = libclang.clang_getResultType(cxType);
        //     var functionType = FunctionType.Parse(cursor, this, resultType);
        //     return new TypeReference(cursor.CursorHashLocation(), functionType);
        // }
    }

    pub fn type_from_cx_cursor(&self, cursor: CXCursor) -> Rc<Type> {
        let t = unsafe { clang_getCursorResultType(cursor) };
        self.type_from_cx_type(t, cursor)
    }
}
