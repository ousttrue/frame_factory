use clang_sys::*;
use std::{fs, io::{BufWriter, stdout}, io::{stderr, Write}};

mod translation_unit;
pub use translation_unit::*;

mod cx_source_location;
mod cx_string;

mod visitor;
pub use visitor::*;

mod type_map;
pub use type_map::*;

mod function;
pub use function::*;

mod typedef;
pub use typedef::*;

pub struct Root {
    stack: Vec<u32>,
    ns: Vec<String>,
    type_map: Option<TypeMap>,
}

impl Root
{
    pub fn get(&self)->&TypeMap
    {
        self.type_map.as_ref().unwrap()
    }    
}

impl Drop for Root {
    fn drop(&mut self) {
        println!("drop Data");
    }
}

#[allow(non_upper_case_globals)]
impl OnVisit<Root> for Root
{
    fn on_visit(&mut self, t: *mut Root, cursor: CXCursor, parent: CXCursor)->bool {
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
                visit_children(cursor, t);
                self.ns.pop();
            }
    
            CXCursor_UnexposedDecl => {
                self.ns.push(spelling.to_string());
                visit_children(cursor, t);
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
                    let f = Function::parse(result_type, cursor, self.get());
                    t.decl.replace(Decl::Function(f));
                }              
            }
    
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
                // var reference = m_typeMap.GetOrCreate(cursor);
                // var structType = StructType.Parse(cursor, m_typeMap);
                // reference.Type = structType;
                self.ns.push(spelling.to_string());
                visit_children(cursor, t);
                self.ns.pop();
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

struct Export
{
    header: String,
    dll: String,
}

struct Args
{
    exports: Vec<Export>,
    dst: String,
}

impl Args
{
    fn parse(args: &[String])->Args
    {
        let mut exports: Vec<Export> = Vec::new();
        let mut dst = String::new();

        for arg in args
        {
            if arg.starts_with("-E")
            {
                let split: Vec<&str> = arg[2..].rsplitn(2, ",").collect();
                exports.push(Export{
                    header: split[1].to_owned(),
                    dll: split[0].to_owned(),
                });
            }
            else if arg.starts_with("-D")
            {
                dst = arg[2..].to_owned();
            }
            else{
                panic!()
            }
        }

        Args
        {
            exports,
            dst
        }
    }
}

pub fn run(args: &[String]) -> Result<TypeMap, Error> {
    // args
    let args= Args::parse(args);

    // parse
    let tu = TranslationUnit::parse(&args.exports[0].header)?;
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    // aggregate
    let type_map = visit_children_with(tu.get_cursor(), ||{
        Root::new()
    });

    // generate
    let dst = std::path::Path::new(&args.dst);
    fs::create_dir_all(dst.parent().unwrap()).unwrap();

    let mut f = BufWriter::new(fs::File::create(dst).unwrap());     

    for (k, v) in type_map.get().iter()
    {
        if let Type::UserType(t) = &**v
        {
            if t.file.ends_with("/imgui.h")
            {
                f.write_fmt(format_args!("// {:?}\n", v));
            }
        }
    }

    Ok(type_map)
}
