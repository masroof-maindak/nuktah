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
    FnCallParamCount,
    FnCallParamType,
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
    AttemptedExponentianOfNonNumeric,
    ReturnStmtNotFound,
}
