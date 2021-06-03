use std::rc::Rc;

use clang_sys::*;

use crate::{OnVisit, Type, TypeMap, cx_string, visit_children, visit_children_with};

pub struct Function {
    pub name: String,
    pub result: Rc<Type>,
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
    pub fn parse<'a>(cursor: CXCursor, type_map: &TypeMap) -> Function {
        let result = type_map.type_from_cx_cursor(cursor);
        let name = cx_string::CXString::cursor_spelling(cursor).to_string();
        let mut function = Function {
            name,
            result,
        };

        visit_children_with(cursor, ||{
            FunctionVisitor {
                function: &mut function
            }
        });

        function
    }
}
