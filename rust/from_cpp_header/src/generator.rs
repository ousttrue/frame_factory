use std::io::{BufWriter, Write};

use crate::{Args, Decl, Error, Type, TypeMap};

pub fn generate(type_map: &TypeMap, args: &Args) -> Result<(), Error> {
    let dir = args.dst.parent().unwrap();
    std::fs::create_dir_all(dir).map_err(|e| Error::IOError(e))?;

    let f = std::fs::File::create(&args.dst).map_err(|e| Error::IOError(e))?;
    let mut w = BufWriter::new(f);

    for (_k, v) in type_map.map.iter() {
        if let Type::UserType(t) = &**v {
            if let Some(_) = args.find_export(&t.file) {
                if let Decl::Function(f) = &*t.decl.borrow() {
                    w.write_fmt(format_args!("// {:?}\n", v));
                }
            }
        }
    }

    Ok(())
}
