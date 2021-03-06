use std::{collections::HashMap, fs, rc::Rc};

use serde_json::Value;

use crate::jsonschema::JsonSchema;
use crate::jsonschema::{JsonSchemaError, JsonSchemaType};

pub struct JsonSchemaParser {
    cache: HashMap<String, Rc<JsonSchema>>,
}

impl JsonSchemaParser {
    pub fn new() -> JsonSchemaParser {
        JsonSchemaParser {
            cache: HashMap::new(),
        }
    }

    fn parse_ref(&mut self, path: &str, object: &Value) -> Result<Rc<JsonSchema>, JsonSchemaError> {
        if let Some(cache) = self.cache.get(path) {
            return Ok(cache.clone());
        }

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
                    let cache = self.parse_file(path.as_str())?;
                    self.cache.insert(path, cache.clone());
                    return Ok(cache);
                }
            }
        }

        Err(JsonSchemaError::ParseError)
    }

    pub fn parse(
        &mut self,
        path: &str,
        json: &serde_json::Value,
    ) -> Result<Rc<JsonSchema>, JsonSchemaError> {
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
        let base: Option<Rc<JsonSchema>> = if let Some(all_of) = json.get("allOf") {
            if let Some(all_of) = all_of.as_array() {
                // inherit
                if all_of.len() != 1 {
                    panic!();
                }
                Some(self.parse_ref(path, all_of.get(0).unwrap()).unwrap())
            } else {
                None
            }
        } else if let Some(_) = json.get("$ref") {
            Some(self.parse_ref(path, json).unwrap())
        } else {
            None
        };
        let json = json.as_object().unwrap();

        let js = if let Some(t) = json.get("type") {
            let js_type = match t.as_str().unwrap() {
                "object" => {
                    if let Some(additional_properties) = json.get("additionalProperties") {
                        JsonSchemaType::Dictionary(self.parse(path, additional_properties).unwrap())
                    } else {
                        let mut props: HashMap<String, Rc<JsonSchema>> = HashMap::new();
                        if let Some(_) = json.get("properties") {
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

            Rc::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type,
            })
        } else if let Some(any_of) = json.get("anyOf") {
            let any_of = any_of.as_array().unwrap();
            let any_of_type = any_of[any_of.len() - 1]["type"].as_str().unwrap();
            match any_of_type {
                "integer" => Rc::new(JsonSchema {
                    path: String::from(path),
                    title,
                    description,
                    base,
                    js_type: JsonSchemaType::Integer,
                }),
                "string" => Rc::new(JsonSchema {
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
            Rc::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type: JsonSchemaType::None,
            })
        } else if json.len() == 1 && base.is_some() {
            Rc::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type: JsonSchemaType::None,
            })
        } else {
            Rc::new(JsonSchema {
                path: String::from(path),
                title,
                description,
                base,
                js_type: JsonSchemaType::None,
            })
        };

        Ok(js)
    }

    pub fn parse_file(&mut self, path: &str) -> Result<Rc<JsonSchema>, JsonSchemaError> {
        if !std::path::Path::new(path).exists() {
            return Err(JsonSchemaError::FileNotFound);
        }

        let buf = fs::read_to_string(path).map_err(|_| JsonSchemaError::CanNotRead)?;

        let json: serde_json::Value =
            serde_json::from_str(&buf).map_err(|_| JsonSchemaError::ParseError)?;

        self.parse(path, &json)
    }
}
