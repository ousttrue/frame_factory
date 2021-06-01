use clang_sys::*;
use std::{collections::HashMap, ffi::c_void, ptr};

mod translation_unit;
pub use translation_unit::*;

mod cxsourcelocation;
mod cxstring;

enum Kind {
    None,
    Namespace(String),
    Function(String),
}

struct Value {
    hash: u32,
    parent_hash: u32,
    kind: Kind,
}

impl Value {
    pub fn new(cursor: CXCursor, parent_hash: u32) -> Value {
        let hash = unsafe { clang_hashCursor(cursor) };

        match cursor.kind {
            CXCursor_Namespace => {
                let spelling = cxstring::CXString::from_cursor(cursor);
                Value {
                    hash,
                    parent_hash,
                    kind: Kind::Namespace(spelling.to_string()),
                }
            }
            CXCursor_FunctionDecl => {
                let spelling = cxstring::CXString::from_cursor(cursor);
                Value {
                    hash,
                    parent_hash,
                    kind: Kind::Function(spelling.to_string()),
                }
            }
            _ => Value {
                hash,
                parent_hash,
                kind: Kind::None,
            },
        }
    }
}

struct Data {
    map: HashMap<u32, Value>,
}

struct ParentIterator<'a> {
    current: Option<&'a Value>,
    data: &'a Data,
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if let Some(value) = self.data.map.get(&current.parent_hash) {
                self.current = Some(value);
                return Some(&value);
            }
        }

        self.current = None;
        None
    }
}

impl Data {
    fn new() -> Data {
        Data {
            map: HashMap::new(),
        }
    }

    // fn push_cursor_value(&mut self, cursor: CXCursor, value: Value) {
    //     let hash = unsafe { clang_hashCursor(cursor) };
    //     self.map.insert(hash, value);
    // }

    fn get_cursor_value(&self, cursor: CXCursor) -> Option<&Value> {
        let hash = unsafe { clang_hashCursor(cursor) };
        if let Some(value) = self.map.get(&hash) {
            Some(value)
        } else {
            None
        }
    }

    fn iter_parents(&self, cursor: CXCursor) -> ParentIterator {
        ParentIterator {
            current: self.get_cursor_value(cursor),
            data: self,
        }
    }

    fn on_function(&mut self, cursor: CXCursor) {
        for parent in self.iter_parents(cursor) {}

        let location = cxsourcelocation::CXSourceLocation::from_cursor(cursor);
        if !location.is_null() {
            let file = location.get_path();
            let spelling = cxstring::CXString::from_cursor(cursor);
            match file.file_name() {
                Some(name) if name == "imgui.h" => {
                    println!("{}({})", spelling.to_string(), file.to_string_lossy())
                }
                _ => (),
            }
        }
    }

    fn on_visit(&mut self, cursor: CXCursor, parent: CXCursor) {
        let parent_is_null = unsafe { clang_Cursor_isNull(parent) } != 0;
        if parent_is_null {
            panic!();
        }

        {
            let parent_value = self.get_cursor_value(parent).unwrap();
            let value = Value::new(cursor, parent_value.hash);
            self.map.insert(value.hash, value);
        }

        {
            for parent in self.iter_parents(cursor) {
                match &parent.kind {
                    Kind::Namespace(ns) if ns == "ImGui" => {
                        if let Some(value) = self.get_cursor_value(cursor) {
                            match &value.kind {
                                Kind::Function(name) => {
                                    println!("fn {}", name);
                                }
                                Kind::Namespace(name) => (),
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        match cursor.kind {
            CXCursor_UnexposedDecl => (),
            CXCursor_StructDecl => (),
            CXCursor_UnionDecl => (),
            CXCursor_EnumDecl => (),
            CXCursor_EnumConstantDecl => (),
            CXCursor_FieldDecl => (),
            CXCursor_VarDecl => (),
            CXCursor_FunctionDecl => (),
            CXCursor_ParmDecl => (),
            CXCursor_TypedefDecl => (),
            CXCursor_Namespace => (),
            CXCursor_NamespaceRef => (),
            CXCursor_ConversionFunction => (),
            CXCursor_CXXMethod => (),
            CXCursor_Constructor => (),
            CXCursor_Destructor => (),
            CXCursor_FunctionTemplate => (),
            CXCursor_ClassTemplate => (),
            CXCursor_TypeRef => (),
            CXCursor_TemplateRef => (),
            CXCursor_TemplateTypeParameter => (),
            CXCursor_OverloadedDeclRef => (),
            CXCursor_NonTypeTemplateParameter => (),
            CXCursor_ClassTemplatePartialSpecialization => (),
            CXCursor_UsingDeclaration => (),
            //
            CXCursor_ArraySubscriptExpr => (),
            CXCursor_UnexposedExpr => (),
            CXCursor_MemberRefExpr => (),
            CXCursor_CStyleCastExpr => (),
            CXCursor_StringLiteral => (),
            CXCursor_CallExpr => (),
            CXCursor_CXXThisExpr => (),
            CXCursor_DeclRefExpr => (),
            CXCursor_UnaryOperator => (),
            CXCursor_BinaryOperator => (),
            CXCursor_ConditionalOperator => (),
            CXCursor_ParenExpr => (),
            CXCursor_IntegerLiteral => (),
            CXCursor_FloatingLiteral => (),
            CXCursor_CXXThrowExpr => (),
            CXCursor_ObjCStringLiteral => (),
            CXCursor_CXXNullPtrLiteralExpr => (),
            CXCursor_CompoundAssignOperator => (),
            CXCursor_CXXStaticCastExpr => (),
            CXCursor_CXXConstCastExpr => (),
            CXCursor_CXXBoolLiteralExpr => (),
            CXCursor_UnaryExpr => (),
            //
            CXCursor_CompoundStmt => (),
            CXCursor_ReturnStmt => (),
            CXCursor_DeclStmt => (),
            CXCursor_IfStmt => (),
            CXCursor_NullStmt => (),
            //
            CXCursor_UnexposedAttr => (),
            CXCursor_MacroDefinition => (),
            CXCursor_MacroExpansion => (),
            CXCursor_InclusionDirective => (),
            CXCursor_WarnUnusedResultAttr => (),
            CXCursor_StaticAssert => (),
            _ => println!("{:?}", cursor),
        };
    }
}

extern "C" fn visitor(
    cursor: CXCursor,
    parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    let mut data: Box<Data> = unsafe { Box::from_raw(data as *mut Data) };
    data.on_visit(cursor, parent);

    // avoid drop
    Box::into_raw(data);

    CXChildVisit_Recurse
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let tu = TranslationUnit::parse(args[0].as_str())?;

    let mut data = Box::new(Data::new());

    // traverse(&tu.get_cursor());
    let root = tu.get_cursor();
    let hash = unsafe { clang_hashCursor(root) };
    data.map.insert(
        hash,
        Value {
            kind: Kind::None,
            parent_hash: 0,
            hash,
        },
    );

    let p = Box::into_raw(data);
    unsafe { clang_visitChildren(root, visitor, p as *mut c_void) };
    let data = unsafe { Box::from_raw(p) };

    // find "SliderFloat2"

    // traverse

    Ok(())
}
