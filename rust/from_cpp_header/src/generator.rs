use std::{
    borrow::Cow,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Args, Decl, Enum, Function, Primitives, Struct, Type, TypeMap, UserType};

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

fn write_enum<W: Write>(w: &mut W, t: &UserType, e: &Enum) -> Result<(), std::io::Error>
{
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

    Ok(())
}

fn write_struct<W: Write>(w: &mut W, t: &UserType, s: &Struct)->Result<(), std::io::Error>
{
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

    Ok(())
}

fn write_function<W: Write>(w: &mut W, t: &UserType, f: &Function) -> Result<(), std::io::Error>
{
    if let Some(export_name) = &f.export_name {
        w.write_fmt(format_args!(
            "
#[link_name = \"{}\"]
pub fn {}();
",
            export_name, t.name
        ))?;
    }

    Ok(())
}

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), std::io::Error> {
    let dir = args.out.parent().unwrap();
    std::fs::create_dir_all(dir)?;

    let f = std::fs::File::create(&args.out)?;
    let mut w = BufWriter::new(f);

    let export = &args.exports[0];

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
        match &*t.decl.borrow() 
        {
            Decl::Enum(e) => write_enum(&mut w, t, e)?,
            Decl::Struct(s) => write_struct(&mut w, t, s)?,
            _ =>(),
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
            write_function(&mut w, t, f)?;
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ))?;

    Ok(())
}
