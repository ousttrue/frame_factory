use std::rc::Rc;

use clang_sys::*;

use crate::{Visitor, Type, TypeMap, cx_string, visit_children_with};

#[derive(Debug)]
pub struct EnumEntry {
    pub name: String,
    pub value: i64,
}

#[derive(Debug)]
pub struct Enum {
    pub base_type: Rc<Type>,
    pub entries: Vec<EnumEntry>,
}

struct EnumVisitor {
    base_type: Rc<Type>,
    entries: Vec<EnumEntry>,
}

#[allow(non_upper_case_globals, non_snake_case)]
impl Visitor for EnumVisitor {
    type Result = Enum;

    fn on_visit(&mut self, cursor: CXCursor, _type_map: &mut TypeMap) -> bool {
        match cursor.kind {
            CXCursor_EnumConstantDecl => {
                let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                let value = unsafe { clang_getEnumConstantDeclValue(cursor) };
                self.entries.push(EnumEntry {
                    name,
                    value: value,
                });
                true
            }
            _ => panic!(),
        }
    }

    fn result(&mut self, _type_map: &mut TypeMap) -> Self::Result {
        Enum {
            base_type: self.base_type.clone(),
            entries: self.entries.drain(..).collect(),
        }
    }
}

impl Enum {
    pub fn parse(cursor: CXCursor, type_map: &mut TypeMap, base_type: Rc<Type>) -> Enum {

        visit_children_with(cursor, type_map, || EnumVisitor {
            base_type,
            entries: Vec::new(),
        })
    }
}
