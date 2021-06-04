use clang_sys::*;
use std::fmt::Debug;
use std::rc::Rc;

use crate::{cx_string, visit_children_with, OnVisit, Type, TypeMap};

pub struct Function {
    pub result: Rc<Type>,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

struct FunctionVisitor {
    function: Option<Function>,
}

impl FunctionVisitor {
    fn new(result_type: CXType, cursor: CXCursor, type_map: &TypeMap) -> FunctionVisitor {
        let result = type_map.type_from_cx_type(result_type, cursor);
        let function = Function { result };

        FunctionVisitor {
            function: Some(function),
        }
    }
}

#[allow(non_upper_case_globals)]
impl OnVisit<FunctionVisitor> for FunctionVisitor {
    fn on_visit(&mut self, ptr: *mut FunctionVisitor, cursor: CXCursor, parent: CXCursor) -> bool {
        match cursor.kind {
            CXCursor_CompoundStmt => {
                // type.HasBody = true;
            }

            CXCursor_TypeRef => {}

            CXCursor_WarnUnusedResultAttr => {}

            CXCursor_ParmDecl => {
                let child_name = cx_string::CXString::cursor_spelling(cursor);
                // var childType = libclang.clang_getCursorType(child);
                // var typeRef = typeMap.CxTypeToType(childType, child);
                // type.Params.Add(new FunctionParam(type.Params.Count, childName, typeRef));
            }

            CXCursor_DLLImport | CXCursor_DLLExport => {
                // type.DllExport = true;
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
        self.function.take().unwrap()
    }
}

impl Function {
    pub fn parse<'a>(result_type: CXType, cursor: CXCursor, type_map: &TypeMap) -> Function {
        visit_children_with(cursor, || {
            FunctionVisitor::new(result_type, cursor, type_map)
        })
    }
}
