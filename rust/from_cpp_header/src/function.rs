use clang_sys::*;

use crate::{OnVisit, TypeMap, cx_string, visit_children, visit_children_with};

pub struct Function {
    pub name: String,
}

struct FunctionVisitor<'a> {
    function: &'a mut Function
}

#[allow(non_upper_case_globals)]
impl<'a> OnVisit<FunctionVisitor<'a>> for FunctionVisitor<'a> {
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

        visit_children_with(cursor, ||{
            FunctionVisitor {
                function: &mut function
            }
        });

        function
    }
}
