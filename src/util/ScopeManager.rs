use std::{
	collections::{HashMap, HashSet},
	fmt::{Display, write},
	thread::scope,
};

use crate::{features::tokenizer::InstructionEnum, library::Types::{Boolean, Object}, parsers::Parsers::Expression};

#[derive(Debug, Clone)]
pub enum ScopeAction {
	RootScope,
	Repeat(f64),
	WhileTrue,
	Condition(Expression),
}

impl Display for ScopeAction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum TransparentScopeOption {
	True { parent: usize },
	False,
}
use TransparentScopeOption::*;

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionStructure {
	pub scope_pointer: usize,	
	pub condition: Expression
}

impl ConditionStructure {
	pub fn empty() -> Self {
		Self {
			condition: Expression::falsy(),
			scope_pointer: 0
		}
	}

	pub fn is_empty(&self) -> bool {
		matches!(self.condition, Expression::Value(ref obj) if matches!(**obj, Object::Bool(Boolean { value: false })))
			&& self.scope_pointer == 0
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionBlock {
	pub If: ConditionStructure,
	pub Elif: Vec<ConditionStructure>,
	pub Else: ConditionStructure
}

impl ConditionBlock {
	pub fn empty() -> Self {
		Self {
			If: ConditionStructure::empty(),
			Elif: vec![],
			Else: ConditionStructure::empty(),
		}
	}

	pub fn clear(&mut self) {
		*self = Self::empty()
	}

	pub fn new(If: ConditionStructure) -> Self {
		Self {
			If,
			Elif: vec![],
			Else: ConditionStructure::empty(),
		}
		
	}

	pub fn push_elif(&mut self, Elif: ConditionStructure) {
		self.Elif.push(Elif);
	}
	
	pub fn push_elifs(&mut self, Elifs: Vec<ConditionStructure>) {
		for Elif in Elifs {
			self.Elif.push(Elif);
		}
	}

	pub fn push_else(&mut self, Else: ConditionStructure) {
		self.Else = Else
	}

	pub fn is_empty(&self) -> bool {
		self.If.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct Scope {
	pub id: usize,
	pub parent: Option<usize>,
	pub children: HashSet<usize>,
	pub action: Option<ScopeAction>,
	pub block: Vec<InstructionEnum>,
	pub variables: HashMap<String, Object>,
	pub is_transparent: TransparentScopeOption,
}

#[derive(Debug, Clone)]
pub struct ScopeManager {
	scopes: HashMap<usize, Scope>,
	next_id: usize,
}

impl ScopeManager {
	pub fn new() -> Self {
		Self {
			scopes: HashMap::new(),
			next_id: 0,
		}
	}

	pub fn create_scope(&mut self, parent_id: Option<usize>, action: Option<ScopeAction>) -> usize {
		let id = self.next_id;
		self.next_id += 1;

		let scope = Scope {
			id,
			action,
			parent: parent_id,
			children: HashSet::new(),
			block: Vec::new(),
			variables: HashMap::new(),
			is_transparent: False,
		};
		
		if let Some(pid) = parent_id {
			if let Some(parent_scope) = self.scopes.get_mut(&pid) {
				parent_scope.children.insert(id);
			}
		}
		
		self.scopes.insert(id, scope);
		id
	}
	
	/// Transparent scopes redirect variable declaration requests to the upper scope.
	pub fn create_transparent_scope(&mut self, parent_id: usize, action: Option<ScopeAction>) -> usize {
		let id = self.next_id;
		self.next_id += 1;
		
		let scope = Scope {
			id,
			action,
			parent: Some(parent_id),
			children: HashSet::new(),
			block: Vec::new(),
			variables: HashMap::new(),
			is_transparent: True { parent: parent_id },
		};

		if let Some(parent_scope) = self.scopes.get_mut(&parent_id) {
			parent_scope.children.insert(id);
		}

		self.scopes.insert(id, scope);
		id
	}

	pub fn push_code_to_scope(&mut self, id: usize, instr: &InstructionEnum) {
		if let Some(Scope) = self.get_scope_mut(id) {
			Scope.block.push(instr.clone());
		}
	}

	pub fn get_scope(&self, id: usize) -> Option<&Scope> {
		self.scopes.get(&id)
	}

	pub fn get_scope_mut(&mut self, id: usize) -> Option<&mut Scope> {
		self.scopes.get_mut(&id)
	}

	pub fn get_parent(&self, id: usize) -> Option<usize> {
		self.scopes.get(&id)?.parent
	}

	pub fn get_parent_of_parent(&self, id: usize) -> Option<usize> {
		let first_parent = self.scopes.get(&id)?.parent;
		self.scopes.get(&first_parent?)?.parent
	}

	pub fn get_children(&self, id: usize) -> Option<&HashSet<usize>> {
		Some(&self.scopes.get(&id)?.children)
	}

	pub fn get_children_tree(&self, id: usize) -> Vec<usize> {
		let mut result = Vec::new();
		if let Some(children) = self.get_children(id) {
			for &child_id in children {
				result.push(child_id);
				result.extend(self.get_children_tree(child_id));
			}
		}
		result
	}

	pub fn remove_scope(&mut self, id: usize) {
		if let Some(scope) = self.scopes.remove(&id) {
			if let Some(pid) = scope.parent {
				if let Some(parent) = self.scopes.get_mut(&pid) {
					parent.children.remove(&id);
				}
			}

			for child_id in &scope.children {
				self.remove_scope(*child_id);
			}
		}
	}

	pub fn set_var(&mut self, scope_id: usize, name: String, value: Object) {
		let mut current_id = scope_id;
		loop {
			let is_transparent = match self.get_scope(current_id) {
				Some(scope) => scope.is_transparent,
				None => break,
			};
			match is_transparent {
				True { parent } => {
					current_id = parent;
				}
				False => {
					if let Some(scope) = self.scopes.get_mut(&current_id) {
						scope.variables.insert(name, value);
					}
					break;
				}
			}
		}
	}

	pub fn get_var_in_scope<T: AsRef<str>>(&self, scope_id: usize, name: T) -> Option<Object> {
		let name = name.as_ref();
		self.scopes.get(&scope_id)?.variables.get(name).cloned()
	}
	
	pub fn does_var_exists<T: AsRef<str>>(&self, scope_id: usize, name: T) -> bool {
		let name = name.as_ref();
		if let Some(_) = self.get_var_in_scope(scope_id, name) {
			true
		} else {
			false
		}
	}
	/// Use this to retrieve variables.
	pub fn get_var<T: AsRef<str>>(&self, mut scope_id: usize, name: T) -> Option<Object> {
		let name = name.as_ref();
		loop {
			if let Some(value) = self.get_var_in_scope(scope_id, name) {
				return Some(value);
			}
			if let Some(parent_id) = self.get_parent(scope_id) {
				scope_id = parent_id;
			} else {
				break;
			}
		}
		None
	}

	pub fn reset_scope_vars(&mut self, scope_id: usize) {
        if let Some(scope) = self.get_scope_mut(scope_id) {
            scope.variables.clear();
        }
    }

	pub fn get_parent_scope(&self, id: usize) -> Option<&Scope> {
		let parent_id = self.get_parent(id)?;
		self.get_scope(parent_id)
	}

	pub fn get_depth(&self, mut id: usize) -> usize {
		let mut depth = 0;
		while let Some(parent_id) = self.get_parent(id) {
			depth += 1;
			id = parent_id;
		}
		depth
	}
}
