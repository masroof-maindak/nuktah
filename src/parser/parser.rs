use crate::lexer::token::Token;

//────────────┐
// Nuktah BNF |
//────────────┘
//
// comp-unit        -> decl-list
// decl-list        -> decl | decl • decl-list
// decl             -> var-decl | fn-decl
//
// var-decl         -> type • T_IDENTIFIER • T_ASSIGN • expr • T_SEMICOLON
// fn-decl          -> type • T_IDENTIFIER • T_PARENL • params • T_PARENR • block
// type             -> T_INT | T_STRING | T_FLOAT
//
// params           -> param | param • T_COMMA • params | EPSILON
// param            -> type • T_IDENTIFIER
//
// block            -> T_BRACEL • stmts • T_BRACER
// stmts            -> stmt • stmts | EPSILON
// stmt             -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt
//
// for-stmt         -> T_FOR • T_PARENL • expr-stmt • expr-stmt • expr • T_PARENR • block
// if-stmt          -> T_IF • T_PARENL • expr • T_PARENR • block • T_ELSE • block
// ret-stmt         -> T_RET • T_SEMICOLON | T_RET • expr • T_SEMICOLON
//
// expr-stmt        -> expr • T_SEMICOLON
// expr             -> assign-expr
// assign-expr      -> bitwise-or-expr | bitwise-or-expr • T_ASSIGN • assign-expr
//
// bitwise-or-expr  -> bitwise-xor-expr | bitwise-or-expr • T_BITWISEOR • bitwise-xor-expr
// bitwise-xor-expr -> bitwise-and-expr | bitwise-xor-expr • T_EXP_OP • bitwise-and-expr
// bitwise-and-expr -> bool-expr | bitwise-and-expr • T_BITWISEAND • bool-expr
//
// bool-expr        -> comp-expr | comp-expr • bool-op • bool-expr
// bool-op          -> T_BOOLEANAND | T_BOOLEANOR
//
// comp-expr        -> shift-expr | shift-expr • comp-op • comp-expr
// comp-op          -> T_LESSTHAN | T_GREATERTHAN | T_EQUALSOP
//
// shift-expr       -> add-expr | add-expr • shift-op • shift-expr
// shift-op         -> T_SHIFTLEFT | T_SHIFTRIGHT
//
// add-expr         -> mul-expr | mul-expr • add-op • add-expr
// add-op           -> T_ADDOP | T_SUBOP
//
// mul-expr         -> unary-expr | unary-expr • mul-op • mul-expr
// mul-op           -> T_MULOP | T_DIVOP | T_MODOP
//
// unary-expr       -> primary | unary-op • unary-expr
// unary-op         -> T_SUBOP | T_BOOLEANOT | T_BITWISENOT
//
// primary          -> T_INTLIT | T_FLOATLIT | T_STRINGLIT | T_IDENTIFIER | T_PARENL • expr • T_PARENR | fn-call
//
// fn-call          -> T_IDENTIFIER • T_PARENL • args • T_PARENR
// args             -> expr | expr • T_COMMA • args | EPSILON
//

pub fn parse_token_list(_tokens: Vec<Token>) {
    todo!()
}
