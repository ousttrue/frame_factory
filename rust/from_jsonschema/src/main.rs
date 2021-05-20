mod baseiterator;
mod jsonschema;
mod parser;
mod generator;

extern crate serde_json;
use crate::parser::JsonSchemaParser;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!(
            "usage: {} {{path_to_glTF.schema.json}} {{dst_dir}}",
            args[0]
        );
        return;
    }

    let mut parser = JsonSchemaParser::new();
    match parser.parse_file(&args[1]) {
        Ok(json_schema) => {
            // remove dir if exis
            if fs::metadata(&args[2]).is_ok() {
                fs::remove_dir_all(&args[2]).unwrap();
            }

            // create dir
            fs::create_dir_all(&args[2]).unwrap();

            // open
            let out_path = format!("{}/mod.rs", args[2]);
            let mut generator = generator::Generator::new(&out_path);

            generator.generate(&json_schema);
        }
        Err(err) => println!("{:?}", err),
    }
}
