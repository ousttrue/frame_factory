use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    rc::Rc,
};

use serde::Serialize;
use tera::Tera;

use crate::{
    c_macro, Decl, Export, Function, Primitives, Type, TypeMap, UserType,
};

fn escape_symbol(src: &str, i: usize) -> Cow<str> {
    match src {
        "" => format!("_{}", i).into(),
        "type" | "in" | "ref" | "mod" => format!("r#{}", src).into(),
        _ => src.into(),
    }
}

fn rename_type(t: &str) -> String {
    match t {
        "size_t" => "usize".into(),
        "va_list" => "va_list::VaList".into(),
        //
        "Sint8" => "i8".into(),
        "Sint16" => "i16".into(),
        "Sint32" => "i32".into(),
        "Sint64" => "i64".into(),
        "Uint8" => "u8".into(),
        "Uint16" => "u16".into(),
        "Uint32" => "u32".into(),
        "Uint64" => "u64".into(),
        //
        "int8_t" => "i8".into(),
        "int16_t" => "i16".into(),
        "int32_t" => "i32".into(),
        "int64_t" => "i64".into(),
        "uint8_t" => "u8".into(),
        "uint16_t" => "u16".into(),
        "uint32_t" => "u32".into(),
        "uint64_t" => "u64".into(),
        //
        "HWND__" => "c_void".into(),
        "HDC__" => "c_void".into(),
        "HINSTANCE__" => "c_void".into(),
        _ => t.into(),
    }
}

#[derive(Clone, Copy)]
struct TypeContext {
    is_const: bool,
    is_argument: bool,
}

impl TypeContext {
    fn mutable(&self) -> TypeContext {
        TypeContext {
            is_const: false,
            ..*self
        }
    }
}

fn get_rust_type(t: &Type, context: TypeContext) -> Cow<'static, str> {
    match t {
        Type::Primitive(Primitives::Void) => "c_void".into(),
        Type::Primitive(Primitives::Bool) => "bool".into(),
        Type::Primitive(Primitives::I8) => "i8".into(),
        Type::Primitive(Primitives::I16) => "i16".into(),
        Type::Primitive(Primitives::I32) => "i32".into(),
        Type::Primitive(Primitives::I64) => "i64".into(),
        Type::Primitive(Primitives::U8) => "u8".into(),
        Type::Primitive(Primitives::U16) => "u16".into(),
        Type::Primitive(Primitives::U32) => "u32".into(),
        Type::Primitive(Primitives::U64) => "u64".into(),
        Type::Primitive(Primitives::F32) => "f32".into(),
        Type::Primitive(Primitives::F64) => "f64".into(),
        Type::Pointer(pointee) => {
            let pointee_type = get_rust_type(&pointee, context.mutable());
            format!(
                "*{} {}",
                if context.is_const { "const" } else { "mut" },
                pointee_type
            )
            .into()
        }
        Type::Array(element, size) => {
            let element_type = get_rust_type(&*element, context.mutable());
            if context.is_argument {
                // not FFI-safe warning
                format!(
                    "*{} {}",
                    if context.is_const { "const" } else { "mut" },
                    element_type
                )
                .into()
            } else {
                format!("[{}; {}]", element_type, size).into()
            }
        }
        Type::UserType(u) => match &*u.get_decl() {
            Decl::Enum(e) => get_rust_type(&*e.base_type, context).into(),
            Decl::Typedef(d) => get_rust_type(&*d.base_type, context).into(),
            Decl::Struct(_) => u.get_name().to_owned().into(),
            // to function pointer
            Decl::Function(f) => {
                let mut params = String::new();
                for p in &f.params {
                    std::fmt::Write::write_fmt(
                        &mut params,
                        format_args!("{},", get_rust_type(&*p.param_type, context.mutable())),
                    )
                    .unwrap();
                }
                format!(
                    "extern fn({}) -> {}",
                    params,
                    get_rust_type(&*f.result, context.mutable())
                )
                .into()
            }
            Decl::None => rename_type(&u.get_name()).into(),
        },
        _ => format!("unknown").into(),
    }
}

