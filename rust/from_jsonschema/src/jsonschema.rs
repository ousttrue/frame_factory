use std::{collections::HashMap, fs, rc::Rc};

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

pub struct JsonSchema {
    pub path: String,
    pub title: String,
    pub description: String,
    pub base: Option<Rc<JsonSchema>>,
    pub js_type: JsonSchemaType,
}

impl JsonSchema {
    pub fn generate(&self, file: &mut fs::File) {
        // 深さ優先で object 列挙する

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

    fn get_prop(&self, key: &str) -> &Rc<JsonSchema> {
        if let JsonSchemaType::Object(props) = &self.js_type {
            if let Some(prop) = props.get(key) {
                if let JsonSchemaType::None = self.js_type {
                    if let Some(base) = &self.base {
                        return base.get_prop(key);
                    }
                } else {
                    return prop;
                }
            }
        }

        panic!();
    }
}
