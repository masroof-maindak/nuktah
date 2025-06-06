#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // duhrao, agar, warna, wapsi, dhancha, toro
    For,
    If,
    Else,
    Return,
    Struct,
    Break,

    // ginti, jumla, asharia, khali, fn
    Int,
    String,
    Float,
    Void,
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
    Newline,
    Colon,
    Semicolon,
    Comment,

    // main, foo, bar, baz, etc
    Identifier(String),

    // 33, `hello world!`, 5.1
    IntLit(i64),
    StringLit(String),
    FloatLit(f64),

    // =, +, -, *, /, %, ^, ==
    AssignOp,
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    ModOp,
    ExpOp,
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
