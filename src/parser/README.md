 ## Nuktah BNF

translation-unit -> decl-list
decl-list        -> decl | decl • decl-list
decl             -> var-decl | fn-decl

var-decl         -> type • T\_IDENTIFIER • T\_ASSIGN • expr-stmt
fn-decl          -> T\_FUNC • type • T\_IDENTIFIER • T\_PARENL • params • T\_PARENR • block
type             -> T\_INT | T\_STRING | T\_FLOAT

params           -> param | param • T\_COMMA • params | EPSILON
param            -> type • T\_IDENTIFIER

block            -> T\_BRACEL • stmts • T\_BRACER
stmts            -> stmt • stmts | EPSILON
stmt             -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt

for-stmt         -> T\_FOR • T\_PARENL • expr-stmt • expr-stmt • expr • T\_PARENR • block
if-stmt          -> T\_IF • T\_PARENL • expr • T\_PARENR • block • T\_ELSE • block
ret-stmt         -> T\_RET • expr • T\_SEMICOLON

expr-stmt        -> expr • T\_SEMICOLON
expr             -> assign-expr
assign-expr      -> bitwise-or-expr | assign-expr • T\_ASSIGN • bitwise-or-expr

bitwise-or-expr  -> bitwise-and-expr | bitwise-or-expr • T\_BITWISEOR • bitwise-and-expr
bitwise-and-expr -> bool-expr | bitwise-and-expr • T\_BITWISEAND • bool-expr

bool-expr        -> comp-expr | bool-expr • bool-op • comp-expr
bool-op          -> T\_BOOLEANAND | T\_BOOLEANOR

comp-expr        -> shift-expr | comp-expr • comp-op • shift-expr
comp-op          -> T\_LESSTHAN | T\_GREATERTHAN | T\_EQUALSOP

shift-expr       -> add-expr | shift-expr • shift-op • add-expr
shift-op         -> T\_SHIFTLEFT | T\_SHIFTRIGHT

add-expr         -> mul-expr | add-expr • add-op • mul-expr
add-op           -> T\_ADDOP | T\_SUBOP

mul-expr         -> exp-expr | mul-expr • mul-op • exp-expr
mul-op           -> T\_MULOP | T\_DIVOP | T\_MODOP

exp-expr         -> unary-expr | exp-expr • T\_EXPOP • unary-expr

unary-expr       -> primary | unary-op • unary-expr
unary-op         -> T\_SUBOP | T\_BOOLEANOT | T\_BITWISENOT

primary          -> T\_INTLIT | T\_FLOATLIT | T\_STRINGLIT | T\_IDENTIFIER | T\_PARENL • expr • T\_PARENR | fn-call

fn-call          -> T\_IDENTIFIER • T\_PARENL • args • T\_PARENR
fn-args          -> expr | expr • T\_COMMA • fn-args | EPSILON

