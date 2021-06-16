use std::{
    borrow::Cow,
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
    rc::Rc,
};

use crate::{
    Decl, Enum, Export, Function, Primitives, Struct, Type, TypeMap, Typedef, UserType,
};

fn escape_symbol(src: &str) -> Cow<str> {
    match src {
        "type" | "in" | "ref" => format!("r#{}", src).into(),
        _ => src.into(),
    }
}

// typedef => typedef => pointer => typedef
fn is_function(t: &Type) -> bool {
    match &*t {
        Type::Pointer(p) => {
            return is_function(p);
        }
        Type::UserType(u) => match &*u.get_decl() {
            Decl::Function(_f) => {
                return true;
            }
            Decl::Typedef(d) => return is_function(&*d.base_type),
            _ => (),
        },
        _ => (),
    }

    false
}

fn rename_type(t: &str) -> String {
    match t {
        "size_t" => "usize".into(),
        "va_list" => "va_list::VaList".into(),
        //
        "Uint32" => "u32".into(),
        "Uint16" => "u16".into(),
        _ => t.into(),
    }
}

fn get_rust_type(t: &Type, is_const: bool) -> Cow<'static, str> {
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
            let pointee_type = get_rust_type(&pointee, false);
            format!(
                "*{} {}",
                if is_const { "const" } else { "mut" },
                pointee_type
            )
            .into()
        }
        Type::Array(element, size) => {
            let element_type = get_rust_type(&*element, false);
            format!("[{}; {}]", element_type, size).into()
        }
        Type::UserType(u) => match &*u.get_decl() {
            Decl::Enum(_) => u.get_name().to_owned().into(),
            Decl::Typedef(_) => rename_type(u.get_name().as_str()).into(),
            Decl::Struct(_) => u.get_name().to_owned().into(),
            // to function pointer
            Decl::Function(f) => {
                let mut params = String::new();
                for p in &f.params {
                    std::fmt::Write::write_fmt(
                        &mut params,
                        format_args!("{},", get_rust_type(&*p.param_type, false)),
                    )
                    .unwrap();
                }
                format!(
                    "extern fn({}) -> {}",
                    params,
                    get_rust_type(&*f.result, false)
                )
                .into()
            }
            Decl::None => rename_type(&u.get_name()).into(),
        },
        _ => format!("unknown").into(),
    }
}

fn get_sorted_entries<'a, F: Fn(&Decl) -> bool>(
    type_map: &'a TypeMap,
    path: &Path,
    f: F,
) -> Vec<&'a UserType> {
    let mut enums: Vec<&UserType> = type_map
        .map
        .iter()
        .filter_map(|(_k, v)| {
            if let Type::UserType(t) = &**v {
                if t.file == path {
                    if f(&*t.get_decl()) {
                        return Some(t);
                    }
                }
            }

            None
        })
        .collect();

    enums.sort_by(|a, b| a.line.cmp(&b.line));

    enums
}

fn is_i32(t: &Rc<Type>) -> bool {
    if let Type::Primitive(Primitives::I32) = &**t {
        true
    } else {
        false
    }
}

