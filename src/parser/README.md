## Nuktah BNF Reference

**translation-unit** -> decl-list<br>
**decl-list**        -> decl | decl • decl-list<br>
**decl**             -> var-decl | fn-decl

**var-decl**         -> type • T_IDENTIFIER • T_ASSIGN • expr-stmt<br>
**fn-decl**          -> T_FUNC • type • T_IDENTIFIER • T_PARENL • params • T_PARENR • block<br>
**type**             -> T_INT | T_STRING | T_FLOAT

**params**           -> param | param • T_COMMA • params | EPSILON<br>
**param**            -> type • T_IDENTIFIER

**block**            -> T_BRACEL • stmts • T_BRACER<br>
**stmts**            -> stmt • stmts | EPSILON<br>
**stmt**             -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt

**for-stmt**         -> T_FOR • T_PARENL • expr-stmt • expr-stmt • expr • T_PARENR • block<br>
**if-stmt**          -> T_IF • T_PARENL • expr • T_PARENR • block • T_ELSE • block<br>
**ret-stmt**         -> T_RET • expr • T_SEMICOLON

**expr-stmt**        -> expr • T_SEMICOLON<br>
**expr**             -> assign-expr<br>
**assign-expr**      -> bitwise-or-expr | assign-expr • T_ASSIGN • bitwise-or-expr

**bitwise-or-expr**  -> bitwise-and-expr | bitwise-or-expr • T_BITWISEOR • bitwise-and-expr<br>
**bitwise-and-expr** -> bool-expr | bitwise-and-expr • T_BITWISEAND • bool-expr

**bool-expr**        -> comp-expr | bool-expr • bool-op • comp-expr<br>
**bool-op**          -> T_BOOLEANAND | T_BOOLEANOR

**comp-expr**        -> shift-expr | comp-expr • comp-op • shift-expr<br>
**comp-op**          -> T_LESSTHAN | T_GREATERTHAN | T_EQUALSOP

**shift-expr**       -> add-expr | shift-expr • shift-op • add-expr<br>
**shift-op**         -> T_SHIFTLEFT | T_SHIFTRIGHT

**add-expr**         -> mul-expr | add-expr • add-op • mul-expr<br>
**add-op**           -> T_ADDOP | T_SUBOP

**mul-expr**         -> exp-expr | mul-expr • mul-op • exp-expr<br>
**mul-op**           -> T_MULOP | T_DIVOP | T_MODOP

**exp-expr**         -> unary-expr | exp-expr • T_EXPOP • unary-expr

**unary-expr**       -> primary | unary-op • unary-expr<br>
**unary-op**         -> T_SUBOP | T_BOOLEANOT | T_BITWISENOT

**primary**          -> T_INTLIT | T_FLOATLIT | T_STRINGLIT | T_IDENTIFIER | T_PARENL • expr • T_PARENR | fn-call

**fn-call**          -> T_IDENTIFIER • T_PARENL • args • T_PARENR<br>
**fn-args**          -> expr | expr • T_COMMA • fn-args | EPSILON