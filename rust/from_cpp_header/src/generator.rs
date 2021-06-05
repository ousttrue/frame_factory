use std::{
    io::{BufWriter, Write},
    rc::Rc,
};

use crate::{Args, Decl, Error, Primitives, Type, TypeMap, UserType};

fn repr_type(t: &Type) -> &'static str {
    match t {
        Type::Primitive(Primitives::I32) => "i32",
        _ => panic!(),
    }
}

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), Error> {
    let dir = args.out.parent().unwrap();
    std::fs::create_dir_all(dir).map_err(|e| Error::IOError(e))?;

    let f = std::fs::File::create(&args.out).map_err(|e| Error::IOError(e))?;
    let mut w = BufWriter::new(f);

    let export = &args.exports[0];

    // enums
    let mut enums: Vec<&UserType> = type_map
        .map
        .iter()
        .filter_map(|(k, v)| {
            if let Type::UserType(t) = &**v {
                if let Some(_) = args.find_export(&t.file) {
                    if let Decl::Enum(e) = &*t.decl.borrow() {
                        return Some(t);
                    }
                }
            }

            None
        })
        .collect();

    enums.sort_by(|a, b| a.line.cmp(&b.line));

    for t in enums {
        if let Decl::Enum(e) = &*t.decl.borrow() {
            w.write_fmt(format_args!(
                "#[repr({})]
enum {} {{
",
                repr_type(&*e.base_type), t.name,
            ));

            for entry in &e.entries {
                w.write_fmt(format_args!("    {} = 0x{:x},\n", entry.name, entry.value as i32));
            }

            w.write("}\n\n".as_bytes());
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
    ));

    let mut functions: Vec<&UserType> = type_map
        .map
        .iter()
        .filter_map(|(k, v)| {
            if let Type::UserType(t) = &**v {
                if t.file == export.header {
                    if let Decl::Function(f) = &*t.decl.borrow() {
                        return Some(t);
                    }
                }
            }

            None
        })
        .collect();

    functions.sort_by(|a, b| a.line.cmp(&b.line));

    for t in functions {
        if let Decl::Function(f) = &*t.decl.borrow() {
            if let Some(export_name) = &f.export_name {
                w.write_fmt(format_args!(
                    "
    #[link_name = \"{}\"]
    pub fn {}();
",
                    export_name, t.name
                ));
            }
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ));

    Ok(())
}
