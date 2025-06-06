#[derive(Debug)]
pub enum ScopeError {
    UndeclaredIdentifierUsed,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

#[derive(Debug)]
pub enum TypeChkError {
    FunctionCallAndPrototypeMismatch,
    VariableLiteralValueAssignment,
    VariableExpressionAssignment,
    ExpectedBooleanExpression,
}
