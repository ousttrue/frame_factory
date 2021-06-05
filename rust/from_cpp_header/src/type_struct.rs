use std::rc::Rc;

use clang_sys::*;

use crate::{cx_string, visit_children_with, Namespace, NamespaceVisitor, Type, TypeMap, Visitor};

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub field_type: Rc<Type>,
    pub offset: i64,
}

#[derive(Debug)]
pub struct Struct {
    pub is_union: bool,
    pub size: usize,
    pub fields: Vec<Field>,
    // for struct inner types
    pub namespace: Namespace,
}

struct StructVisitor {
    cursor: CXCursor,
    fields: Vec<Field>,
    namespace_visitor: NamespaceVisitor,
}

#[allow(non_upper_case_globals)]
impl Visitor for StructVisitor {
    type Result = Struct;

    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap) -> bool {
        match cursor.kind {
            CXCursor_FieldDecl => {
                let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                let cx_type = unsafe { clang_getCursorType(cursor) };
                let field_type = type_map.type_from_cx_type(cx_type, cursor);
                let offset = unsafe { clang_Cursor_getOffsetOfField(cursor) };
                self.fields.push(Field {
                    name,
                    field_type,
                    offset,
                });
            }

            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
                // anonymous field
                if unsafe { clang_Cursor_isAnonymous(cursor) } != 0 {
                    let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                    let field_type = type_map.get_or_create_user_type(cursor);
                    let offset = unsafe { clang_Cursor_getOffsetOfField(cursor) };
                    self.fields.push(Field {
                        name,
                        field_type,
                        offset,
                    });
                }
            }

            _ => {
                self.namespace_visitor.on_visit(cursor, type_map);
            }
        };

        true
    }

    fn result(&mut self, type_map: &mut TypeMap) -> Self::Result {
        let is_union = self.cursor.kind == CXCursor_UnionDecl;
        let cx_type = unsafe { clang_getCursorType(self.cursor) };
        let size = unsafe { clang_Type_getSizeOf(cx_type) } as usize;

        Struct {
            fields: self.fields.drain(..).collect(),
            is_union,
            size,
            namespace: self.namespace_visitor.result(type_map),
        }
    }
}

impl Struct {
    pub fn parse(cursor: CXCursor, type_map: &mut TypeMap) -> Struct {
        let name = cx_string::CXString::cursor_spelling(cursor).to_string();
        visit_children_with(cursor, type_map, || StructVisitor {
            cursor,
            fields: Vec::new(),
            namespace_visitor: NamespaceVisitor::new(format!("[{}]", name)),
        })
    }
}
