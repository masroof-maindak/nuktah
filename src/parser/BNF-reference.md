## Nuktah BNF Reference

**translation-unit** -> decl-list<br>
**decl-list**        -> decl | decl • decl-list<br>
**decl**             -> var-decl | fn-decl

// no empty initialization<br>
**var-decl**         -> type • T\_IDENTIFIER • T\_ASSIGN • expr-stmt<br>

**fn-decl**          -> T\_FUNC • type • T\_IDENTIFIER • T\_PARENL • params • T\_PARENR • block • T\_DOT<br>
**type**             -> T\_INT | T\_STRING | T\_FLOAT

**params**           -> param | param • T\_COMMA • params | EPSILON<br>
**param**            -> type • T\_IDENTIFIER

**block**            -> T\_BRACEL • stmts • T\_BRACER<br>
**stmts**            -> stmt • stmts | EPSILON<br>
**stmt**             -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt

// no empty init/cond/inc<br>
**for-stmt**         -> T\_FOR • T\_PARENL • var-decl • expr-stmt • expr • T\_PARENR • block

// mandatory else<br>
**if-stmt**          -> T\_IF • T\_PARENL • expr • T\_PARENR • block • T\_ELSE • block<br>

**ret-stmt**         -> T\_RET • expr-stmt

**expr-stmt**        -> expr • T\_DOT<br>
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
*add-op**           -> T\_ADDOP | T\_SUBOP

**mul-expr**         -> exp-expr | mul-expr • mul-op • exp-expr<br>
**mul-op**           -> T\_MULOP | T\_DIVOP | T\_MODOP

**exp-expr**         -> unary-expr | unary-expr • T\_EXPOP • exp-expr

**unary-expr**       -> primary | unary-op • unary-expr<br>
**unary-op**         -> T\_SUBOP | T\_BOOLEANOT | T\_BITWISENOT

**primary**          -> T\_INTLIT | T\_FLOATLIT | T\_STRINGLIT | T\_IDENTIFIER | T\_PARENL • expr • T\_PARENR | fn-call

**fn-call**          -> T\_IDENTIFIER • T\_PARENL • fn-args • T\_PARENR<br>
**fn-args**          -> expr | expr • T\_COMMA • fn-args | EPSILON

## Acknowledgements

- The Dragon Book
- [C's grammar](https://cs.wmich.edu/~gupta/teaching/cs4850/sumII06/The%20syntax%20of%20C%20in%20Backus-Naur%20form.htm)
- [Simple but Powerful Pratt Parsing - Matklad](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html)
- [Parsing Expressions by Precedence Climbing - Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing)
- [Parsing Expressions by Recursive Descent - Theodore Norvell](https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm)
