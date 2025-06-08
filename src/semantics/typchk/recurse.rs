use crate::{
    parser::ast::core::*,
    semantics::{
        errors::TypeChkError,
        spaghetti::{Id, SpaghettiStack, SymType},
    },
};

/// Get the type of the expression `expr`, encountered in node (ScopeMap) w/ Id `node_id`
pub fn get_expr_type(
    _spaghet: &SpaghettiStack,
    _expr: &Expr,
    _node_id: Id,
) -> Result<SymType, TypeChkError> {
    // Recursive Descent:

    // if primaryexpr::ident, check if type matches that in the symbol table, starting from
    // node_id, (use utils::find_info_in_table !)

    // if primaryexpr::fncall

    Ok(SymType::Int)
}
