use crate::{Decl, Export, Function, Primitives, Type, TypeMap, UserType};
use serde::Serialize;
use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    rc::Rc,
};
use tera::Tera;

const TEMPLATE: &str = "-- this is generated.
local ffi = require 'ffi'

ffi.cdef[[
{% for t in types -%}
{% if t.user_type == 'enum' -%}
enum {{t.name}} {
{% for e in t.entries %}    {{e.name}} = {{e.value}},
{% endfor -%}
};
{% elif t.user_type == 'struct' or t.user_type == 'union' -%}
{%- if t.entries | length == 0 -%}
typedef struct {{t.name}} {{t.name}};
{% else -%}
typedef {{t.user_type}} {{t.name}} {
{% for e in t.entries %}     {{e.field}};
{% endfor -%}
} {{t.name}};
{% endif -%}
{% endif -%}
{% endfor -%}
    
{% if functions | length > 0 -%}
{% for f in functions -%}{{f}}
{% endfor %}
{% endif %}
]]
";

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

fn escape_symbol(src: &str, i: usize) -> Cow<str> {
    src.into()
}

fn rename_type(t: &str) -> String {
    match t {
        "size_t" => "size_t".into(),
        "va_list" => "...".into(),
        //
        "Sint8" => "char".into(),
        "Sint16" => "short".into(),
        "Sint32" => "int".into(),
        "Sint64" => "int64_t".into(),
        "Uint8" => "uint8_t".into(),
        "Uint16" => "uint16_t".into(),
        "Uint32" => "uint32_t".into(),
        "Uint64" => "uint64_t".into(),
        //
        "HWND__" => "void".into(),
        "HDC__" => "void".into(),
        "HINSTANCE__" => "void".into(),
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

fn get_lua_type(t: &Type, context: TypeContext) -> Cow<'static, str> {
    match t {
        Type::Primitive(Primitives::Void) => "void".into(),
        Type::Primitive(Primitives::Bool) => "bool".into(),
        Type::Primitive(Primitives::I8) => "char".into(),
        Type::Primitive(Primitives::I16) => "short".into(),
        Type::Primitive(Primitives::I32) => "int".into(),
        Type::Primitive(Primitives::I64) => "int64_t".into(),
        Type::Primitive(Primitives::U8) => "uint8_t".into(),
        Type::Primitive(Primitives::U16) => "uint16_t".into(),
        Type::Primitive(Primitives::U32) => "uint32_t".into(),
        Type::Primitive(Primitives::U64) => "uint64_t".into(),
        Type::Primitive(Primitives::F32) => "float".into(),
        Type::Primitive(Primitives::F64) => "double".into(),
        Type::Pointer(pointee) => {
            let pointee_type = get_lua_type(&pointee, context.mutable());
            format!(
                "{}{}*",
                if context.is_const { "const " } else { "" },
                pointee_type
            )
            .into()
        }
        Type::Array(element, size) => {
            let element_type = get_lua_type(&*element, context.mutable());
            if context.is_argument {
                // not FFI-safe warning
                format!(
                    "{}{}*",
                    if context.is_const { "const " } else { "" },
                    element_type
                )
                .into()
            } else {
                format!("[{}; {}]", element_type, size).into()
            }
        }
        Type::UserType(u) => match &*u.get_decl() {
            Decl::Enum(e) => get_lua_type(&*e.base_type, context).into(),
            Decl::Typedef(d) => get_lua_type(&*d.base_type, context).into(),
            Decl::Struct(_) => u.get_name().to_owned().into(),
            // to function pointer
            Decl::Function(f) => {
                // let mut params = String::new();
                // for p in &f.params {
                //     std::fmt::Write::write_fmt(
                //         &mut params,
                //         format_args!("{},", get_lua_type(&*p.param_type, context.mutable())),
                //     )
                //     .unwrap();
                // }
                // format!(
                //     "extern fn({}) -> {}",
                //     params,
                //     get_lua_type(&*f.result, context.mutable())
                // )
                // .into()
                "void".into()
            }
            Decl::None => rename_type(&u.get_name()).into(),
        },
        _ => format!("unknown").into(),
    }
}

fn write_function(name: &str, t: &UserType, f: &Function) -> Option<String> {
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
            let comma = if i == f.params.len() - 1 { "" } else { "," };
            pw.write_fmt(format_args!(
                "    {} {}{}\n",
                get_lua_type(
                    &*param.param_type,
                    TypeContext {
                        is_argument: true,
                        is_const: param.is_const
                    }
                ),
                escape_symbol(&param.name, i),
                comma
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

    // let mut result = comment;
    let mut result = String::new();
    result.push_str(&format!(
        "{} {}({})",
        get_lua_type(
            &*f.result,
            TypeContext {
                is_argument: false,
                is_const: f.result_is_const
            }
        ),
        name,
        params,
    ));
    if export_name != name {
        result.push_str(&format!(" asm(\"{}\")", export_name));
    }
    result.push(';');

    Some(result)
}

#[derive(Serialize)]
struct TemplateUserTypeEntry {
    name: String,
    value: String,
    field: String,
}

#[derive(Serialize)]
struct TemplateUserType {
    user_type: &'static str,
    base_type: String,
    entries: Vec<TemplateUserTypeEntry>,
    name: String,
    has_default: bool,
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

fn get_field(name: &str, value: &str) -> String {
    if value.starts_with('[') {
        let mut cs = value.to_owned();
        cs.pop();
        cs.remove(0);

        let x: Vec<&str> = cs.split(';').collect();
        // let cs = cs.drain(1..cs.len()-2);

        // split [void*; 2]
        // let unquoted = value.chars().skip(1).collect();
        // let x = unquoted.split(';');
        format!("{} {}[{}]", x[0], name, x[1])
    } else {
        return format!("{} {}", value, name);
    }
}

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
                base_type: get_lua_type(
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
                        field: String::new(),
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
                    user_type: if s.is_union { "union" } else { "struct" },
                    name: u.get_name().clone(),
                    has_default: s.fields.iter().all(|f| has_default_type(&f.field_type)),
                    base_type: Default::default(),
                    entries: s
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(i, f)| {
                            let name = escape_symbol(&f.name, i).to_string();
                            let value = get_lua_type(
                                &*f.field_type,
                                TypeContext {
                                    is_argument: false,
                                    is_const: false,
                                },
                            )
                            .to_string();
                            let field = get_field(&name, &value);
                            let t = TemplateUserTypeEntry { name, value, field };
                            t
                        })
                        .collect(),
                })
            }
            _ => None,
        })
        .collect();
    context.insert("types", &types);

    ///
    /// functions
    ///
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

    context.insert("functions", &function_texts);

    // render tera template
    let rendered = tera.render("template.rs", &context).unwrap();
    w.write(rendered.as_bytes())?;

    Ok(())
}
