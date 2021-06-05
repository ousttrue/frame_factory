use std::{rc::Rc};

use clang_sys::*;

use crate::{cx_string, visit_children_with, OnVisit, Type, TypeMap};

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
}

struct StructVisitor<'a> {
    cursor: CXCursor,
    type_map: &'a mut TypeMap,
    fields: Vec<Field>,
}

#[allow(non_upper_case_globals)]
impl<'a> OnVisit for StructVisitor<'a> {
    type Result = Struct;

    fn on_visit(
        &mut self,
        _ptr: *mut StructVisitor<'a>,
        cursor: CXCursor,
        _parent: CXCursor,
    ) -> bool {
        match cursor.kind {
            CXCursor_FieldDecl => {
                let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                let cx_type = unsafe { clang_getCursorType(cursor) };
                let field_type = self.type_map.type_from_cx_type(cx_type, cursor);
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
                    let field_type = self.type_map.get_or_create_user_type(cursor);
                    let offset = unsafe { clang_Cursor_getOffsetOfField(cursor) };
                    self.fields.push(Field {
                        name,
                        field_type,
                        offset,
                    });
                }
            }

            _ => (),
        };

        true
    }

    fn result(&mut self) -> Self::Result {
        let is_union = self.cursor.kind == CXCursor_UnionDecl;
        let cx_type = unsafe { clang_getCursorType(self.cursor) };
        let size = unsafe { clang_Type_getSizeOf(cx_type) } as usize;

        Struct {
            fields: self.fields.drain(..).collect(),
            is_union,
            size,
        }
    }
}

impl Struct {
    pub fn parse(cursor: CXCursor, type_map: &mut TypeMap) -> Struct {
        visit_children_with(cursor, || StructVisitor {
            cursor,
            type_map,
            fields: Vec::new(),
        })
    }
}
