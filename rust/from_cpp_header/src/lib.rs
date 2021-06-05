use clang_sys::*;
use std::{io::{stdout}, io::{stderr, Write}};

mod translation_unit;
pub use translation_unit::*;

mod cx_source_location;
mod cx_string;

mod args;
pub use args::*;

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
}

impl Drop for Root {
    fn drop(&mut self) {
        println!("drop Data");
    }
}

#[allow(non_upper_case_globals)]
impl OnVisit for Root
{
    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap)->bool {

        let ptr = self as *mut Root;

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
                // self.ns.push(spelling.to_string());
                // visit_children(cursor, Data{ptr, type_map});
                // self.ns.pop();
            }
    
            CXCursor_UnexposedDecl => {
                // self.ns.push(spelling.to_string());
                // visit_children(cursor, ptr);
                // self.ns.pop();
            }
    
            CXCursor_TypedefDecl => {
                let t = type_map.get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let def = Typedef::parse(cursor, type_map);
                    t.decl.replace(Decl::Typedef(def));
                }
            }
    
            CXCursor_FunctionDecl => {
                let t =type_map.get_or_create_user_type(cursor);                
                if let Type::UserType(t) = &*t
                {
                    let result_type = unsafe{clang_getCursorResultType(cursor)};
                    let f = Function::parse(cursor, type_map, result_type);
                    t.decl.replace(Decl::Function(f));
                }              
            }
    
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
                let t = type_map.get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let s = Struct::parse(cursor, type_map);
                    t.decl.replace(Decl::Struct(s));
                }

                // parse as namespace
                // self.ns.push(spelling.to_string());
                // visit_children(cursor, ptr);
                // self.ns.pop();
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
                let t = type_map.get_or_create_user_type(cursor);
                if let Type::UserType(t) = &*t
                {
                    let cx_type = unsafe{clang_getEnumDeclIntegerType(cursor)};
                    let base_type = type_map.type_from_cx_type(cx_type, cursor);            
                    let e = Enum::parse(cursor, type_map, base_type);
                    t.decl.replace(Decl::Enum(e));
                }              
            }
    
            CXCursor_VarDecl => (),
    
            _ => panic!("unknown CXCursorKind"),
        }
    
        // processc children
    
        // self.stack.pop();

        true
    }

    type Result = ();

    fn result(&mut self, _type_map: &mut TypeMap) -> Self::Result {
        ()
    }
}

pub fn run(args: &[String]) -> Result<(), Error> {
    // args
    let args= Args::parse(args);

    // parse
    let tu = TranslationUnit::parse(&args.exports[0].header, &args.compile_args)?;
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    let mut type_map = TypeMap::new();

    // aggregate
    visit_children_with(tu.get_cursor(), &mut type_map, ||{
        Root{}
    });

    // generate
    generator::generate(&type_map, &args).map_err(|e|Error::IOError(e))?;

    Ok(())
}
