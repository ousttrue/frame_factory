use clang_sys::*;
use std::{io::{stdout}, io::{stderr, Write}};

mod translation_unit;
pub use translation_unit::*;

mod cx_source_location;
mod cx_string;

mod args;
pub use args::*;

mod no_drop;
pub use no_drop::*;

mod visitor;
pub use visitor::*;

mod types;
pub use types::*;
mod type_map;
pub use type_map::*;

mod type_function;
pub use type_function::*;

mod type_typedef;
pub use type_typedef::*;

mod type_enum;
pub use type_enum::*;

mod type_struct;
pub use type_struct::*;

mod generator;

pub struct Root {
    stack: Vec<u32>,
    ns: Vec<String>,
    type_map: Option<TypeMap>,
}

impl Root
{
    pub fn get(&mut self)->&mut TypeMap
    {
        self.type_map.as_mut().unwrap()
    }    
}

impl Drop for Root {
    fn drop(&mut self) {
        println!("drop Data");
    }
}

#[allow(non_upper_case_globals)]
impl OnVisit for Root
{
    fn on_visit(&mut self, ptr: *mut Root, cursor: CXCursor, parent: CXCursor)->bool {
        let parent_is_null = unsafe { clang_Cursor_isNull(parent) } != 0;
        assert!(!parent_is_null);
        // assert!(data.stack.len() == 0);

        let spelling = cx_string::CXString::cursor_spelling(cursor);
        // let location = cx_source_location::CXSourceLocation::from_cursor(cursor);
   
        match cursor.kind {
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
                self.ns.push(spelling.to_string());
                visit_children(cursor, ptr);
                self.ns.pop();
            }
    
            CXCursor_UnexposedDecl => {
                self.ns.push(spelling.to_string());
                visit_children(cursor, ptr);
                self.ns.pop();
            }
    
            CXCursor_TypedefDecl => {
                let t = self.get().get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let def = Typedef::parse(cursor, self.get());
                    t.decl.replace(Decl::Typedef(def));
                }
            }
    
            CXCursor_FunctionDecl => {
                let t =self.get().get_or_create_user_type(cursor);                
                if let Type::UserType(t) = &*t
                {
                    let result_type = unsafe{clang_getCursorResultType(cursor)};
                    let f = Function::parse(cursor, result_type, self.get());
                    t.decl.replace(Decl::Function(f));
                }              
            }
    
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
                let t = self.get().get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let s = Struct::parse(cursor, self.get());
                    t.decl.replace(Decl::Struct(s));
                }

                // parse as namespace
                self.ns.push(spelling.to_string());
                visit_children(cursor, ptr);
                self.ns.pop();
            }
    
            CXCursor_FieldDecl => {
                // process in Struct::parse. skip
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
                let t = self.get().get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let cx_type = unsafe{clang_getEnumDeclIntegerType(cursor)};
                    let base_type = self.get().type_from_cx_type(cx_type, cursor);            
                    let e = Enum::parse(cursor, base_type);
                    t.decl.replace(Decl::Enum(e));
                }              
            }
    
            CXCursor_VarDecl => (),
    
            _ => panic!("unknown CXCursorKind"),
        }
    
        // processc children
    
        self.stack.pop();

        true
    }

    type Result = TypeMap;

    fn result(&mut self) -> Self::Result {
        self.type_map.take().unwrap()
    }
}

#[allow(non_upper_case_globals)]
impl Root {
    fn new() -> Root {
        Root {
            stack: Vec::new(),
            ns: Vec::new(),
            type_map: Some(TypeMap::new()),
        }
    }
}

pub fn run(args: &[String]) -> Result<(), Error> {
    // args
    let args= Args::parse(args);

    // parse
    let tu = TranslationUnit::parse(&args.exports[0].header, &args.compile_args)?;
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    // aggregate
    let type_map = visit_children_with(tu.get_cursor(), ||{
        Root::new()
    });

    // generate
    generator::generate(&type_map, &args).map_err(|e|Error::IOError(e))?;

    Ok(())
}
