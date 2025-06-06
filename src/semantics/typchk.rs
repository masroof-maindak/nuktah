use crate::{parser::ast, semantics::spaghetti::SpaghettiStack};

#[derive(Debug)]
pub enum TypeChkError {
    FunctionCallAndPrototypeMismatch,
    VariableLiteralValueAssignment,
    VariableExpressionAssignment,
    ExpectedBooleanExpression,
}

pub fn check_types(
    _ast_root: &ast::core::TranslationUnit,
    _symbol_table: &SpaghettiStack,
) -> Result<(), TypeChkError> {
    Ok(())
}
