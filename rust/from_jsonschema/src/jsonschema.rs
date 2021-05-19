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
