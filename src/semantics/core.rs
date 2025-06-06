use super::{errors, scope, spaghetti::SpaghettiStack, typchk};
use crate::parser::ast;

#[derive(Debug)]
pub enum SemanticError {
    ScopeError(errors::ScopeError),
    TypeChkError(errors::TypeChkError),
}

// TODO: Can we do this w/ a macro?

impl From<errors::ScopeError> for SemanticError {
    fn from(err: errors::ScopeError) -> SemanticError {
        SemanticError::ScopeError(err)
    }
}

impl From<errors::TypeChkError> for SemanticError {
    fn from(err: errors::TypeChkError) -> SemanticError {
        SemanticError::TypeChkError(err)
    }
}

pub fn analyse_semantics(
    ast_root: &ast::core::TranslationUnit,
) -> Result<SpaghettiStack, SemanticError> {
    let symbol_table = scope::analyse_scope(ast_root)?;
    typchk::check_types(ast_root, &symbol_table)?;
    Ok(symbol_table)
}
