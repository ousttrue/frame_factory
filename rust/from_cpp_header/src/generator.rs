use std::io::{BufWriter, Write};

use crate::{Args, Decl, Error, Type, TypeMap, UserType};

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), Error> {
    let dir = args.out.parent().unwrap();
    std::fs::create_dir_all(dir).map_err(|e| Error::IOError(e))?;

    let f = std::fs::File::create(&args.out).map_err(|e| Error::IOError(e))?;
    let mut w = BufWriter::new(f);

    let export = &args.exports[0];

    w.write_fmt(format_args!(
        "#[link(name=\"{}\", kind=\"dylib\")]
extern {{
",
        export.dll
    ));

    //
    // functions
    //

    let mut functions: Vec<&UserType> = type_map
        .map
        .iter()
        .filter_map(|(k, v)| {
            if let Type::UserType(t) = &**v {
                if let Some(_) = args.find_export(&t.file) {
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
        if t.file == export.header {
            if let Decl::Function(f) = &*t.decl.borrow() {
                if let Some(export_name) = &f.export_name {
                    w.write_fmt(format_args!(
                        "
    #[link_name=\"{}\"]
    pub fn {}();
",
                        export_name, t.name
                    ));
                }
            }
        }
    }

    w.write_fmt(format_args!(
        "}}
"
    ));

    Ok(())
}