fn get_sorted_entries<'a, F: Fn(&UserType) -> bool>(
    type_map: &'a TypeMap,
    path: &Path,
    f: F,
) -> Vec<&'a UserType> {
    let mut user_types: Vec<&UserType> = type_map
        .map
        .iter()
        .filter_map(|(_k, v)| {
            if let Type::UserType(t) = &**v {
                if t.file == path {
                    if f(&*t) {
                        return Some(t);
                    }
                }
            }

            None
        })
        .collect();

    user_types.sort_by(|a, b| {
        let line = a.line.cmp(&b.line);
        if let Ordering::Equal = line {
            a.column.cmp(&b.column)
        } else {
            line
        }
    });

    user_types
}

fn enum_value(value: i64) -> String {
    if value > 0 {
        format!("0x{:x}", value as i32)
    } else {
        value.to_string()
    }
}

fn has_default_type(t: &Rc<Type>) -> bool {
    match &**t {
        Type::Pointer(_) => false,
        Type::Array(_, _) => false,
        Type::UserType(u) => match &*u.get_decl() {
            Decl::Typedef(d) => has_default_type(&d.base_type),
            _ => false,
        },
        _ => true,
    }
}

fn write_function(name: &str, _t: &UserType, f: &Function) -> Option<String> {
    let export_name = f.export_name.as_ref()?;
    if export_name.len() == 0 {
        return None;
    }

    let mut params = String::new();
    let pw = &mut params as &mut dyn std::fmt::Write;
    let mut comment = "\n".to_owned();
    let cw = &mut comment as &mut dyn std::fmt::Write;

    if f.params.len() > 0 {
        pw.write_str("\n").unwrap();

        let mut i = 0;
        for param in &f.params {
            pw.write_fmt(format_args!(
                "        {}: {},\n",
                escape_symbol(&param.name, i),
                get_rust_type(
                    &*param.param_type,
                    TypeContext {
                        is_argument: true,
                        is_const: param.is_const
                    }
                )
            ))
            .unwrap();

            // default value
            let default_value = if let Some(default_value) = &param.default_value {
                default_value
            } else {
                ""
            };
            cw.write_fmt(format_args!(
                "    /// * {}: {}\n",
                param.name, default_value
            ))
            .unwrap();

            i += 1;
        }

        pw.write_str("    ").unwrap();
    }

    let mut result = comment;
    if export_name != name {
        result.push_str(&format!("    #[link_name = \"{}\"]\n", export_name));
    }
    result.push_str(&format!(
        "    pub fn {}({}) -> {};",
        name,
        params,
        get_rust_type(
            &*f.result,
            TypeContext {
                is_argument: false,
                is_const: false
            }
        )
    ));

    Some(result)
}

#[derive(Serialize)]
struct TemplateUserTypeEntry {
    name: String,
    value: String,
}

#[derive(Serialize)]
struct TemplateUserType {
    user_type: &'static str,
    base_type: String,
    entries: Vec<TemplateUserTypeEntry>,
    name: String,
    has_default: bool,
}

const TEMPLATE: &str = "// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
use std::ffi::c_void;
extern crate va_list;
use super::*;

{% for define in defines -%}
{{define}}
{% endfor -%}

{% for t in types -%}
{% if t.user_type == 'enum' -%}
{% for e in t.entries -%}
pub const {{e.name}}: {{t.base_type}} = {{e.value}};
{% endfor -%}
{% elif t.user_type == 'struct' -%}
{% if t.entries | length == 0 -%}
pub type {{t.name}} = c_void;
{% else %}
#[repr(C)]
#[derive(Clone, Copy{% if t.has_default -%}, Default{% else -%}{% endif -%})]
pub struct {{t.name}} {
{%for e in t.entries %}    pub {{e.name}}: {{e.value}},
{% endfor -%}
}
{% endif -%}
{% endif -%}
{% endfor -%}

{% if functions | length > 0 %}
#[link(name = \"{{link_name}}\", kind = \"{{link_kind}}\")]
extern \"C\" {
{% for f in functions %}{{f}}
{% endfor -%}
}
{% endif -%}
";

