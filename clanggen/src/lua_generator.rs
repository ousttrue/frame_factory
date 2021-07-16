use tera::Tera;

use crate::{Decl, Export, Function, Primitives, Type, TypeMap, UserType};
use std::{
    borrow::Cow,
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

const TEMPLATE: &str = "-- this is generated.
local ffi = require 'ffi'

ffi.cdef[[
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
                let mut params = String::new();
                for p in &f.params {
                    std::fmt::Write::write_fmt(
                        &mut params,
                        format_args!("{},", get_lua_type(&*p.param_type, context.mutable())),
                    )
                    .unwrap();
                }
                format!(
                    "extern fn({}) -> {}",
                    params,
                    get_lua_type(&*f.result, context.mutable())
                )
                .into()
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
            let comma = if i==f.params.len()-1 {
                ""
            }
            else{
                ","
            };
            pw.write_fmt(format_args!(
                "        {} {}{}\n",
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
        "    {} {}({})",
        get_lua_type(
            &*f.result,
            TypeContext {
                is_argument: false,
                is_const: false
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

///
/// entry point
///
pub fn generate(f: &mut File, type_map: &TypeMap, export: &Export) -> Result<(), std::io::Error> {
    let mut w = BufWriter::new(f);
    let mut tera = Tera::default();
    tera.add_raw_template("template.rs", TEMPLATE).unwrap();
    // Using the tera Context struct
    let mut context = tera::Context::new();

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
