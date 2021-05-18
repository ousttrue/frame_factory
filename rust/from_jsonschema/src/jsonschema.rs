extern crate itertools;
use core::panic;
use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
    rc::Rc,
};

use super::baseiterator::BaseIterator;
use itertools::Itertools;

#[derive(Debug)]
pub enum JsonSchemaError {
    FileNotFound,
    CanNotRead,
    ParseError,
}

pub enum JsonSchemaType {
    None,
    Boolean,
    Number,
    Integer,
    String,
    Array(Rc<JsonSchema>),
    Object(HashMap<String, Rc<JsonSchema>>),
    Dictionary(Rc<JsonSchema>),
}

fn concat(lhs: &str, rhs: &str) -> String {
    let mut lhs = lhs.to_owned();
    lhs.push_str(rhs);
    lhs
}

fn escape(src: &str) -> &str {
    if src == "type" {
        &"r#type"
    } else {
        src
    }
}

pub struct JsonSchema {
    pub path: String,
    pub title: String,
    pub description: String,
    pub base: Option<Rc<JsonSchema>>,
    pub js_type: JsonSchemaType,
}

impl JsonSchema {
    pub fn generate(&self, file: &mut BufWriter<fs::File>) {
        // 深さ優先で object 列挙する

        if self.title == "glTFProperty" {
            return;
        }

        match &self.js_type {
            JsonSchemaType::Object(props) => {
                self.generate_object(file, props).unwrap();
            }
            JsonSchemaType::Array(items) => {
                items.generate(file);
            }
            JsonSchemaType::None => {
                for base in BaseIterator::new(self) {
                    if let JsonSchemaType::Object(props) = &base.js_type {
                        self.generate_object(file, props).unwrap();
                        break;
                    }
                }
            }
            _ => (),
        }
    }

    fn get_prop(&self, key: &str) -> Option<Rc<JsonSchema>> {
        if let JsonSchemaType::Object(props) = &self.js_type {
            if let Some(prop) = props.get(key) {
                if let Some(base) = &self.base {
                    if let Some(from_base) = base.get_prop(key) {
                        return Some(from_base);
                    }
                }

                return Some(prop.clone());
            }
        }

        None
    }

    fn get_type(&self, key: &str, prop: &Rc<JsonSchema>) -> String {
        if key == "extras" || key == "extensions" {
            return "serd_json::Value".to_owned();
        }

        match &prop.js_type {
            JsonSchemaType::None => {
                if let Some(prop) = &prop.base {
                    return self.get_type(key, prop);
                };

                for base in BaseIterator::new(self) {
                    if let Some(prop) = base.get_prop(key) {
                        return base.get_type(key, &prop);
                    }
                }
                panic!();
            }
            JsonSchemaType::Boolean => "bool".to_owned(),
            JsonSchemaType::Integer => "i32".to_owned(),
            JsonSchemaType::Number => "f32".to_owned(),
            JsonSchemaType::String => "String".to_owned(),
            JsonSchemaType::Array(items) => format!("Vec<{}>", self.get_type(key, items)),
            JsonSchemaType::Object(_) => prop.title.replace(" ", ""),
            JsonSchemaType::Dictionary(additionalProperties) => format!(
                "HashMap<String, {}>",
                self.get_type(key, additionalProperties)
            ),
            _ => "## unknown ##".to_owned(),
        }
    }

    fn generate_object(
        &self,
        file: &mut BufWriter<fs::File>,
        props: &HashMap<String, Rc<JsonSchema>>,
    ) -> std::io::Result<()> {
        // recursive
        for (k, _) in props.iter() {
            if let Some(prop) = self.get_prop(k) {
                prop.generate(file);
            }
        }

        // write self
        file.write(format!("/// {} \n", self.get_title()).as_bytes())?;
        file.write(format!("/// {} \n", self.description).as_bytes())?;
        file.write(format!("struct {} {{\n", self.get_title().replace(" ", "")).as_bytes())?;
        for k in props.keys().sorted() {
            let v = props.get(k).unwrap();
            let t = self.get_type(k, v);
            let k = escape(k);
            file.write(format!("    {}: {},\n", k, t).as_bytes())?;
        }
        file.write(b"}\n")?;
        file.write(b"\n")?;

        Ok(())
    }

    fn bases(&self) -> BaseIterator {
        BaseIterator::new(self)
    }

    fn get_title(&self) -> String {
        if self.title.len() > 0 {
            return self.title.clone();
        };

        for base in self.bases() {
            if base.title.len() > 0 {
                return base.title.clone();
            }
        }

        panic!();
    }
}
