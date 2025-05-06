#![allow(non_snake_case, dead_code)]

use super::Types::Object;
use crate::library::{Methods::Throw, Types::ZenError};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, Debug)]
pub struct Environment {
	pub values: HashMap<String, Object>,
	pub parent: Option<Rc<RefCell<Environment>>>,
}

#[derive(Clone, Debug)]
pub struct IsolatedEnvironment {
	pub values: HashMap<String, Object>,
}

impl Environment {
	pub fn new() -> Self {
		Environment {
			values: HashMap::new(),
			parent: None,
		}
	}

	pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
		Environment {
			values: HashMap::new(),
			parent: Some(parent),
		}
	}

	pub fn get(&self, name: &str) -> Option<Object> {
		match self.values.get(name) {
			Some(val) => Some(val.clone()),
			None => match &self.parent {
				Some(parent) => parent.borrow().get(name),
				None => None,
			},
		}
	}

	pub fn set(&mut self, name: &str, value: Object) {
		if self.values.contains_key(name) {
			self.values.insert(name.to_string(), value);
		} else if let Some(parent) = self.parent.as_ref() {
			parent.borrow_mut().set(name, value);
		} else {
			self.values.insert(name.to_string(), value);
		}
	}
}
