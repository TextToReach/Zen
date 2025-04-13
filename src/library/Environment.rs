#![allow(non_snake_case)]

use std::collections::HashMap;
use crate::library::{Methods::{Str, Throw}, Types::ZenError};
use super::Types::ZenType;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, ZenType>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: &str, value: ZenType) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn get(&self, name: &str) -> Option<ZenType> {
        match self.values.get(name) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
        }
    }

    pub fn set(&mut self, name: &str, value: ZenType) {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
        } else if let Some(parent) = self.parent.as_deref_mut() {
            parent.set(name, value);
        } else {
            Throw(format!("Değişken '{}' tanımlanmamış fakat üzerine değer atanmaya çalışılmış.", name), ZenError::NotDeclaredError, None, Some(true));
        }
    }
}
