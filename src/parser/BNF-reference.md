<!-- vim: nospell -->
## Nuktah BNF Reference

**translation-unit** -> decl-list<br>
**decl-list**        -> decl | decl • decl-list<br>
**decl**             -> var-decl | fn-decl

**var-decl**         -> type • T\_IDENTIFIER • T\_ASSIGN • expr-stmt<br>

**fn-decl**          -> T\_FUNC • fn-type • T\_IDENTIFIER • T\_PAREN\_L • params • T\_PAREN\_R • block • T\_DOT<br>
**fn-type**          -> type | T\_VOID<br>
**type**             -> T\_INT | T\_STRING | T\_FLOAT | T\_BOOL

**params**           -> param | param • T\_COMMA • params | EPSILON<br>
**param**            -> type • T\_IDENTIFIER

**block**            -> T\_BRACE\_L • stmts • T\_BRACE\_R<br>
**stmts**            -> stmt • stmts | EPSILON<br>
**stmt**             -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt | break-stmt

**for-stmt**         -> T\_FOR • T\_PAREN\_L • init • cond • updt • T\_PAREN\_R • block<br>
**init**             -> var-decl | T\_DOT<br>
**cond**             -> expr-stmt<br>
**updt**             -> expr | EPSILON

**if-stmt**          -> T\_IF • T\_PAREN\_L • expr • T\_PAREN\_R • block • T\_ELSE • block // (mandatory else!)

**ret-stmt**         -> T\_RET • expr-stmt

**break-stmt**       -> T\_BREAK

**expr-stmt**        -> expr • T\_DOT | T\_DOT<br>
**expr**             -> assign-expr<br>
**assign-expr**      -> bool-expr | bool-expr • T\_ASSIGN • assign-expr

**bool-expr**        -> bitwise-or-expr | bool-expr • bool-op • bitwise-or-expr<br>
**bool-op**          -> T\_BOOLEANOR | T\_BOOLEANAND

**bitwise-or-expr**  -> bitwise-and-expr | bitwise-or-expr • T\_BITWISEOR • bitwise-and-expr

**bitwise-and-expr** -> comp-expr | bitwise-and-expr • T\_BITWISEAND • comp-expr

**comp-expr**        -> shift-expr | comp-expr • comp-op • shift-expr<br>
**comp-op**          -> T\_LESSTHAN | T\_GREATERTHAN | T\_EQUALSOP

**shift-expr**       -> add-expr | shift-expr • shift-op • add-expr<br>
**shift-op**         -> T\_SHIFTLEFT | T\_SHIFTRIGHT

**add-expr**         -> mul-expr | add-expr • add-op • mul-expr<br>
**add-op**           -> T\_ADDOP | T\_SUBOP

**mul-expr**         -> exp-expr | mul-expr • mul-op • exp-expr<br>
**mul-op**           -> T\_MULOP | T\_DIVOP | T\_MODOP

**exp-expr**         -> unary-expr | unary-expr • T\_EXPOP • exp-expr

**unary-expr**       -> primary | unary-op • unary-expr<br>
**unary-op**         -> T\_SUBOP | T\_BOOLEANOT | T\_BITWISENOT

**primary**          -> T\_INTLIT | T\_FLOATLIT | T\_STRINGLIT | bool-lit | T\_IDENTIFIER | T\_PAREN\_L • expr • T\_PAREN\_R | fn-call<br>
**bool-lit**         -> T\_TRUELIT |T\_FALSELIT

**fn-call**          -> T\_IDENTIFIER • T\_PAREN\_L • fn-args • T\_PAREN\_R<br>
**fn-args**          -> expr | expr • T\_COMMA • fn-args | EPSILON
