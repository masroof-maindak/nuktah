use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

// NOTE: Symbol 'information' usually comprises one of either two things
// 1. Type information -> simple; less traversal
// 2. Reference to AST node AKA weaving -> more information; compact
//
// As we don't have structs at the moment, the former will suffice.
#[derive(Debug)]
pub enum SymType {
    Int,
    Float,
    String,
}

// CHECK: should I store a bool `is_var` to remember whether an identifier is a variable or a function?

pub type Id = usize;

#[derive(Debug)]
struct ScopeMap {
    value: HashMap<String, SymType>,
    parent: Option<Id>,
    children: Vec<Id>,
}

impl ScopeMap {
    fn new(parent_id: Option<Id>) -> ScopeMap {
        ScopeMap {
            value: HashMap::new(),
            parent: parent_id,
            children: vec![],
        }
    }

    fn insert_val(&mut self, ident: &str, sym_type: SymType) {
        self.value.insert(ident.to_string(), sym_type);
    }

    fn insert_child(&mut self, child_id: Id) {
        self.children.push(child_id);
    }
}

#[derive(Debug)]
/// Goofy-ass 'spaghetti stack' implemented as a b-tree w/ IDs because I had skill issues storing
/// nodes directly within nodes via smart pointer fuckery. This approach works well in my particular
/// use-case because there is no chance of any disassociation b/w an Id and a reference to a node
/// (as we won't be removing any nodes at all)
pub struct SpaghettiStack {
    // root: Id,
    descendants: BTreeMap<Id, ScopeMap>,
}

impl SpaghettiStack {
    pub fn new() -> SpaghettiStack {
        SpaghettiStack {
            descendants: BTreeMap::new(),
        }
    }

    pub fn create_scope_map(&mut self, parent_id: Option<Id>) -> Id {
        let id = self.descendants.len();
        self.descendants.insert(id, ScopeMap::new(parent_id));
        id
    }

    pub fn insert_identifier_in_node(&mut self, node_id: Id, ident: &str, sym_type: SymType) {
        self.descendants
            .get_mut(&node_id)
            .expect("id should point to valid ScopeMap")
            .insert_val(ident, sym_type);
    }

    pub fn add_node_as_child_of(&mut self, node_id: Id, child_id: Id) {
        self.descendants
            .get_mut(&node_id)
            .expect("id should point to valid ScopeMap")
            .insert_child(child_id);
    }

    pub fn get_node_parent_id(&self, node_id: Id) -> Option<Id> {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .parent
    }

    pub fn does_identifier_exist(&self, node_id: Id, ident: &str) -> bool {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .value
            .get(ident)
            .is_some()
    }
}
