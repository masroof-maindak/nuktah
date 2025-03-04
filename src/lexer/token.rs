#[derive(Debug, PartialEq)]
pub enum Token {
    // for, while, if, else, elif, ret
    For,
    While,
    If,
    Else,
    ElseIf,
    Return,

    // int, str, char, float, bool, fn
    Int,
    String,
    Char, // CHECK: how to handle char literals?
    Float,
    Bool,
    Function,

    // (), {}, [], `, ", '
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Backtick,
    Quotes,
    Quote,

    //  , :, ;
    Whitespace,
    Colon,
    Semicolon,

    // main, foo, bar, baz, etc
    Identifier(String),

    // 33, `hello world!`, 5.1, TRUE, FALSE
    IntLit(i64),
    StringLit(String),
    FloatLit(f64),
    BooleanLit(bool),

    // =, +, -, *, /, %, ^, ++, --, ==
    AssignOp,
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    ModOp,
    ExpOp,
    IncrementOp,
    DecrementOp,
    EqualsOp,

    // ., ,, !, &, |, &&, ||, ~, <, >, <<, >>
    Dot,
    Comma,
    BooleanNot,
    BitwiseAnd,
    BitwiseOr,
    BooleanAnd,
    BooleanOr,
    BitwiseNot,
    LessThan,
    GreaterThan,
    ShiftLeft,
    ShiftRight,
}
