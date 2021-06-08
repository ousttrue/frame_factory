use std::{
    io::stdout,
    io::{stderr, Write},
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

mod generator;

pub fn run(args: &[String]) -> Result<(), Error> {
    // args
    let args = Args::parse(args);

    // parse
    let tu = TranslationUnit::parse(&args.exports[0].header, &args.compile_args)?;
    stderr().flush().unwrap();
    stdout().flush().unwrap();

    // aggregate
    let mut type_map = TypeMap::new();
    let root_namespace = visit_children_with(tu.get_cursor(), &mut type_map, || NamespaceVisitor::nameless());
    // root_namespace.debug_print("");

    // generate
    generator::generate(&type_map, &args).map_err(|e| Error::IOError(e))?;

    Ok(())
}
