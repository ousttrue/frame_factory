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
    Skip,
    IO,
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
    pub fn get_prop(&self, key: &str) -> Option<Rc<JsonSchema>> {
        if let JsonSchemaType::Object(props) = &self.js_type {
            if let Some(prop) = props.get(key) {
                return Some(prop.clone());
            } else {
                return None;
            }
        } else {
            for base in self.bases() {
                if let JsonSchemaType::Object(props) = &base.js_type {
                    if let Some(prop) = props.get(key) {
                        return Some(prop.clone());
                    } else {
                        return None;
                    }
                }
            }
            panic!();
        }
    }

    pub fn get_type(&self, key: &str, prop: &Rc<JsonSchema>) -> String {
        if key == "extras" || key == "extensions" {
            return "serde_json::Value".to_owned();
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

    pub fn bases(&self) -> BaseIterator {
        BaseIterator::new(self)
    }

    pub fn get_title(&self) -> String {
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
