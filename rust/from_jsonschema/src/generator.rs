use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{BufWriter, Write},
    rc::Rc,
};

use itertools::Itertools;

use crate::{
    baseiterator::BaseIterator,
    jsonschema::{JsonSchema, JsonSchemaError, JsonSchemaType},
};

fn escape(src: &str) -> &str {
    if src == "type" {
        &"r#type"
    } else {
        src
    }
}

pub struct Generator {
    file: BufWriter<fs::File>,
    used: HashSet<String>,
}

impl Generator {
    pub fn new(path: &str) -> Generator {
        let file = BufWriter::new(fs::File::create(path).unwrap());

        let mut generator = Generator {
            file,
            used: HashSet::new(),
        };

        generator.writeln("use serde::{Deserialize, Serialize};
use std::collections::HashMap;

");

        generator
    }

    fn writeln(&mut self, src: &str)
    {
        self.file.write(src.as_bytes()).unwrap();
        self.file.write(b"\n").unwrap();
    }

    pub fn generate(&mut self, js: &Rc<JsonSchema>) {
        // 深さ優先で object 列挙する

        if js.title == "Accessor Sparse Indices" {
            let a = 0;
        };

        match &js.js_type {
            JsonSchemaType::Object(props) => {
                self.generate_object(js, props).unwrap();
            }
            JsonSchemaType::Array(items) => {
                self.generate(items);
            }
            JsonSchemaType::None => {
                for base in BaseIterator::new(js) {
                    if let JsonSchemaType::Object(props) = &base.js_type {
                        self.generate_object(js, props).unwrap();
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    fn generate_object(
        &mut self,
        js: &Rc<JsonSchema>,
        props: &HashMap<String, Rc<JsonSchema>>,
    ) -> Result<(), JsonSchemaError> {
        // recursive
        for k in props.keys().sorted() {
            if let Some(prop) = js.get_prop(k) {
                self.generate(&prop);
            }
        }

        self.write_object(js, props)
            .map_err(|_| JsonSchemaError::IO)?;

        Ok(())
    }

    fn write_object(
        &mut self,
        js: &Rc<JsonSchema>,
        props: &HashMap<String, Rc<JsonSchema>>,
    ) -> std::io::Result<()> {
        let title = js.get_title();
        if !self.used.contains(&title) {
            // write js
            self.writeln(format!("/// {}", title).as_str());
            self.writeln(format!("/// {}", js.description).as_str());
            self.writeln(format!("#[derive(Serialize, Deserialize, Debug)]").as_str());
            self.writeln(format!("struct {} {{", title.replace(" ", "")).as_str());
            for k in props.keys().sorted() {
                let v = props.get(k).unwrap();
                let t = js.get_type(k, v);
                let k = escape(k);
                self.file.write(format!("    {}: {},\n", k, t).as_bytes())?;
            }
            self.file.write(b"}\n")?;
            self.file.write(b"\n")?;

            self.used.insert(title);
        }

        Ok(())
    }
}
