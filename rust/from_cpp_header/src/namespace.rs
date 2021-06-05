use std::rc::Rc;

use clang_sys::*;

use crate::{Decl, Enum, Function, Struct, Type, TypeMap, Typedef, Visitor, cx_string, revisit_children, visit_children_with};

#[derive(Debug)]
pub struct Namespace {
    pub name: String,
    pub children: Vec<Rc<Namespace>>,
    pub members: Vec<Rc<Type>>,
}

impl Namespace
{
    pub fn debug_print(&self, parent: &str)
    {
        println!("{}::{}[{}]", parent, self.name, self.members.len());
        for child in &self.children
        {
            child.debug_print(&format!("{}::{}", parent, self.name));
        }
    }
}

pub struct NamespaceVisitor
{
    pub name: String,
    pub children: Vec<Rc<Namespace>>,
    pub members: Vec<Rc<Type>>,
}

impl Drop for NamespaceVisitor {
    fn drop(&mut self) {
        if self.members.len()>0 {
            // println!("close {}", self.name);
        }
    }
}

impl NamespaceVisitor
{
    pub fn new(name: String)->NamespaceVisitor
    {
        NamespaceVisitor
        {
            name,
            children: Vec::new(),
            members: Vec::new(),
        }
    }

    pub fn nameless()->NamespaceVisitor
    {
        Self::new("".to_owned())
    }
}

#[allow(non_upper_case_globals)]
impl Visitor for NamespaceVisitor
{
    fn on_visit(&mut self, cursor: CXCursor, type_map: &mut TypeMap)->bool {
  
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
                let name = cx_string::CXString::cursor_spelling(cursor).to_string();
                let child = visit_children_with(cursor, type_map, ||
                {
                    NamespaceVisitor::new(name)
                });                
                self.children.push(Rc::new(child));
            }
    
            CXCursor_UnexposedDecl => {
                revisit_children(cursor, self, type_map);
            }
    
            CXCursor_TypedefDecl => {
                let t = type_map.get_or_create_user_type(cursor);
                self.members.push(t.clone());
                if let Type::UserType(t) = &*t
                {
                    let def = Typedef::parse(cursor, type_map);
                    t.decl.replace(Decl::Typedef(def));
                }
            }
    
            CXCursor_FunctionDecl => {
                let t =type_map.get_or_create_user_type(cursor);                
                self.members.push(t.clone());
                if let Type::UserType(t) = &*t
                {
                    let result_type = unsafe{clang_getCursorResultType(cursor)};
                    let f = Function::parse(cursor, type_map, result_type);
                    t.decl.replace(Decl::Function(f));
                }              
            }
    
            CXCursor_StructDecl | CXCursor_ClassDecl | CXCursor_UnionDecl => {
                let t = type_map.get_or_create_user_type(cursor);
                self.members.push(t.clone());
                if let Type::UserType(t) = &*t
                {
                    let s = Struct::parse(cursor, type_map);
                    t.decl.replace(Decl::Struct(s));
                }
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
                self.members.push(t.clone());
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
    
        true
    }

    type Result = Namespace;

    fn result(&mut self, _type_map: &mut TypeMap) -> Self::Result {
        Namespace{
            name: self.name.clone(),
            children: self.children.drain(..).collect(),
            members: self.members.drain(..).collect(),
        }
    }
}
