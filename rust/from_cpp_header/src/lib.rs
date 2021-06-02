use clang_sys::*;
use std::{collections::HashMap, ffi::c_void, io::{Write, stderr}, io::stdout};

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

#[allow(non_upper_case_globals)]
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
    stack: Vec<u32>,
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

extern "C" fn visitor(
    cursor: CXCursor,
    parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    let data: Box<Data> = unsafe { Box::from_raw(data as *mut Data) };

    let data = on_visit(data, cursor, parent);

    // avoid drop
    Box::into_raw(data);

    CXChildVisit_Continue
}

fn visit_children(data: Box<Data>, cursor: CXCursor) -> Box<Data> {
    let p = Box::into_raw(data);
    unsafe { clang_visitChildren(cursor, visitor, p as *mut c_void) };
    unsafe { Box::from_raw(p) }
}

#[allow(non_upper_case_globals)]
fn on_visit(mut data: Box<Data>, cursor: CXCursor, parent: CXCursor) -> Box<Data> {
    let parent_is_null = unsafe { clang_Cursor_isNull(parent) } != 0;
    assert!(!parent_is_null);
    assert!(data.stack.len() == 0);

    // process current
    {
        let parent_value = data.get_cursor_value(parent).unwrap();
        let value = Value::new(cursor, parent_value.hash);
        data.stack.push(value.hash);
        data.map.insert(value.hash, value);
    }

    // {
    //     for parent in self.iter_parents(cursor) {
    //         match &parent.kind {
    //             Kind::Namespace(ns) if ns == "ImGui" => {
    //                 if let Some(value) = self.get_cursor_value(cursor) {
    //                     match &value.kind {
    //                         Kind::Function(name) => {
    //                             println!("fn {}", name);
    //                         }
    //                         Kind::Namespace(name) => (),
    //                         _ => (),
    //                     }
    //                 }
    //             }
    //             _ => (),
    //         }
    //     }
    // }

    let spelling = cxstring::CXString::from_cursor(cursor);
    let location = cxsourcelocation::CXSourceLocation::from_cursor(cursor);

    if let Some(filename) = location.get_path().file_name() {
        if filename == "imgui.h" {
            match cursor.kind {
                CXCursor_UnexposedDecl => (),
                CXCursor_StructDecl => {
                    // 2
                    println!(
                        "{} struct {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                // CXCursor_UnionDecl => (),
                CXCursor_EnumDecl => {
                    // 5
                    println!(
                        "{} enum {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                // CXCursor_EnumConstantDecl => (),
                // CXCursor_FieldDecl => (),
                CXCursor_VarDecl => {
                    // 9
                    println!(
                        "{} var {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                CXCursor_FunctionDecl => {
                    // 8
                    println!(
                        "{} fn {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                // CXCursor_ParmDecl => (),
                CXCursor_TypedefDecl => {
                    // 20
                    println!(
                        "{} typedef {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                CXCursor_Namespace => {
                    // 22
                    println!(
                        "{} namespace {}",
                        location.get_path().to_string_lossy(),
                        spelling.to_string()
                    );
                }
                // CXCursor_NamespaceRef => (),
                // CXCursor_ConversionFunction => (),
                // CXCursor_CXXMethod => (),
                // CXCursor_Constructor => (),
                // CXCursor_Destructor => (),
                CXCursor_FunctionTemplate => (),
                CXCursor_ClassTemplate => (),
                // CXCursor_TypeRef => (),
                // CXCursor_TemplateRef => (),
                // CXCursor_TemplateTypeParameter => (),
                // CXCursor_OverloadedDeclRef => (),
                // CXCursor_NonTypeTemplateParameter => (),
                // CXCursor_ClassTemplatePartialSpecialization => (),
                // CXCursor_UsingDeclaration => (),
                // //
                // CXCursor_ArraySubscriptExpr => (),
                // CXCursor_UnexposedExpr => (),
                // CXCursor_MemberRefExpr => (),
                // CXCursor_CStyleCastExpr => (),
                // CXCursor_StringLiteral => (),
                // CXCursor_CallExpr => (),
                // CXCursor_CXXThisExpr => (),
                // CXCursor_DeclRefExpr => (),
                // CXCursor_UnaryOperator => (),
                // CXCursor_BinaryOperator => (),
                // CXCursor_ConditionalOperator => (),
                // CXCursor_ParenExpr => (),
                // CXCursor_IntegerLiteral => (),
                // CXCursor_FloatingLiteral => (),
                // CXCursor_CXXThrowExpr => (),
                // CXCursor_ObjCStringLiteral => (),
                // CXCursor_CXXNullPtrLiteralExpr => (),
                // CXCursor_CompoundAssignOperator => (),
                // CXCursor_CXXStaticCastExpr => (),
                // CXCursor_CXXConstCastExpr => (),
                // CXCursor_CXXBoolLiteralExpr => (),
                // CXCursor_UnaryExpr => (),
                // //
                // CXCursor_CompoundStmt => (),
                // CXCursor_ReturnStmt => (),
                // CXCursor_DeclStmt => (),
                // CXCursor_IfStmt => (),
                // CXCursor_NullStmt => (),
                // //
                // CXCursor_UnexposedAttr => (),
                // 501
                CXCursor_MacroDefinition => (),
                // 502
                CXCursor_MacroExpansion => (),
                // 503
                CXCursor_InclusionDirective => (),
                // CXCursor_WarnUnusedResultAttr => (),
                // CXCursor_StaticAssert => (),
                _ => println!("{:?}", cursor),
            };
        }
    }

    // processc children
    // let mut data = visit_children(data, cursor);

    data.stack.pop();

    data
}

#[allow(non_upper_case_globals)]
impl Data {
    fn new() -> Data {
        Data {
            map: HashMap::new(),
            stack: Vec::new(),
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
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let tu = TranslationUnit::parse(args[0].as_str())?;
    stderr().flush().unwrap();    
    stdout().flush().unwrap();    

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

    let data = visit_children(data, root);

    // find "SliderFloat2"

    // traverse

    Ok(())
}
