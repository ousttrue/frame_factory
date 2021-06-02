use clang_sys::*;
use std::{
    collections::HashMap,
    ffi::c_void,
    io::stdout,
    io::{stderr, Write},
    ops::{Deref, DerefMut},
};

mod translation_unit;
pub use translation_unit::*;

mod cxsourcelocation;
mod cxstring;

enum Kind {
    None,
    Namespace(String),
    Function(String),
}

struct Value {
    hash: u32,
    parent_hash: u32,
    kind: Kind,
}

#[allow(non_upper_case_globals)]
impl Value {
    pub fn new(cursor: CXCursor, parent_hash: u32) -> Value {
        let hash = unsafe { clang_hashCursor(cursor) };

        match cursor.kind {
            CXCursor_Namespace => {
                let spelling = cxstring::CXString::from_cursor(cursor);
                Value {
                    hash,
                    parent_hash,
                    kind: Kind::Namespace(spelling.to_string()),
                }
            }
            CXCursor_FunctionDecl => {
                let spelling = cxstring::CXString::from_cursor(cursor);
                Value {
                    hash,
                    parent_hash,
                    kind: Kind::Function(spelling.to_string()),
                }
            }
            _ => Value {
                hash,
                parent_hash,
                kind: Kind::None,
            },
        }
    }
}

struct Data {
    map: HashMap<u32, Value>,
    stack: Vec<u32>,
    ns: Vec<String>,
}

impl Drop for Data {
    fn drop(&mut self) {
        println!("drop Data");
    }
}

struct NoDropData {
    ptr: *mut Data,
    data: Option<Box<Data>>,
}

impl Drop for NoDropData {
    fn drop(&mut self) {
        if let Some(box_data) = self.data.take() {
            Box::into_raw(box_data);
        }
    }
}

impl Deref for NoDropData {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        self.data.as_ref().unwrap()
    }
}

impl DerefMut for NoDropData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.as_mut().unwrap()
    }
}

impl NoDropData {
    pub fn new(data: *mut Data) -> NoDropData {
        NoDropData {
            ptr: data,
            data: Some(unsafe { Box::from_raw(data) }),
        }
    }
}

struct ParentIterator<'a> {
    current: Option<&'a Value>,
    data: &'a Data,
}

impl<'a> Iterator for ParentIterator<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current {
            if let Some(value) = self.data.map.get(&current.parent_hash) {
                self.current = Some(value);
                return Some(&value);
            }
        }

        self.current = None;
        None
    }
}

extern "C" fn visitor(
    cursor: CXCursor,
    parent: CXCursor,
    data: CXClientData,
) -> CXChildVisitResult {
    on_visit(NoDropData::new(data as *mut Data), cursor, parent);

    CXChildVisit_Continue
}

fn visit_children(data: *mut Data, cursor: CXCursor) {
    unsafe { clang_visitChildren(cursor, visitor, data as *mut c_void) };
}

