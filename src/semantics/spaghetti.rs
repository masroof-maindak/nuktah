use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;

// Symbol 'information' usually comprises one of either two things
// 1. Type information -> simple; less traversal
// 2. Reference to AST node AKA weaving -> more information; compact
//
// As we don't have structs at the moment, the former will suffice. Although I think it's possible
// to make do w/ approach #1 in their presence too, but let's see.

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SymType {
    Int,
    String,
    Float,
    Bool,
    Void,
}

#[derive(Debug, Clone)]
pub struct SymInfo {
    is_var: bool,
    sym_type: SymType,
}

impl SymInfo {
    pub fn new(is_var: bool, sym_type: SymType) -> SymInfo {
        SymInfo { is_var, sym_type }
    }

    pub fn is_var(&self) -> bool {
        self.is_var
    }

    pub fn get_type(&self) -> SymType {
        self.sym_type
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScopeType {
    Root,
    FnBlock,
    ForBlock,
    IfBlock,
}

pub type Id = usize;

#[derive(Debug, Clone)]
struct ChildInfo {
    id: Id,
    name: Option<String>, // e.g for associating a function's name with a ScopeMap, within the
                          // parent
}

impl ChildInfo {
    pub fn new(id: Id, name: Option<String>) -> ChildInfo {
        ChildInfo { id, name }
    }
}

#[derive(Debug)]
struct ScopeMap {
    scope_type: ScopeType,
    parent: Option<Id>,
    children: Vec<ChildInfo>,
    symbols: HashMap<String, SymInfo>,
    param_types: Vec<SymType>, // In case this a is the scope of a function
}

impl ScopeMap {
    fn new(parent_id: Option<Id>, scope_type: ScopeType) -> ScopeMap {
        ScopeMap {
            scope_type,
            parent: parent_id,
            children: vec![],
            symbols: HashMap::new(),
            param_types: vec![],
        }
    }

    fn insert_val(&mut self, ident: &str, sym_info: SymInfo, is_param: bool) {
        if is_param {
            self.param_types.push(sym_info.get_type());
        }
        self.symbols.insert(ident.to_string(), sym_info);
    }

    fn insert_child(&mut self, child_id: Id, scope_name: Option<String>) {
        self.children.push(ChildInfo::new(child_id, scope_name));
    }

    fn get_param_types(&self) -> &Vec<SymType> {
        &self.param_types
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

    pub fn insert_ident_in_node(
        &mut self,
        node_id: Id,
        ident: &str,
        sym_info: SymInfo,
        is_param: bool,
    ) {
        let scope_map = self
            .descendants
            .get_mut(&node_id)
            .expect("id should point to valid ScopeMap");

        if is_param {
            assert!(
                scope_map.scope_type == ScopeType::FnBlock,
                "attempted to insert parameter in non-function scope"
            )
        }

        scope_map.insert_val(ident, sym_info, is_param);
    }

    pub fn add_child(&mut self, node_id: Id, child_id: Id, child_scope_name: Option<String>) {
        self.descendants
            .get_mut(&node_id)
            .expect("id should point to valid ScopeMap")
            .insert_child(child_id, child_scope_name);
    }

    pub fn get_node_parent_id(&self, node_id: Id) -> Option<Id> {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .parent
    }

    pub fn get_ident_info(&self, node_id: Id, ident: &str) -> Option<&SymInfo> {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .symbols
            .get(ident)
    }

    /// In a provided scope, return the nth child scope that matches the given scope type. The
    /// caller should probably supplement this w/ a counter on their side.
    pub fn get_nth_child_of_type(
        &self,
        node_id: Id,
        n: usize,
        child_scope_type: ScopeType,
    ) -> Option<Id> {
        let mut ctr = 0;
        for child_info in self
            .descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .children
            .iter()
        {
            if child_scope_type == self.descendants.get(&child_info.id).unwrap().scope_type {
                ctr += 1;
            }

            if ctr == n {
                return Some(child_info.id);
            }
        }

        None
    }

    pub fn get_scope_type(&self, node_id: Id) -> ScopeType {
        self.descendants
            .get(&node_id)
            .expect("id should point to valid ScopeMap")
            .scope_type
            .clone()
    }

    pub fn get_fn_param_types(&self, ident: &str) -> &Vec<SymType> {
        let child_id = self
            .descendants
            .get(&0)
            .expect("id should point to valid ScopeMap")
            .children
            .iter()
            .find(|&info| info.name.as_ref().is_some_and(|name| name == ident))
            .expect("function being searched for doesn't exist")
            .id;

        self.descendants.get(&child_id).unwrap().get_param_types()
    }
}

impl Default for SpaghettiStack {
    fn default() -> Self {
        Self::new()
    }
}
