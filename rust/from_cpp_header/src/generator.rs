use std::{
    borrow::Cow,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Args, Decl, Primitives, Type, TypeMap, UserType};

fn get_rust_type(t: &Type) -> Cow<'static, str> {
    match t {
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
        Type::Pointer(_) => "*mut c_void".into(),
        Type::Array(element, size) => {
            let element_type = get_rust_type(&*element);
            format!("[{}; {}]", element_type, size).into()
        }
        Type::UserType(u) => match &*u.decl.borrow() {
            Decl::Enum(_) => u.name.clone().into(),
            Decl::Typedef(_) => u.name.clone().into(),
            Decl::Struct(_) => u.name.clone().into(),
            _ => format!("unknown {}", u.name).into(),
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
                    if f(&*t.decl.borrow()) {
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

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), std::io::Error> {
    let dir = args.out.parent().unwrap();
    std::fs::create_dir_all(dir)?;

    let f = std::fs::File::create(&args.out)?;
    let mut w = BufWriter::new(f);

    let export = &args.exports[0];

    //
    // enums
    //
    let enums = get_sorted_entries(type_map, &export.header, |d| {
        if let Decl::Enum(_) = d {
            true
        } else {
            false
        }
    });

    for t in enums {
        if let Decl::Enum(e) = &*t.decl.borrow() {
            w.write_fmt(format_args!(
                "#[repr({})]
enum {} {{
",
                get_rust_type(&*e.base_type),
                t.name,
            ))?;

            for entry in &e.entries {
                w.write_fmt(format_args!(
                    "    {} = 0x{:x},\n",
                    entry.name, entry.value as i32
                ))?;
            }

            w.write("}\n\n".as_bytes())?;
        }
    }

    //
    // struct
    //
    let structs = get_sorted_entries(type_map, &export.header, |d| {
        if let Decl::Struct(s) = d {
            if s.fields.len() > 0 {
                return true;
            }
        }

        false
    });

    for t in structs {
        if let Decl::Struct(s) = &*t.decl.borrow() {
            w.write_fmt(format_args!(
                "#[repr(C)]
pub struct {} {{
",
                t.name
            ))?;

            for field in &s.fields {
                w.write_fmt(format_args!(
                    "    {}: {},\n",
                    field.name,
                    get_rust_type(&*field.field_type)
                ))?;
            }

            w.write("}\n\n".as_bytes())?;
        }
    }

    //
    // functions
    //

    w.write_fmt(format_args!(
        "#[link(name = \"{}\", kind = \"dylib\")]
extern {{
",
        export.dll
    ))?;

    let functions = get_sorted_entries(type_map, &export.header, |d| {
        if let Decl::Function(_) = d {
            true
        } else {
            false
        }
    });

    for t in functions {
        if let Decl::Function(f) = &*t.decl.borrow() {
            if let Some(export_name) = &f.export_name {
                w.write_fmt(format_args!(
                    "
    #[link_name = \"{}\"]
    pub fn {}();
",
                    export_name, t.name
                ))?;
            }
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ))?;

    Ok(())
}
