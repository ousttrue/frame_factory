use std::{
    ascii::escape_default,
    borrow::Cow,
    io::{BufWriter, Write},
    path::Path,
};

use crate::{Args, Decl, Enum, Function, Primitives, Struct, Type, TypeMap, Typedef, UserType};

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
            Decl::Function(f) => {
                return true;
            }
            Decl::Typedef(d) => return is_function(&*d.base_type),
            _ => (),
        },
        _ => (),
    }

    false
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
            Decl::Enum(_) => u.name.clone().into(),
            Decl::Typedef(_) => u.name.clone().into(),
            Decl::Struct(_) => u.name.clone().into(),
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

fn write_enum<W: Write>(w: &mut W, t: &UserType, e: &Enum) -> Result<(), std::io::Error> {
    w.write_fmt(format_args!(
        "
#[repr({})]
enum {} {{
",
        get_rust_type(&*e.base_type, false),
        t.name,
    ))?;

    for entry in &e.entries {
        w.write_fmt(format_args!(
            "    {} = 0x{:x},\n",
            entry.name, entry.value as i32
        ))?;
    }

    w.write("}\n".as_bytes())?;

    Ok(())
}

fn write_struct<W: Write>(w: &mut W, t: &UserType, s: &Struct) -> Result<(), std::io::Error> {
    if s.fields.len() == 0 && s.definition.is_some() {
        return Ok(());
    }

    w.write_fmt(format_args!(
        "
#[repr(C)]
pub struct {} {{
",
        t.name
    ))?;

    for field in &s.fields {
        w.write_fmt(format_args!(
            "    {}: {},\n",
            field.name,
            get_rust_type(&*field.field_type, false)
        ))?;
    }

    w.write("}\n".as_bytes())?;

    Ok(())
}

fn write_typedef<W: Write>(w: &mut W, t: &UserType, d: &Typedef) -> Result<(), std::io::Error> {
    if is_function(&*d.base_type) {
        w.write_fmt(format_args!(
            "type {} = *mut c_void; // function pointer\n",
            t.name,
        ))?;
    } else {
        w.write_fmt(format_args!(
            "type {} = {};\n",
            t.name,
            get_rust_type(&*d.base_type, false)
        ))?;
    }

    Ok(())
}

fn write_function<W: Write>(w: &mut W, t: &UserType, f: &Function) -> Result<(), std::io::Error> {
    if let Some(export_name) = &f.export_name {
        let mut params = String::new();
        let pw = &mut params as &mut dyn std::fmt::Write;

        if f.params.len() > 0 {
            pw.write_str("\n").unwrap();

            for param in &f.params {
                if param.is_const {
                    let a = 0;
                }

                pw.write_fmt(format_args!(
                    "        {}: {},\n",
                    escape_symbol(&param.name),
                    get_rust_type(&*param.param_type, param.is_const)
                ))
                .unwrap();
            }

            pw.write_str("    ").unwrap();
        }

        w.write_fmt(format_args!(
            "
    #[link_name = \"{}\"]
    pub fn {}({}) -> {};
",
            export_name,
            t.name,
            params,
            get_rust_type(&*f.result, false)
        ))?;
    }

    Ok(())
}

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), std::io::Error> {
    let dir = args.out.parent().unwrap();
    std::fs::create_dir_all(dir)?;

    let f = std::fs::File::create(&args.out)?;
    let mut w = BufWriter::new(f);
    w.write_fmt(format_args!(
        "use std::ffi::c_void;

"
    ))?;

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

    w.write_fmt(format_args!(
        "
#[link(name = \"{}\", kind = \"dylib\")]
extern \"C\" {{
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
        if let Decl::Function(f) = &*t.get_decl() {
            write_function(&mut w, t, f)?;
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ))?;

    Ok(())
}
