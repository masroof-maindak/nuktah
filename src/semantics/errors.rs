#[derive(Debug)]
pub enum ScopeError {
    UndefinedIdentifierUsed,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

#[derive(Debug)]
pub enum TypeChkError {
    ErroneousVarDecl,
    FunctionCallAndPrototypeMismatch,
    ErroneousReturnType,
    ExpectedBooleanExpression,
    ErroneousBreak,
    NonBooleanCondStmt,
}
