use std::rc::Rc;

use clang_sys::*;

use crate::{OnVisit, Type, cx_string, visit_children_with};

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

impl OnVisit<EnumVisitor> for EnumVisitor {
    type Result = Enum;

    fn on_visit(&mut self, ptr: *mut EnumVisitor, cursor: CXCursor, parent: CXCursor) -> bool {
        match cursor.kind {
            CXCursorEnumConstantDecl => {
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

    fn result(&mut self) -> Self::Result {
        Enum {
            base_type: self.base_type.clone(),
            entries: self.entries.drain(..).collect(),
        }
    }
}

impl Enum {
    pub fn parse(cursor: CXCursor, base_type: Rc<Type>) -> Enum {

        visit_children_with(cursor, || EnumVisitor {
            base_type,
            entries: Vec::new(),
        })
    }
}
