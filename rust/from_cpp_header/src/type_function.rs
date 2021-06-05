use clang_sys::*;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{cx_string, visit_children_with, Visitor, Type, TypeMap};

pub struct Param {
    pub name: String,
}

pub struct Function {
    pub export_name: Option<String>,
    pub result: Rc<Type>,
    pub params: Vec<Param>,
}

impl Debug for Function {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

struct FunctionVisitor {
    cursor: CXCursor,
    result_type: CXType,
    params: Vec<Param>,
    export: bool,
    has_body: bool,
}

impl FunctionVisitor {
    fn new(cursor: CXCursor, result_type: CXType) -> FunctionVisitor {
        FunctionVisitor {
            cursor,
            result_type,
            params: Vec::new(),
            export: false,
            has_body: false,
        }
    }
}

#[allow(non_upper_case_globals)]
impl Visitor for FunctionVisitor {
    fn on_visit(
        &mut self,
        cursor: CXCursor,
        type_map: &mut TypeMap,        
    ) -> bool {

        match cursor.kind {
            CXCursor_CompoundStmt => {
                self.has_body = true;
            }

            CXCursor_TypeRef => {}

            CXCursor_WarnUnusedResultAttr => {}

            CXCursor_ParmDecl => {
                let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                // var childType = libclang.clang_getCursorType(child);
                // var typeRef = typeMap.CxTypeToType(childType, child);
                // type.Params.Add(new FunctionParam(type.Params.Count, childName, typeRef));
                self.params.push(Param { name });
            }

            CXCursor_DLLImport | CXCursor_DLLExport => {
                self.export = true;
            }

            CXCursor_UnexposedAttr => {}

            _ => {
                panic!("unknown param type");
            }
        }

        true
    }

    type Result = Function;

    fn result(&mut self, type_map: &mut TypeMap) -> Self::Result {
        let result = 
            type_map
            .type_from_cx_type(self.result_type, self.cursor);

        let export_name = if self.export {
            let mangling = cx_string::CXString::cursor_mangling(self.cursor).to_string();
            Some(mangling)
        } else {
            None
        };

        Function {
            export_name,
            result,
            params: self.params.drain(..).collect(),
        }
    }
}

impl Function {
    pub fn parse<'a>(cursor: CXCursor, type_map: &mut TypeMap, result_type: CXType) -> Function {
        visit_children_with(cursor, type_map, || {
            FunctionVisitor::new(cursor, result_type)
        })
    }
}
