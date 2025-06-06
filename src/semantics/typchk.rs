use crate::{parser::ast, semantics::spaghetti::ScopeMap};

#[derive(Debug)]
pub enum TypeChkError {
    FunctionCallAndPrototypeMismatch,
    VariableLiteralValueAssignment,
    VariableExpressionAssignment,
    ExpectedBooleanExpression,
}

pub fn check_types(
    _ast_root: &ast::core::TranslationUnit,
    _sym_table: &ScopeMap,
) -> Result<(), TypeChkError> {
    Ok(())
}