///
/// entry point
///
pub fn generate(f: &mut File, type_map: &TypeMap, export: &Export) -> Result<(), std::io::Error> {
    let mut w = BufWriter::new(f);

    let mut tera = Tera::default();
    tera.add_raw_template("template.rs", TEMPLATE).unwrap();
    // Using the tera Context struct
    let mut context = tera::Context::new();

    //
    // defines
    //
    let defines: Vec<String> = type_map
        .defines
        .iter()
        .filter_map(|def| {
            if def.path == export.header {
                if def.is_function {
                    c_macro::to_func(&def.tokens)
                } else {
                    c_macro::to_const(&def.tokens)
                }
            } else {
                None
            }
        })
        .collect();
    context.insert("defines", &defines);

    //
    // enum, struct, typedef
    //
    let types = get_sorted_entries(type_map, &export.header, |u| {
        if let Decl::Function(_) = &*u.get_decl() {
            false
        } else {
            true
        }
    });

    // rename anonymous struct / union
    let mut anonymous_count = 0;
    for t in &types {
        match &*t.get_decl() {
            Decl::Struct(_) => {
                if t.get_name().len() == 0 {
                    // anonymous
                    t.set_name(format!("anonymous_{}", anonymous_count));
                    anonymous_count += 1;
                }
            }
            _ => (),
        }
    }

    let types: Vec<TemplateUserType> = types
        .iter()
        .filter_map(|u| match &*u.get_decl() {
            Decl::Enum(e) => Some(TemplateUserType {
                user_type: "enum",
                name: u.get_name().clone(),
                has_default: false,
                base_type: get_rust_type(
                    &*e.base_type,
                    TypeContext {
                        is_argument: false,
                        is_const: false,
                    },
                )
                .to_string(),
                entries: e
                    .entries
                    .iter()
                    .map(|e| TemplateUserTypeEntry {
                        name: e.name.clone(),
                        value: enum_value(e.value),
                    })
                    .collect(),
            }),
            Decl::Struct(s) => {
                if s.fields.len() == 0 && s.definition.is_some() {
                    return None;
                }

                // if s.fields.len() == 0 {
                //     w.write_fmt(format_args!("pub type {} = c_void;\n", &t.get_name()))?;

                //     return Ok(());
                // }

                Some(TemplateUserType {
                    user_type: "struct",
                    name: u.get_name().clone(),
                    has_default: s.fields.iter().all(|f| has_default_type(&f.field_type)),
                    base_type: Default::default(),
                    entries: s
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| TemplateUserTypeEntry {
                            name: escape_symbol(&f.name, i).to_string(),
                            value: get_rust_type(
                                &*f.field_type,
                                TypeContext {
                                    is_argument: false,
                                    is_const: false,
                                },
                            )
                            .to_string(),
                        })
                        .collect(),
                })
            }
            _ => None,
        })
        .collect();
    context.insert("types", &types);

    //
    // functions
    //
    let link_name;
    let link_kind;
    if export.link.ends_with(".dll") {
        link_name = export.link.trim_end_matches(".dll");
        link_kind = "dylib";
    } else if export.link.ends_with(".lib") {
        link_name = export.link.trim_end_matches(".lib");
        link_kind = "static";
    } else {
        panic!();
    }
    context.insert("link_name", link_name);
    context.insert("link_kind", link_kind);

    let functions = get_sorted_entries(type_map, &export.header, |u| {
        if u.get_name().starts_with("operator ") {
            return false;
        }

        if let Decl::Function(_) = &*u.get_decl() {
            true
        } else {
            false
        }
    });

    let mut function_texts: Vec<String> = Vec::new();

    let mut used: HashSet<String> = HashSet::new();
    for t in functions {
        if let Decl::Function(f) = &*t.get_decl() {
            let mut name = t.get_name().to_owned();
            while used.contains(&name) {
                name.push('_');
            }
            if let Some(value) = write_function(&name, t, f) {
                function_texts.push(value);
            }
            used.insert(name);
        }
    }

    context.insert("functions", &function_texts);

    // render tera template
    let rendered = tera.render("template.rs", &context).unwrap();
    w.write(rendered.as_bytes())?;

    Ok(())
}
