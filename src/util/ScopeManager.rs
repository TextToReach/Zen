use std::{collections::{HashMap, HashSet}, fmt::{write, Display}};

use crate::{features::tokenizer::InstructionEnum, library::Types::Object, parsers::Parsers::Expression};

#[derive(Debug, Clone)]
pub enum ScopeAction {
    RootScope,
    Repeat(f64),
    WhileTrue,
    IfBlock { condition: Expression },
    ElifBlock { condition: Expression },
    ElseBlock
}

impl Display for ScopeAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
}

#[derive(Debug, Clone)]
pub struct ScopeManager {
    scopes: HashMap<usize, Scope>,
    next_id: usize,
}

impl ScopeManager {
    pub fn new() -> Self {
        let mut manager = Self {
            scopes: HashMap::new(),
            next_id: 0,
        };
        manager.create_scope(None, None); // root scope
        manager
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
        };

        if let Some(pid) = parent_id {
            if let Some(parent_scope) = self.scopes.get_mut(&pid) {
                parent_scope.children.insert(id);
            }
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

    pub fn define_var(&mut self, scope_id: usize, name: String, value: Object) {
        if let Some(scope) = self.scopes.get_mut(&scope_id) {
            scope.variables.insert(name, value);
        }
    }

    pub fn get_var_in_scope(&self, scope_id: usize, name: &str) -> Option<Object> {
        self.scopes
            .get(&scope_id)?
            .variables
            .get(name)
            .cloned()
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
