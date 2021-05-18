use std::rc::Rc;
use crate::jsonschema::JsonSchema;

pub struct BaseIterator {
    base: Option<Rc<JsonSchema>>,
}

impl BaseIterator {
    pub fn new(js: &JsonSchema) -> BaseIterator {
        if let Some(base) = &js.base {
            BaseIterator {
                base: Some(base.clone()),
            }
        } else {
            BaseIterator { base: None }
        }
    }
}

impl Iterator for BaseIterator {
    type Item = Rc<JsonSchema>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(base) = self.base.clone() {
            // update
            self.base = if let Some(next) = &base.base {
                Some(next.clone())
            }
            else{
                None
            };
            Some(base)
        } else {
            None
        }
    }
}
