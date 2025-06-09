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
    ExpressionTypeMismatch,
    ExpectedBooleanExpression,
    ErroneousBreak,
    NonBooleanCondStmt,
    EmptyExpression,
    AttemptedBoolOpToNonBools,
    AttemptedBitOpToNonNumeric,
    AttemptedShiftOnNonInt,
    AttemptedAddOpToNonNumeric,
    AttemptedExponentianOfNonNumeric
}
