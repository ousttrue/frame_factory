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

        generator.writeln(
            "use serde::{Deserialize, Serialize};
use std::collections::HashMap;

",
        );

        generator
    }

    fn writeln(&mut self, src: &str) {
        self.file.write(src.as_bytes()).unwrap();
        self.file.write(b"\n").unwrap();
    }

    pub fn generate(&mut self, js: &Rc<JsonSchema>) {
        // 深さ優先で object 列挙する

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

    pub fn get_type(js: &Rc<JsonSchema>, key: &str, prop: &Rc<JsonSchema>) -> String {
        if key == "extras" || key == "extensions" {
            return "serde_json::Value".to_owned();
        }

        match &prop.js_type {
            JsonSchemaType::None => {
                for base in BaseIterator::new(js) {
                    if let Some(prop) = base.get_prop(key) {
                        return Self::get_type(&base, key, &prop);
                    }
                }

                if let Some(prop) = &prop.base {
                    return Self::get_type(js, key, prop);
                };
                panic!();
            }
            JsonSchemaType::Boolean => "bool".to_owned(),
            JsonSchemaType::Integer => "i32".to_owned(),
            JsonSchemaType::Number => "f32".to_owned(),
            JsonSchemaType::String => "String".to_owned(),
            JsonSchemaType::Array(items) => format!("Vec<{}>", Self::get_type(js, key, items)),
            JsonSchemaType::Object(_) => prop.title.replace(" ", ""),
            JsonSchemaType::Dictionary(additional_properties) => format!(
                "HashMap<String, {}>",
                Self::get_type(js, key, additional_properties)
            ),
        }
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
            self.writeln("#[derive(Serialize, Deserialize, Debug)]");
            self.writeln("#[allow(non_snake_case, non_camel_case_types)]");
            self.writeln(format!("pub struct {} {{", title.replace(" ", "")).as_str());
            for k in props.keys().sorted() {
                let v = props.get(k).unwrap();
                let mut t = Self::get_type(js, k, v);
                let k = escape(k);
                if t.starts_with("Vec<") || t.starts_with("HashMap<") {
                    self.writeln("    #[serde(default)]");
                    self.file
                        .write(format!("    pub {}: {},\n", k, t).as_bytes())?;
                } else {
                    t = format!("Option<{}>", t);
                    self.file
                        .write(format!("    pub {}: {},\n", k, t).as_bytes())?;
                }
            }
            self.writeln("}");
            self.writeln("");

            self.used.insert(title);
        }

        Ok(())
    }
}
