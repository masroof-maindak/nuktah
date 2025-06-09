#[derive(Debug)]
pub enum ScopeError {
    UndeclaredVariableCalled,
    UndefinedFunctionCalled,
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
