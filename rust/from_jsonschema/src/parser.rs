use std::{collections::HashMap, fs};

use serde_json::Value;

use crate::jsonschema::{JsonSchemaError, JsonSchemaType};
use crate::jsonschema::JsonSchema;


pub struct JsonSchemaParser {}

impl JsonSchemaParser {
    pub fn new() -> JsonSchemaParser {
        JsonSchemaParser {}
    }

    fn parse_ref(&self, path: &str, object: &Value) -> Result<Box<JsonSchema>, JsonSchemaError> {
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
                    return self.parse_file(path.as_str());
                }
            }
        }

        Err(JsonSchemaError::ParseError)
    }

    pub fn parse(
        &self,
        path: &str,
        json: &serde_json::Value,
    ) -> Result<Box<JsonSchema>, JsonSchemaError> {
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
                Some(self.parse_ref(path, allOf.get(0).unwrap()).unwrap())
            } else {
                None
            }
        } else if let Some(ref_) = json.get("$ref") {
            Some(self.parse_ref(path, json).unwrap())
        } else {
            None
        };
        let json = json.as_object().unwrap();

        let mut js = if let Some(t) = json.get("type") {
            let js_type = match t.as_str().unwrap() {
                "object" => {
                    if let Some(additionalProperties) = json.get("additionalProperties") {
                        JsonSchemaType::Dictionary(self.parse(path, additionalProperties).unwrap())
                    } else {
                        let mut props: HashMap<String, Box<JsonSchema>> = HashMap::new();
                        if let Some(properties) = json.get("properties") {
                            for (prop_name, prop) in json["properties"].as_object().unwrap().iter()
                            {
                                let prop = self.parse(path, prop)?;
                                props.insert(prop_name.clone(), prop);
                            }
                        }
                        JsonSchemaType::Object(props)
                    }
                }
                "array" => {
                    if let Some(object) = json.get("items") {
                        if let Ok(item) = self.parse_ref(path, object) {
                            // ref
                            JsonSchemaType::Array(item)
                        } else {
                            // items
                            JsonSchemaType::Array(self.parse(path, object).unwrap())
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

    pub fn parse_file(&self, path: &str) -> Result<Box<JsonSchema>, JsonSchemaError> {
        if !std::path::Path::new(path).exists() {
            return Err(JsonSchemaError::FileNotFound);
        }

        let buf = fs::read_to_string(path).map_err(|_| JsonSchemaError::CanNotRead)?;

        let json: serde_json::Value =
            serde_json::from_str(&buf).map_err(|_| JsonSchemaError::ParseError)?;

        self.parse(path, &json)
    }
}
