use std::{collections::HashMap, ffi::c_void, ptr};

mod translation_unit;
pub use translation_unit::*;

mod cxstring;
pub use cxstring::*;

mod cxsourcelocation;
pub use cxsourcelocation::*;

struct Data {
    map: HashMap<u32, u32>,
}

impl Data {
    fn new() -> Data {
        Data {
            map: HashMap::new(),
        }
    }

    fn push_cursor_value(&mut self, cursor: clang_sys::CXCursor, value: u32) {
        let hash = unsafe { clang_sys::clang_hashCursor(cursor) };
        self.map.insert(hash, value);
    }

    fn get_cursor_value(&self, cursor: clang_sys::CXCursor) -> Option<u32> {
        let hash = unsafe { clang_sys::clang_hashCursor(cursor) };
        if let Some(value) = self.map.get(&hash) {
            Some(value.clone())
        } else {
            None
        }
    }

    fn on_function(&mut self, cursor: clang_sys::CXCursor, parent: clang_sys::CXCursor) {
        let spelling = CXString::from_cursor(cursor);

        let location = CXSourceLocation::from_cursor(cursor);
        if !location.is_null() {
            let file = location.get_path();
            match file.file_name() {
                Some(name) if name == "imgui.h" => {
                    println!("{}({})", spelling.to_string(), file.to_string_lossy())
                },
                _ => (),
            }
        }
    }

    fn on_visit(&mut self, cursor: clang_sys::CXCursor, parent: clang_sys::CXCursor) {
        let parent_is_null = unsafe { clang_sys::clang_Cursor_isNull(parent) } != 0;
        if parent_is_null {
            panic!();
        }

        let value = self.get_cursor_value(parent).unwrap();

        match cursor.kind {
            clang_sys::CXCursor_UnexposedDecl => (),
            clang_sys::CXCursor_StructDecl => (),
            clang_sys::CXCursor_UnionDecl => (),
            clang_sys::CXCursor_EnumDecl => (),
            clang_sys::CXCursor_EnumConstantDecl => (),
            clang_sys::CXCursor_FieldDecl => (),
            clang_sys::CXCursor_VarDecl => (),
            clang_sys::CXCursor_FunctionDecl => self.on_function(cursor, parent),
            clang_sys::CXCursor_ParmDecl => (),
            clang_sys::CXCursor_TypedefDecl => (),
            clang_sys::CXCursor_Namespace => (),
            clang_sys::CXCursor_NamespaceRef => (),
            clang_sys::CXCursor_ConversionFunction => (),
            clang_sys::CXCursor_CXXMethod => (),
            clang_sys::CXCursor_Constructor => (),
            clang_sys::CXCursor_Destructor => (),
            clang_sys::CXCursor_FunctionTemplate => (),
            clang_sys::CXCursor_ClassTemplate => (),
            clang_sys::CXCursor_TypeRef => (),
            clang_sys::CXCursor_TemplateRef => (),
            clang_sys::CXCursor_TemplateTypeParameter => (),
            clang_sys::CXCursor_OverloadedDeclRef => (),
            clang_sys::CXCursor_NonTypeTemplateParameter => (),
            clang_sys::CXCursor_ClassTemplatePartialSpecialization => (),
            clang_sys::CXCursor_UsingDeclaration => (),
            //
            clang_sys::CXCursor_ArraySubscriptExpr => (),
            clang_sys::CXCursor_UnexposedExpr => (),
            clang_sys::CXCursor_MemberRefExpr => (),
            clang_sys::CXCursor_CStyleCastExpr => (),
            clang_sys::CXCursor_StringLiteral => (),
            clang_sys::CXCursor_CallExpr => (),
            clang_sys::CXCursor_CXXThisExpr => (),
            clang_sys::CXCursor_DeclRefExpr => (),
            clang_sys::CXCursor_UnaryOperator => (),
            clang_sys::CXCursor_BinaryOperator => (),
            clang_sys::CXCursor_ConditionalOperator => (),
            clang_sys::CXCursor_ParenExpr => (),
            clang_sys::CXCursor_IntegerLiteral => (),
            clang_sys::CXCursor_FloatingLiteral => (),
            clang_sys::CXCursor_CXXThrowExpr => (),
            clang_sys::CXCursor_ObjCStringLiteral => (),
            clang_sys::CXCursor_CXXNullPtrLiteralExpr => (),
            clang_sys::CXCursor_CompoundAssignOperator => (),
            clang_sys::CXCursor_CXXStaticCastExpr => (),
            clang_sys::CXCursor_CXXConstCastExpr => (),
            clang_sys::CXCursor_CXXBoolLiteralExpr => (),
            clang_sys::CXCursor_UnaryExpr => (),
            //
            clang_sys::CXCursor_CompoundStmt => (),
            clang_sys::CXCursor_ReturnStmt => (),
            clang_sys::CXCursor_DeclStmt => (),
            clang_sys::CXCursor_IfStmt => (),
            clang_sys::CXCursor_NullStmt => (),
            //
            clang_sys::CXCursor_UnexposedAttr => (),
            clang_sys::CXCursor_MacroDefinition => (),
            clang_sys::CXCursor_MacroExpansion => (),
            clang_sys::CXCursor_InclusionDirective => (),
            clang_sys::CXCursor_WarnUnusedResultAttr => (),
            clang_sys::CXCursor_StaticAssert => (),
            _ => println!("{:?}", cursor),
        };
        self.push_cursor_value(cursor, value + 1);
    }
}

fn get_indent(level: u32) -> String {
    let mut indent = String::new();

    for _ in 0..level {
        indent.push_str("  ");
    }

    indent
}

extern "C" fn visitor(
    cursor: clang_sys::CXCursor,
    parent: clang_sys::CXCursor,
    data: clang_sys::CXClientData,
) -> clang_sys::CXChildVisitResult {
    let mut data: Box<Data> = unsafe { Box::from_raw(data as *mut Data) };
    data.on_visit(cursor, parent);

    // avoid drop
    Box::into_raw(data);

    clang_sys::CXChildVisit_Recurse
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let tu = TranslationUnit::parse(args[0].as_str())?;

    let mut data = Box::new(Data::new());

    // traverse(&tu.get_cursor());
    let root = tu.get_cursor();
    data.push_cursor_value(root, 0);

    let p = Box::into_raw(data);
    unsafe { clang_sys::clang_visitChildren(root, visitor, p as *mut c_void) };

    Ok(())
}