#[allow(non_upper_case_globals)]
fn on_visit(mut data: NoDropData, cursor: CXCursor, parent: CXCursor) {
    let parent_is_null = unsafe { clang_Cursor_isNull(parent) } != 0;
    assert!(!parent_is_null);
    // assert!(data.stack.len() == 0);

    let spelling = cxstring::CXString::from_cursor(cursor);
    let location = cxsourcelocation::CXSourceLocation::from_cursor(cursor);

    //
    // process current
    //
    {
        let parent_value = data.get_cursor_value(parent).unwrap();
        let value = Value::new(cursor, parent_value.hash);
        data.stack.push(value.hash);
        data.map.insert(value.hash, value);
    }

    match (cursor.kind) {
        // skip
        CXCursor_InclusionDirective
        | CXCursor_ClassTemplate
        | CXCursor_ClassTemplatePartialSpecialization
        | CXCursor_FunctionTemplate
        | CXCursor_UsingDeclaration
        | CXCursor_StaticAssert
        // | CXCursor_FirstExpr
        | CXCursor_AlignedAttr
        | CXCursor_CXXAccessSpecifier
        | CXCursor_Constructor
        | CXCursor_Destructor
        | CXCursor_ConversionFunction => (),

        CXCursor_MacroDefinition => {
            // m_typeMap.ParseMacroDefinition(cursor);
        }

        CXCursor_MacroExpansion => {
            //
        }

        CXCursor_Namespace => {
            data.ns.push(spelling.to_string());
            visit_children(data.ptr, cursor);
            data.ns.pop();
        }

        CXCursor_UnexposedDecl => {
            data.ns.push(spelling.to_string());
            visit_children(data.ptr, cursor);
            data.ns.pop();
        }

        CXCursor_TypedefDecl => {
            // var reference = m_typeMap.GetOrCreate(cursor);
            // reference.Type = TypedefType.Parse(cursor, m_typeMap);
        }

        CXCursor_FunctionDecl => {
            // var reference = m_typeMap.GetOrCreate(cursor);
            // reference.Type = FunctionType.Parse(cursor, m_typeMap);
        }

        CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
            // var reference = m_typeMap.GetOrCreate(cursor);
            // var structType = StructType.Parse(cursor, m_typeMap);
            // reference.Type = structType;
            data.ns.push(spelling.to_string());
            visit_children(data.ptr, cursor);
            data.ns.pop();
            // // if (libclang.clang_Cursor_isAnonymousRecordDecl(cursor) != 0)
            // if (libclang.clang_Cursor_isAnonymous(cursor) != 0)
            // {
            //     // anonymous type decl add field to current struct.
            //     structType.AnonymousParent = context.Current;
            //     var fieldOffset = (uint)libclang.clang_Cursor_getOffsetOfField(cursor);
            //     var current = context.Current;
            //     // var fieldName = cursor.Spelling();
            //     // FIXME: anonymous type field offset ?
            //     if (current != null)
            //     {
            //         current.Fields.Add(new StructField(current.Fields.Count, "", reference, 0));
            //     }
            //     else
            //     {
            //         var a = 0;
            //     }
            // }
        }

        CXCursor_FieldDecl => {
            // var fieldName = cursor.Spelling();
            // var fieldOffset = (uint)libclang.clang_Cursor_getOffsetOfField(cursor);
            // var fieldType = libclang.clang_getCursorType(cursor);
            // var current = context.Current;
            // if (!string.IsNullOrEmpty(fieldName) && current.Fields.Any(x => x.Name == fieldName))
            // {
            //     throw new Exception();
            // }
            // current.Fields.Add(new StructField(current.Fields.Count, fieldName, m_typeMap.CxTypeToType(fieldType, cursor).Item1, fieldOffset));
            // break;
        }

        CXCursor_CXXBaseSpecifier => {
            // var referenced = libclang.clang_getCursorReferenced(cursor);
            // var baseClass = m_typeMap.GetOrCreate(referenced);
            // context.Current.BaseClass = baseClass;
        }

        CXCursor_UnexposedAttr => {
            // var src = m_typeMap.GetSource(cursor);
            // if (StructType.TryGetIID(src, out Guid iid))
            // {
            //     context.Current.IID = iid;
            // }
        }

        CXCursor_CXXMethod => {
            // var method = FunctionType.Parse(cursor, m_typeMap);
            // if (!method.HasBody)
            // {
            //     context.Current.Methods.Add(method);
            // }
        }

        CXCursor_EnumDecl => {
            // var reference = m_typeMap.GetOrCreate(cursor);
            // reference.Type = EnumType.Parse(cursor);
        }

        CXCursor_VarDecl => (),

        _ => panic!("unknown CXCursorKind"),
    }

    // processc children

    data.stack.pop();
}

#[allow(non_upper_case_globals)]
impl Data {
    fn new() -> Data {
        Data {
            map: HashMap::new(),
            stack: Vec::new(),
            ns: Vec::new(),
        }
    }

    fn get_namespace(&self) -> String {
        let mut s = String::with_capacity(self.ns.iter().map(|x| x.len() + 2).sum());
        for x in &self.ns {
            s.push_str(x.as_str());
            s.push_str("::");
        }
        s
    }

    // fn push_cursor_value(&mut self, cursor: CXCursor, value: Value) {
    //     let hash = unsafe { clang_hashCursor(cursor) };
    //     self.map.insert(hash, value);
    // }

    fn get_cursor_value(&self, cursor: CXCursor) -> Option<&Value> {
        let hash = unsafe { clang_hashCursor(cursor) };
        if let Some(value) = self.map.get(&hash) {
            Some(value)
        } else {
            None
        }
    }

    fn iter_parents(&self, cursor: CXCursor) -> ParentIterator {
        ParentIterator {
            current: self.get_cursor_value(cursor),
            data: self,
        }
    }

    fn on_function(&mut self, cursor: CXCursor) {
        for parent in self.iter_parents(cursor) {}

        let location = cxsourcelocation::CXSourceLocation::from_cursor(cursor);
        if !location.is_null() {
            let file = location.get_path();
            let spelling = cxstring::CXString::from_cursor(cursor);
            match file.file_name() {
                Some(name) if name == "imgui.h" => {
                    println!("{}({})", spelling.to_string(), file.to_string_lossy())
                }
                _ => (),
            }
        }
    }
}

pub fn run(args: &[String]) -> Result<(), Error> {
    let tu = TranslationUnit::parse(args[0].as_str())?;
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    let mut data = Box::new(Data::new());

    // traverse(&tu.get_cursor());
    let root = tu.get_cursor();
    let hash = unsafe { clang_hashCursor(root) };
    data.map.insert(
        hash,
        Value {
            kind: Kind::None,
            parent_hash: 0,
            hash,
        },
    );

    let p = Box::into_raw(data);
    visit_children(p, root);
    let data = unsafe { Box::from_raw(p) };

    // find "SliderFloat2"

    // traverse

    Ok(())
}
