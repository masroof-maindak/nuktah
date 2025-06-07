#[derive(Debug)]
pub enum ScopeError {
    UndefinedIdentifierUsed,
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
