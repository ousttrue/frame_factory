extern crate tera;
extern crate serde;

use std::{
    io::stdout,
    io::{stderr, Write},
    path::Path,
};

mod translation_unit;
pub use translation_unit::*;

mod cx_source_location;
mod cx_string;
mod cx_token;

mod args;
pub use args::*;

mod visitor;
pub use visitor::*;

mod namespace;
pub use namespace::*;

mod types;
pub use types::*;
mod type_map;
pub use type_map::*;

mod type_function;
pub use type_function::*;

mod type_typedef;
pub use type_typedef::*;

mod type_enum;
pub use type_enum::*;

mod type_struct;
pub use type_struct::*;

mod define;
pub use define::*;

mod c_macro;
mod generator;

pub fn run(args: &[String]) -> Result<(), Error> {
    // args
    let args = Args::parse(args);
    if args.exports.len() == 0 {
        return Err(Error::StaticMessage("no export"));
    }

    // parse
    let tu = if args.exports.len() == 1 {
        TranslationUnit::parse(&args.exports[0].header, "", &args.compile_args)?
    } else {
        TranslationUnit::parse(
            Path::new("__unsaved_header__.h"),
            &args.merge_export_header(),
            &args.compile_args,
        )?
    };
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    // aggregate
    let mut type_map = TypeMap::new();
    let _root_namespace = visit_children_with(tu.get_cursor(), &mut type_map, || {
        NamespaceVisitor::nameless()
    });
    // root_namespace.debug_print("");

    // generate
    let mut mod_content = "#![allow(non_snake_case)]\n".to_owned();

    std::fs::create_dir_all(&args.out_dir).map_err(|e| Error::IOError(e))?;
    for export in &args.exports {
        let export_name = export.header.file_stem().unwrap().to_string_lossy();
        let path = format!("{}/{}.rs", args.out_dir.to_string_lossy(), export_name);
        let mut f = std::fs::File::create(&path).map_err(|e| Error::IOError(e))?;
        generator::generate(&mut f, &type_map, export).map_err(|e| Error::IOError(e))?;

        mod_content.push_str(&format!(
            "pub mod {};\npub use {}::*;\n",
            export_name, export_name
        ));
    }

    std::fs::write(
        format!("{}/mod.rs", args.out_dir.to_string_lossy()),
        mod_content,
    )
    .map_err(|e| Error::IOError(e))?;

    Ok(())
}
