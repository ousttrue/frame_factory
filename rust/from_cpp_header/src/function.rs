use clang_sys::*;

use crate::{cx_string, visit_children, OnVisit, TypeMap};

pub struct Function {
    pub name: String,
}

pub struct FunctionVisitor {
    function: Function
}

#[allow(non_upper_case_globals)]
impl OnVisit<FunctionVisitor> for FunctionVisitor {
    fn on_visit(&mut self, ptr: *mut FunctionVisitor, cursor: CXCursor, parent: CXCursor) {
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
    }
}

impl Function {
    pub fn parse(cursor: CXCursor, type_map: &TypeMap) -> Function {
        let result = unsafe { clang_getCursorResultType(cursor) };
        let spelling = cx_string::CXString::cursor_spelling(cursor);
        let mut function = Function {
            name: spelling.to_string(),
        };

        let visitor = Box::new(FunctionVisitor {
            function
        });
        let ptr = Box::into_raw(visitor);
        visit_children(ptr, cursor);
        // for drop
        let visitor = unsafe { Box::from_raw(ptr) };

        visitor.function
    }
}
