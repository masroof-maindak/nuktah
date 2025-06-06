use std::collections::HashMap;

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

pub struct ScopeMap {
    value: HashMap<String, SymType>,
    // parent: Option<&ScopeMap>,
    children: Vec<ScopeMap>,
}

impl ScopeMap {
    pub fn new() -> ScopeMap {
        ScopeMap {
            value: HashMap::new(),
            children: vec![],
        }
    }

    pub fn insert_val(&mut self, ident: &str, sym_type: SymType) {
        self.value.insert(ident.to_string(), sym_type);
    }

    pub fn insert_child(&mut self, new_child: ScopeMap) {
        self.children.push(new_child);
    }
}
