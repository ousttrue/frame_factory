use clang_sys::*;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{cx_string, visit_children_with, OnVisit, Type, TypeMap};

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

struct FunctionVisitor<'a> {
    cursor: CXCursor,
    result_type: CXType,
    type_map: &'a mut TypeMap,
    params: Vec<Param>,
    export: bool,
    has_body: bool,
}

impl<'a> FunctionVisitor<'a> {
    fn new(cursor: CXCursor, result_type: CXType, type_map: &mut TypeMap) -> FunctionVisitor {
        FunctionVisitor {
            cursor,
            result_type,
            type_map,
            params: Vec::new(),
            export: false,
            has_body: false,
        }
    }
}

#[allow(non_upper_case_globals)]
impl<'a> OnVisit<FunctionVisitor<'a>> for FunctionVisitor<'a> {
    fn on_visit(
        &mut self,
        _ptr: *mut FunctionVisitor,
        cursor: CXCursor,
        _parent: CXCursor,
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

    fn result(&mut self) -> Self::Result {
        let result = self
            .type_map
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
    pub fn parse<'a>(cursor: CXCursor, result_type: CXType, type_map: &mut TypeMap) -> Function {
        visit_children_with(cursor, || {
            FunctionVisitor::new(cursor, result_type, type_map)
        })
    }
}