fn write_enum<W: Write>(w: &mut W, t: &UserType, e: &Enum) -> Result<(), std::io::Error> {
    assert!(is_i32(&e.base_type));

    w.write_fmt(format_args!(
        "
#[repr({})]
pub enum {} {{
",
        get_rust_type(&*e.base_type, false),
        t.get_name(),
    ))?;

    let mut used: HashSet<i64> = HashSet::new();
    for entry in &e.entries {
        let mut prefix = "";
        if used.contains(&entry.value) {
            prefix = "// ";
        } else {
            used.insert(entry.value);
        }

        let mut name = entry.name.as_str();
        if name.starts_with(t.get_name().as_str()) {
            name = name.trim_start_matches(t.get_name().as_str());
        }

        let value = if entry.value > 0 {
            format!("0x{:x}", entry.value as i32)
        } else {
            entry.value.to_string()
        };

        w.write_fmt(format_args!("    {}{} = {},\n", prefix, name, value))?;
    }

    w.write("}\n".as_bytes())?;

    Ok(())
}

fn write_struct<W: Write>(w: &mut W, t: &UserType, s: &Struct) -> Result<(), std::io::Error> {
    if s.fields.len() == 0 && s.definition.is_some() {
        return Ok(());
    }

    if s.fields.len() == 0 {
        w.write_fmt(format_args!("pub type {} = c_void;\n", &t.get_name()))?;

        return Ok(());
    }

    w.write_fmt(format_args!(
        "
#[repr(C)]
pub struct {} {{
",
        t.get_name()
    ))?;

    for field in &s.fields {
        w.write_fmt(format_args!(
            "    pub {}: {},\n",
            field.name,
            get_rust_type(&*field.field_type, false)
        ))?;
    }

    w.write("}\n".as_bytes())?;

    Ok(())
}

fn write_typedef<W: Write>(w: &mut W, t: &UserType, d: &Typedef) -> Result<(), std::io::Error> {
    if let Type::UserType(b) = &*d.base_type {
        if b.get_name().as_str() == t.get_name().as_str() {
            return Ok(());
        }
    }

    if is_function(&*d.base_type) {
        w.write_fmt(format_args!(
            "pub type {} = *mut c_void; // function pointer\n",
            t.get_name(),
        ))?;
    } else {
        w.write_fmt(format_args!(
            "pub type {} = {};\n",
            t.get_name(),
            get_rust_type(&*d.base_type, false)
        ))?;
    }

    Ok(())
}

fn write_function<W: Write>(
    w: &mut W,
    name: &str,
    _t: &UserType,
    f: &Function,
) -> Result<(), std::io::Error> {
    if let Some(export_name) = &f.export_name {
        let mut params = String::new();
        let pw = &mut params as &mut dyn std::fmt::Write;
        let mut comment = "\n".to_owned();
        let cw = &mut comment as &mut dyn std::fmt::Write;

        if f.params.len() > 0 {
            pw.write_str("\n").unwrap();

            for param in &f.params {
                pw.write_fmt(format_args!(
                    "        {}: {},\n",
                    escape_symbol(&param.name),
                    get_rust_type(&*param.param_type, param.is_const)
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
            }

            pw.write_str("    ").unwrap();
        }

        w.write_fmt(format_args!("{}", comment))?;
        w.write_fmt(format_args!(
            "    #[link_name = \"{}\"]
    pub fn {}({}) -> {};
",
            export_name,
            name,
            params,
            get_rust_type(&*f.result, false)
        ))?;
    };

    Ok(())
}

pub fn generate(f: &mut File, type_map: &TypeMap, export: &Export) -> Result<(), std::io::Error> {
    let mut w = BufWriter::new(f);
    w.write_fmt(format_args!(
        "// this is generated.
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]        
use std::ffi::c_void;
extern crate va_list;

"
    ))?;

    //
    // defines
    //
    for def in &type_map.defines {
        if def.path == export.header {
            w.write(crate::c_macro::to_const(&def.tokens).as_bytes())?;
        }
    }

    //
    // enum, struct, typedef
    //
    let types = get_sorted_entries(type_map, &export.header, |d| {
        if let Decl::Function(_) = d {
            false
        } else {
            true
        }
    });

    for t in types {
        match &*t.get_decl() {
            Decl::Enum(d) => write_enum(&mut w, t, d)?,
            Decl::Struct(d) => write_struct(&mut w, t, d)?,
            Decl::Typedef(d) => write_typedef(&mut w, t, d)?,
            _ => (),
        }
    }

    //
    // functions
    //
    let link_name;
    let kind;
    if export.link.ends_with(".dll") {
        link_name = export.link.trim_end_matches(".dll");
        kind = "dylib";
    } else if export.link.ends_with(".lib") {
        link_name = export.link.trim_end_matches(".lib");
        kind = "static";
    } else {
        panic!();
    }

    w.write_fmt(format_args!(
        "
#[link(name = \"{}\", kind = \"{}\")]
extern \"C\" {{
",
        link_name, kind
    ))?;

    let functions = get_sorted_entries(type_map, &export.header, |d| {
        if let Decl::Function(_) = d {
            true
        } else {
            false
        }
    });

    let mut used: HashSet<String> = HashSet::new();
    for t in functions {
        if let Decl::Function(f) = &*t.get_decl() {
            let mut name = t.get_name().to_owned();
            while used.contains(&name) {
                name.push('_');
            }
            write_function(&mut w, &name, t, f)?;
            used.insert(name);
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ))?;

    Ok(())
}
