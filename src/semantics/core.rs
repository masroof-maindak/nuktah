use super::{
    errors::{ScopeError, TypeChkError},
    scope,
    spaghetti::SpaghettiStack,
    typchk,
};
use crate::parser::ast;

// TODO: perform error conversion w/ a macro

#[derive(Debug)]
pub enum SemanticError {
    ScopeError(ScopeError),
    TypeChkError(TypeChkError),
}

impl From<ScopeError> for SemanticError {
    fn from(err: ScopeError) -> SemanticError {
        SemanticError::ScopeError(err)
    }
}

impl From<TypeChkError> for SemanticError {
    fn from(err: TypeChkError) -> SemanticError {
        SemanticError::TypeChkError(err)
    }
}

pub fn analyse_semantics(
    ast_root: &ast::core::TranslationUnit,
) -> Result<SpaghettiStack, SemanticError> {
    let symbol_table = scope::core::analyse_scope(ast_root)?;
    typchk::core::check_types(ast_root, &symbol_table)?;
    Ok(symbol_table)
}
