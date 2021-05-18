use std::{
    collections::HashMap,
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
}

impl Generator {
    pub fn new(path: &str) -> Generator {
        let mut file = BufWriter::new(fs::File::create(path).unwrap());

        file.write("use std::collections::HashMap;\n".as_bytes())
            .unwrap();
        file.write("\n".as_bytes()).unwrap();

        Generator { file }
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
        for (k, _) in props.iter() {
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

        // write js
        self.file.write(format!("/// {} \n", title).as_bytes())?;
        self.file
            .write(format!("/// {} \n", js.description).as_bytes())?;
        self.file
            .write(format!("struct {} {{\n", title.replace(" ", "")).as_bytes())?;
        for k in props.keys().sorted() {
            let v = props.get(k).unwrap();
            let t = js.get_type(k, v);
            let k = escape(k);
            self.file.write(format!("    {}: {},\n", k, t).as_bytes())?;
        }
        self.file.write(b"}\n")?;
        self.file.write(b"\n")?;

        Ok(())
    }
}
