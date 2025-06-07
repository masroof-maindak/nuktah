use super::{
    errors::{ScopeError, TypeChkError},
    scope,
    spaghetti::SpaghettiStack,
    typchk,
};
use crate::convert_across_err;
use crate::parser::ast;

// TODO: perform error conversion w/ a macro

#[derive(Debug)]
pub enum SemanticError {
    ScopeErr(ScopeError),
    TypeChkErr(TypeChkError),
}

convert_across_err!(ScopeError, SemanticError, ScopeErr);
convert_across_err!(TypeChkError, SemanticError, TypeChkErr);

pub fn analyse_semantics(
    ast_root: &ast::core::TranslationUnit,
) -> Result<SpaghettiStack, SemanticError> {
    let symbol_table = scope::core::analyse_scope(ast_root)?;
    typchk::core::check_types(ast_root, &symbol_table)?;
    Ok(symbol_table)
}
