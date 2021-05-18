extern crate itertools;
use std::{
    collections::HashMap,
    fs,
    io::{BufWriter, Write},
    rc::Rc,
    usize,
};

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

        match &self.js_type {
            JsonSchemaType::Object(props) => {
                self.generate_object(file, props).unwrap();
            }
            JsonSchemaType::Array(items) => {
                items.generate(file);
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
                let mut current = self;
                if let Some(prop) = &prop.base {
                    return self.get_type(key, prop);
                };
                loop {
                    if let Some(base) = &current.base {
                        if let Some(prop) = base.get_prop(key) {
                            return base.get_type(key, &prop);
                        }
                        current = base;
                    } else {
                        panic!();
                    }
                }
            }
            JsonSchemaType::Boolean => "bool".to_owned(),
            JsonSchemaType::Integer => "i32".to_owned(),
            JsonSchemaType::Number => "f32".to_owned(),
            JsonSchemaType::String => "String".to_owned(),
            JsonSchemaType::Array(items) => format!("Vec<{}>", self.get_type(key, items)),
            JsonSchemaType::Object(_) => prop.title.replace(" ", ""),
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
            let prop = self.get_prop(k).unwrap();
            prop.generate(file);
        }

        // write self
        file.write(format!("// {} \n", self.title).as_bytes())?;
        file.write(format!("struct {} {{\n", self.title.replace(" ", "")).as_bytes())?;
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

    // match &self.js_type {
    //     JsonSchemaType::Object(props) => {
    //         println!("{{");
    //         for (k, v) in props.iter() {
    //             print!("{}: ", k);
    //             let prop = self.get_prop(k);
    //             prop.print(&concat(indent, "  "));
    //             println!("");
    //         }
    //         print!("}}");
    //     }
    //     JsonSchemaType::Array(items) => {
    //         print!("[");
    //         items.print("");
    //         print!("]");
    //     }
    //     JsonSchemaType::Integer => {
    //         print!("integer")
    //     }
    //     JsonSchemaType::Number => {
    //         print!("number")
    //     }
    //     JsonSchemaType::String => {
    //         print!("string")
    //     }
    //     JsonSchemaType::None => {
    //         print!("none")
    //     }
    //     _ => print!("{}", self.title),
    // }
}
