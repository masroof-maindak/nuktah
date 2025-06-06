use super::{scope, typchk};
use crate::{parser::ast, semantics::spaghetti::ScopeMap};

#[derive(Debug)]
pub enum SemanticError {
    ScopeError(scope::ScopeError),
    TypeChkError(typchk::TypeChkError),
}

// TODO: Can we do this w/ a macro?

impl From<scope::ScopeError> for SemanticError {
    fn from(err: scope::ScopeError) -> SemanticError {
        SemanticError::ScopeError(err)
    }
}

impl From<typchk::TypeChkError> for SemanticError {
    fn from(err: typchk::TypeChkError) -> SemanticError {
        SemanticError::TypeChkError(err)
    }
}

pub fn analyse_semantics(ast_root: &ast::core::TranslationUnit) -> Result<ScopeMap, SemanticError> {
    let sym_table = scope::analyse_scope(ast_root)?;
    typchk::check_types(ast_root, &sym_table)?;
    Ok(sym_table)
}
