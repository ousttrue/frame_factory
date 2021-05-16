extern crate serde_json;
use core::panic;
use serde_json::value::Value;
use std::{collections::HashMap, fs};

#[derive(Debug)]
enum JsonSchemaError {
    FileNotFound,
    CanNotRead,
    ParseError,
}

enum JsonSchemaType {
    None,
    Boolean,
    Number,
    Integer,
    String,
    Array(Box<JsonSchema>),
    Object(HashMap<String, Box<JsonSchema>>),
    Dictionary(Box<JsonSchema>),
}

fn concat(lhs: &str, rhs: &str) -> String {
    let mut lhs = lhs.to_owned();
    lhs.push_str(rhs);
    lhs
}

struct JsonSchema {
    path: String,
    title: String,
    description: String,
    base: Option<Box<JsonSchema>>,
    js_type: JsonSchemaType,
}

impl JsonSchema {
    fn print(&self, indent: &str) {
        if let Some(base) = &self.base {
            println!("{}{}: {}", indent, self.title, base.title);
        } else {
            println!("{}{}", indent, self.title);
        }

        match &self.js_type {
            JsonSchemaType::Array(item) => item.print(&concat(indent, "  ")),
            JsonSchemaType::Object(props) => {
                for (k, v) in props.iter() {
                    println!("{}{} =>", indent, k);
                    v.print(&concat(indent, "  "));
                }
            }
            _ => panic!(),
        }
    }
}

fn parse_ref(path: &str, object: &Value) -> Result<Box<JsonSchema>, JsonSchemaError> {
    if let Some(object) = object.as_object() {
        if object.len() == 1 {
            if let Some(ref_path) = object.get("$ref") {
                let path = format!(
                    "{}/{}",
                    std::path::Path::new(path)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    ref_path.as_str().unwrap()
                );
                return parse_file(path.as_str());
            }
        }
    }

    Err(JsonSchemaError::ParseError)
}

fn parse(path: &str, json: &serde_json::Value) -> Result<Box<JsonSchema>, JsonSchemaError> {
    let title = if let Some(title) = json.get("title") {
        title.as_str().unwrap()
    } else {
        ""
    }
    .to_owned();
    let description = if let Some(description) = json.get("description") {
        description.as_str().unwrap()
    } else {
        ""
    }
    .to_owned();
    let base: Option<Box<JsonSchema>> = if let Some(allOf) = json.get("allOf") {
        if let Some(allOf) = allOf.as_array() {
            // inherit
            if allOf.len() != 1 {
                panic!();
            }
            Some(parse_ref(path, allOf.get(0).unwrap()).unwrap())
        } else {
            None
        }
    } else if let Some(ref_) = json.get("$ref") {
        Some(parse_ref(path, json).unwrap())
    } else {
        None
    };
    let json = json.as_object().unwrap();

    let mut js = if let Some(t) = json.get("type") {
        let js_type = match t.as_str().unwrap() {
            "object" => {
                if let Some(additionalProperties) = json.get("additionalProperties") {
                    JsonSchemaType::Dictionary(parse(path, additionalProperties).unwrap())
                } else {
                    let mut props: HashMap<String, Box<JsonSchema>> = HashMap::new();
                    if let Some(properties) = json.get("properties") {
                        for (prop_name, prop) in json["properties"].as_object().unwrap().iter() {
                            let prop = parse(path, prop)?;
                            props.insert(prop_name.clone(), prop);
                        }
                    }
                    JsonSchemaType::Object(props)
                }
            }
            "array" => {
                if let Some(object) = json.get("items") {
                    if let Ok(item) = parse_ref(path, object) {
                        // ref
                        JsonSchemaType::Array(item)
                    } else {
                        // items
                        JsonSchemaType::Array(parse(path, object).unwrap())
                    }
                } else {
                    panic!();
                }
            }
            "boolean" => JsonSchemaType::Boolean,
            "number" => JsonSchemaType::Number,
            "integer" => JsonSchemaType::Integer,
            "string" => JsonSchemaType::String,
            _ => panic!(""),
        };

        Box::new(JsonSchema {
            path: String::from(path),
            title,
            description,
            base,
            js_type,
        })
    } else if let Some(anyOf) = json.get("anyOf") {
        let anyOf = anyOf.as_array().unwrap();
        let anyOf_type = anyOf[anyOf.len() - 1]["type"].as_str().unwrap();
        match anyOf_type {
            "integer" => Box::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type: JsonSchemaType::Integer,
            }),
            "string" => Box::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type: JsonSchemaType::String,
            }),
            _ => panic!(),
        }
    } else if json.len() == 0 {
        // empty. name, extensions, extras
        Box::new(JsonSchema {
            path: String::from(path),
            title,
            description,
            base,
            js_type: JsonSchemaType::None,
        })
    } else if json.len() == 1 && base.is_some() {
        Box::new(JsonSchema {
            path: String::from(path),
            title,
            description,
            base,
            js_type: JsonSchemaType::None,
        })
    } else {
        Box::new(JsonSchema {
            path: String::from(path),
            title,
            description,
            base,
            js_type: JsonSchemaType::None,
        })
    };

    // for (k, v) in json.as_object().unwrap().iter() {
    //     match k.as_str() {
    //         "$schema" => {}
    //         "title" => {
    //             js.title = String::from(v.as_str().unwrap());
    //         }
    //         "allOf" => {
    //         }
    //         "dependencies" => {
    //             // validation
    //         }
    //         "description" => {
    //             js.description = String::from(v.as_str().unwrap());
    //         }
    //         "required" => {
    //             // validation
    //         }
    //         "type" => {}
    //         "properties" => {}
    //         "items" => {}
    //         _ => {
    //             print!("unknown key: {}", k);
    //         }
    //     }
    // }

    Ok(js)
}

fn parse_file(path: &str) -> Result<Box<JsonSchema>, JsonSchemaError> {
    if !std::path::Path::new(path).exists() {
        return Err(JsonSchemaError::FileNotFound);
    }

    let buf = fs::read_to_string(path).map_err(|_| JsonSchemaError::CanNotRead)?;

    let json: serde_json::Value =
        serde_json::from_str(&buf).map_err(|_| JsonSchemaError::ParseError)?;

    parse(path, &json)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("usage: {} {{path_to_glTF.schema.json}}", args[0]);
        return;
    }

    match parse_file(&args[1]) {
        Ok(json_schema) => {
            json_schema.print("");
        }
        Err(err) => println!("{:?}", err),
    }
}
