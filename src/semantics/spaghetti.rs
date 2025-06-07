use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

// NOTE: Symbol 'information' usually comprises one of either two things
// 1. Type information -> simple; less traversal
// 2. Reference to AST node AKA weaving -> more information; compact
//
// As we don't have structs at the moment, the former will suffice. Although I think it's possible
// to make do w/ approach #1 in their presence too, but let's see.

#[derive(Debug)]
pub enum SymType {
    Int,
    Float,
    String,
}

#[derive(Debug)]
pub struct SymInfo {
    is_var: bool,
    sym_type: SymType,
}

impl SymInfo {
    pub fn new(is_var: bool, sym_type: SymType) -> SymInfo {
        SymInfo { is_var, sym_type }
    }
}

#[derive(Debug)]
pub enum ScopeType {
    Root,
    FnBlock,
    ForBlock,
    IfBlock,
}

pub type Id = usize;

#[derive(Debug)]
struct ScopeMap {
    scope_type: ScopeType,
    parent: Option<Id>,
    children: Vec<Id>,
    value: HashMap<String, SymInfo>,
}

impl ScopeMap {
    fn new(parent_id: Option<Id>, scope_type: ScopeType) -> ScopeMap {
        ScopeMap {
            scope_type,
            parent: parent_id,
            children: vec![],
            value: HashMap::new(),
        }
    }

    fn insert_val(&mut self, ident: &str, sym_info: SymInfo) {
        self.value.insert(ident.to_string(), sym_info);
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

    pub fn create_scope_map(&mut self, parent_id: Option<Id>, scope_type: ScopeType) -> Id {
        let id = self.descendants.len();
        self.descendants
            .insert(id, ScopeMap::new(parent_id, scope_type));
        id
    }

    pub fn insert_ident_in_node(&mut self, node_id: Id, ident: &str, sym_info: SymInfo) {
        self.descendants
            .get_mut(&node_id)
            .expect("id should point to valid ScopeMap")
            .insert_val(ident, sym_info);
    }

    pub fn add_child(&mut self, node_id: Id, child_id: Id) {
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

    pub fn does_ident_exist(&self, node_id: Id, ident: &str) -> bool {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .value
            .contains_key(ident)
    }
}

impl Default for SpaghettiStack {
    fn default() -> Self {
        Self::new()
    }
}
