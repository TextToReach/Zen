use crate::util::ScopeManager::ScopeManager;

pub fn run_tests() {
	let mut manager = ScopeManager::new();
	let root_scope = manager.create_scope(Some(0), None);
	// manager.define_var(scope_id, name, value);

	println!("{:#?}", manager.get_scope(root_scope));
}