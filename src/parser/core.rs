use crate::lexer::Token;
use crate::parser::ast;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEOF,
    FailedToFindToken(Token),
    ExpectedTypeToken,
    ExpectedIdentifier,
    UnexpectedToken(Token),
    ExpectedFloatLit,
    ExpectedIntLit,
    ExpectedStringLit,
    ExpectedBoolLit,
    ExpectedExpr,
}

const PRIMITIVE_TYPES: [Token; 4] = [Token::Int, Token::String, Token::Float, Token::Bool];

pub fn parse_token_stream(tokens: &Vec<Token>) -> Result<ast::core::TranslationUnit, ParseError> {
    let mut p = Parser::new(tokens);
    p.parse_translation_unit()
}

struct Parser<'a> {
    pos: usize,
    token_stream: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            pos: 0,
            token_stream: tokens,
        }
    }

    // Returns the current token without consuming it.
    fn peek(&self) -> Option<&Token> {
        self.token_stream.get(self.pos)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.token_stream.get(self.pos + 1)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn accept(&mut self, expected: &Token) -> bool {
        if std::mem::discriminant(&self.token_stream[self.pos]) == std::mem::discriminant(expected)
        {
            self.advance();
            true
        } else {
            false
        }
    }

    // Consumes the token if it matches the expected token, otherwise returns an error.
    fn consume(&mut self, expected: Token) -> Result<(), ParseError> {
        if self.pos >= self.token_stream.len() {
            return Err(ParseError::UnexpectedEOF);
        }

        if !self.accept(&expected) {
            return Err(ParseError::FailedToFindToken(expected.clone()));
        }

        Ok(())
    }

    fn consume_prim_type_tok(&mut self) -> Result<ast::core::Type, ParseError> {
        if let Some(t) = self.peek().cloned() {
            if PRIMITIVE_TYPES.contains(&t) {
                self.advance();
                return Ok(t);
            }
        }

        Err(ParseError::ExpectedTypeToken)
    }

    fn consume_identifier(&mut self) -> Result<String, ParseError> {
        match self.token_stream.get(self.pos) {
            Some(Token::Identifier(x)) => {
                let name = x.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(ParseError::ExpectedIdentifier),
        }
    }

    fn consume_floatlit(&mut self) -> Result<f64, ParseError> {
        match self.token_stream.get(self.pos) {
            Some(Token::FloatLit(x)) => {
                let value = *x;
                self.advance();
                Ok(value)
            }
            _ => Err(ParseError::ExpectedFloatLit),
        }
    }

    fn consume_intlit(&mut self) -> Result<i64, ParseError> {
        match self.token_stream.get(self.pos) {
            Some(Token::IntLit(x)) => {
                let value = *x;
                self.advance();
                Ok(value)
            }
            _ => Err(ParseError::ExpectedIntLit),
        }
    }

    fn consume_stringlit(&mut self) -> Result<String, ParseError> {
        match self.token_stream.get(self.pos) {
            Some(Token::StringLit(x)) => {
                let value = x.clone();
                self.advance();
                Ok(value.clone())
            }
            _ => Err(ParseError::ExpectedStringLit),
        }
    }

    // translation-unit -> decl-list
    fn parse_translation_unit(&mut self) -> Result<ast::core::TranslationUnit, ParseError> {
        self.parse_decl_list()
    }

    // decl-list -> decl | decl • decl-list
    fn parse_decl_list(&mut self) -> Result<ast::core::DeclList, ParseError> {
        let mut root = Vec::new();

        while self.pos < self.token_stream.len() {
            let decl = self.parse_decl()?;
            root.push(decl);
        }

        Ok(root)
    }

    // decl -> var-decl | fn-decl
    fn parse_decl(&mut self) -> Result<ast::core::Decl, ParseError> {
        if let Some(Token::Function) = self.peek() {
            Ok(ast::core::Decl::Fn(self.parse_fn_decl()?))
        } else {
            Ok(ast::core::Decl::Var(self.parse_var_decl()?))
        }
    }

    // fn-decl -> T_FUNC • fn-type • T_IDENTIFIER • T_PAREN_L • params • T_PAREN_R • block • T_DOT
    fn parse_fn_decl(&mut self) -> Result<ast::core::FnDecl, ParseError> {
        self.consume(Token::Function)?;
        let type_token: Token;
        if let Some(Token::Void) = self.peek() {
            self.advance();
            type_token = Token::Void
        } else {
            type_token = self.consume_prim_type_tok()?;
        }
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let params = self.parse_params()?;
        self.consume(Token::ParenR)?;
        let block = self.parse_block()?;
        self.consume(Token::Dot)?;

        Ok(ast::core::FnDecl {
            type_tok: type_token,
            ident,
            params,
            block,
        })
    }

    // var-decl -> type • T_IDENTIFIER • T_ASSIGN • expr-stmt
    fn parse_var_decl(&mut self) -> Result<ast::core::VarDecl, ParseError> {
        let type_token = self.consume_prim_type_tok()?;
        let ident = self.consume_identifier()?;
        self.consume(Token::AssignOp)?;
        let expr_stmt = self.parse_expr_stmt()?;

        if expr_stmt.expr.is_none() {
            return Err(ParseError::ExpectedExpr);
        }

        Ok(ast::core::VarDecl {
            type_tok: type_token,
            ident,
            expr: expr_stmt.expr,
        })
    }

    // params -> param | param • T_COMMA • params | EPSILON
    fn parse_params(&mut self) -> Result<Vec<ast::core::Param>, ParseError> {
        let mut params: Vec<ast::core::Param> = Vec::new();

        if let Some(Token::ParenR) = self.peek() {
            return Ok(params);
        }

        params.push(self.parse_param()?);

        while let Some(Token::Comma) = self.peek() {
            self.consume(Token::Comma)?;
            params.push(self.parse_param()?);
        }

        Ok(params)
    }

    // param -> type • T_IDENTIFIER
    fn parse_param(&mut self) -> Result<ast::core::Param, ParseError> {
        let type_token = self.consume_prim_type_tok()?;
        let ident = self.consume_identifier()?;

        Ok(ast::core::Param {
            type_tok: type_token,
            ident,
        })
    }

    // block -> T_BRACE_L • stmts • T_BRACE_R
    fn parse_block(&mut self) -> Result<ast::core::Block, ParseError> {
        self.consume(Token::BraceL)?;
        let stmts = self.parse_stmts()?;
        self.consume(Token::BraceR)?;

        Ok(stmts)
    }

    // stmts -> stmt • stmts | EPSILON
    // stmt -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt
    fn parse_stmts(&mut self) -> Result<Vec<ast::core::Stmt>, ParseError> {
        let mut stmts: Vec<ast::core::Stmt> = Vec::new();

        while let Some(t) = self.peek() {
            match t {
                Token::For => stmts.push(ast::core::Stmt::For(self.parse_for_stmt()?)),
                Token::If => stmts.push(ast::core::Stmt::If(self.parse_if_stmt()?)),
                Token::Return => stmts.push(ast::core::Stmt::Ret(self.parse_ret_stmt()?)),
                t if PRIMITIVE_TYPES.contains(t) => {
                    stmts.push(ast::core::Stmt::VarDecl(self.parse_var_decl()?))
                }
                Token::Break => {
                    stmts.push(ast::core::Stmt::Break);
                    self.advance();
                }
                Token::BraceR => break, // end of encapsulating block...
                _ => stmts.push(ast::core::Stmt::Expr(self.parse_expr_stmt()?)),
            }
        }

        Ok(stmts)
    }

    // for-stmt -> T_FOR • T_PAREN_L • var-decl • expr-stmt • expr • T_PAREN_R • block
    fn parse_for_stmt(&mut self) -> Result<ast::core::ForStmt, ParseError> {
        self.consume(Token::For)?;
        self.consume(Token::ParenL)?;

        let init: Option<ast::core::VarDecl>;
        let updt: ast::core::Expr;

        if let Some(Token::Dot) = self.peek() {
            self.advance();
            init = None;
        } else {
            init = Some(self.parse_var_decl()?);
        }

        let cond = self.parse_expr_stmt()?;

        if let Some(Token::ParenR) = self.peek() {
            self.advance();
            updt = None;
        } else {
            updt = self.parse_expr()?;
            self.consume(Token::ParenR)?;
        }

        Ok(ast::core::ForStmt {
            init,
            cond,
            updt,
            block: self.parse_block()?,
        })
    }

    // if-stmt -> T_IF • T_PAREN_L • expr • T_PAREN_R • block • T_ELSE • block
    fn parse_if_stmt(&mut self) -> Result<ast::core::IfStmt, ParseError> {
        self.consume(Token::If)?;
        self.consume(Token::ParenL)?;
        let cond = self.parse_expr()?;
        self.consume(Token::ParenR)?;
        let if_block = self.parse_block()?;
        self.consume(Token::Else)?;
        let else_block = self.parse_block()?;

        Ok(ast::core::IfStmt {
            cond,
            if_block,
            else_block,
        })
    }

    // ret-stmt -> T_RET • expr • T_DOT
    fn parse_ret_stmt(&mut self) -> Result<ast::core::RetStmt, ParseError> {
        self.consume(Token::Return)?;
        self.parse_expr_stmt()
    }

    // expr-stmt -> expr • T_DOT | T_DOT
    fn parse_expr_stmt(&mut self) -> Result<ast::core::ExprStmt, ParseError> {
        if let Some(Token::Dot) = self.peek() {
            self.advance();
            return Ok(ast::core::ExprStmt { expr: None });
        }

        let expr = self.parse_expr()?;
        self.consume(Token::Dot)?;

        Ok(ast::core::ExprStmt { expr })
    }

    // expr -> assign-expr
    // Note that this function will always return some.
    // Empty expressions, where used, are detected via lookaheads.
    fn parse_expr(&mut self) -> Result<ast::core::Expr, ParseError> {
        let expr = self.parse_assign_expr()?;
        Ok(Some(expr))
    }

    // assign-expr -> bool-expr | bool-expr • T_ASSIGN • assign-expr
    fn parse_assign_expr(&mut self) -> Result<ast::core::AssignExpr, ParseError> {
        let left = self.parse_bool_expr()?;
        let ret: ast::core::AssignExpr;

        // NOTE: the idea is that the right hand side, by virtue of a recursive call, will
        // automatically resolve another assignment expression -- and this would naturally
        // comprise the right tree

        if let Some(Token::AssignOp) = self.peek() {
            self.advance();
            let right = self.parse_assign_expr()?;
            ret = ast::core::AssignExpr::Assign(left, Box::new(right));
        } else {
            ret = ast::core::AssignExpr::Bool(left);
        }

        Ok(ret)
    }

    // bool-expr -> bitwise-or-expr | bool-expr • bool_op • bitwise-or-expr
    // bool-op -> T_BOOLEANOR | T_BOOLEANAND
    fn parse_bool_expr(&mut self) -> Result<ast::core::BoolExpr, ParseError> {
        let mut left = ast::core::BoolExpr::BitOr(self.parse_bitwise_or_expr()?);

        while let Some(bool_op @ (Token::BooleanOr | Token::BooleanAnd)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_bitwise_or_expr()?;
            left = ast::core::BoolExpr::Bool(
                Box::new(left),
                bool_op, // && or ||
                right,
            )
        }

        Ok(left)
    }

    // bitwise-or-expr -> bitwise-and-expr | bitwise-or-expr • T_BITWISE_OR • bitwise-and-expr
    fn parse_bitwise_or_expr(&mut self) -> Result<ast::core::BitOrExpr, ParseError> {
        let mut left = ast::core::BitOrExpr::BitAnd(self.parse_bitwise_and_expr()?);

        while let Some(Token::BitwiseOr) = self.peek() {
            self.advance();
            let right = self.parse_bitwise_and_expr()?;
            left = ast::core::BitOrExpr::BitOr(Box::new(left), right)
        }

        Ok(left)
    }

    // bitwise-and-expr -> comp-expr | bitwise-and-expr • T_BITWISEAND • comp-expr
    fn parse_bitwise_and_expr(&mut self) -> Result<ast::core::BitAndExpr, ParseError> {
        let mut left = ast::core::BitAndExpr::Comp(self.parse_comp_expr()?);

        while let Some(Token::BitwiseAnd) = self.peek().cloned() {
            self.advance();
            let right = self.parse_comp_expr()?;
            left = ast::core::BitAndExpr::BitAnd(Box::new(left), right)
        }

        Ok(left)
    }

    // comp-expr -> shift-expr | comp-expr • comp_op • shift-expr
    // comp-op -> T_LESSTHAN, T_GREATERTHAN, T_EQUALSOP
    fn parse_comp_expr(&mut self) -> Result<ast::core::CompExpr, ParseError> {
        let mut left = ast::core::CompExpr::Shift(self.parse_shift_expr()?);

        while let Some(comp_op @ (Token::LessThan | Token::GreaterThan | Token::EqualsOp)) =
            self.peek().cloned()
        {
            self.advance();
            let right = self.parse_shift_expr()?;
            left = ast::core::CompExpr::Comp(
                Box::new(left),
                comp_op, // < or > or ==
                right,
            )
        }

        Ok(left)
    }

    // shift-expr -> add-expr | shift-expr • shift-op • add-expr
    // shift-op -> T_SHIFTLEFT | T_SHIFTRIGHT
    fn parse_shift_expr(&mut self) -> Result<ast::core::ShiftExpr, ParseError> {
        let mut left = ast::core::ShiftExpr::Add(self.parse_add_expr()?);

        while let Some(shift_op @ (Token::ShiftLeft | Token::ShiftRight)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_add_expr()?;
            left = ast::core::ShiftExpr::Shift(
                Box::new(left),
                shift_op, // << or >>
                right,
            )
        }

        Ok(left)
    }

    // add-expr -> mul-expr | add-expr • add-op • mul-expr
    // add-op -> T_ADDOP | T_SUBOP
    fn parse_add_expr(&mut self) -> Result<ast::core::AddExpr, ParseError> {
        let mut left = ast::core::AddExpr::Mul(self.parse_mul_expr()?);

        while let Some(add_op @ (Token::AddOp | Token::SubOp)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_mul_expr()?;
            left = ast::core::AddExpr::Add(
                Box::new(left),
                add_op, // + or -
                right,
            )
        }

        Ok(left)
    }

    // mul-expr -> exp-expr | mul-expr • mul-op • exp-expr
    // mul-op -> T_MULOP | T_DIVOP | T_MODOP
    fn parse_mul_expr(&mut self) -> Result<ast::core::MulExpr, ParseError> {
        let mut left = ast::core::MulExpr::Exp(self.parse_exp_expr()?);

        while let Some(mul_op @ (Token::MulOp | Token::DivOp | Token::ModOp)) = self.peek().cloned()
        {
            self.advance();
            let right = self.parse_exp_expr()?;
            left = ast::core::MulExpr::Mul(
                Box::new(left),
                mul_op, // * or / or %
                right,
            )
        }

        Ok(left)
    }

    // exp-expr -> unary-expr | unary-expr • T_EXPOP • exp-expr
    fn parse_exp_expr(&mut self) -> Result<ast::core::ExpExpr, ParseError> {
        let left = self.parse_unary_expr()?;
        let ret: ast::core::ExpExpr;

        if let Some(Token::ExpOp) = self.peek() {
            self.advance();
            let right = self.parse_exp_expr()?;
            ret = ast::core::ExpExpr::Exp(left, Box::new(right));
        } else {
            ret = ast::core::ExpExpr::Unary(left);
        }

        Ok(ret)
    }

    // unary-expr -> primary | unary-op • unary-expr
    // unary-op -> T_SUBOP | T_BOOLEANOT | T_BITWISENOT
    fn parse_unary_expr(&mut self) -> Result<ast::core::UnaryExpr, ParseError> {
        match self.peek().cloned() {
            Some(t @ (Token::SubOp | Token::BooleanNot | Token::BitwiseNot)) => {
                self.advance();
                let right = self.parse_unary_expr()?;
                Ok(ast::core::UnaryExpr::Unary(t, Box::new(right)))
            }
            _ => Ok(ast::core::UnaryExpr::Primary(self.parse_primary_expr()?)),
        }
    }

    // primary-expr -> T_IDENTIFIER | T_INTLIT | T_FLOATLIT | T_STRINGLIT | bool-lit | T_PAREN_L • expr • T_PAREN_R | fn-call
    // bool-lit -> T_TRUE | T_FALSE
    fn parse_primary_expr(&mut self) -> Result<ast::core::PrimaryExpr, ParseError> {
        if self.peek().is_none() {
            return Err(ParseError::UnexpectedEOF);
        }

        match self.peek().unwrap() {
            Token::Identifier(_) => {
                if let Some(Token::ParenL) = self.peek_next() {
                    return Ok(ast::core::PrimaryExpr::Call(self.parse_fn_call()?));
                }

                let ident = self.consume_identifier()?;
                Ok(ast::core::PrimaryExpr::Ident(ident))
            }

            Token::IntLit(_) => {
                let i_lit = self.consume_intlit()?;
                Ok(ast::core::PrimaryExpr::IntLit(i_lit))
            }

            Token::FloatLit(_) => {
                let f_lit = self.consume_floatlit()?;
                Ok(ast::core::PrimaryExpr::FloatLit(f_lit))
            }

            Token::True => {
                self.advance();
                Ok(ast::core::PrimaryExpr::BoolLit(true))
            }

            Token::False => {
                self.advance();
                Ok(ast::core::PrimaryExpr::BoolLit(false))
            }

            Token::Quotes => {
                self.consume(Token::Quotes)?;
                let str = self.consume_stringlit()?;
                self.consume(Token::Quotes)?;
                Ok(ast::core::PrimaryExpr::StringLit(str))
            }

            Token::ParenL => {
                self.consume(Token::ParenL)?;
                let expr = self.parse_expr()?;
                self.consume(Token::ParenR)?;

                Ok(ast::core::PrimaryExpr::Paren(Box::new(expr)))
            }

            _ => Err(ParseError::UnexpectedToken(self.peek().unwrap().clone())),
        }
    }

    // fn-call -> T_IDENTIFIER • T_PAREN_L • fn-args • T_PAREN_R
    fn parse_fn_call(&mut self) -> Result<ast::core::FnCall, ParseError> {
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let args = self.parse_fn_args()?;
        self.consume(Token::ParenR)?;

        Ok(ast::core::FnCall { ident, args })
    }

    // fn-args -> expr | expr • T_COMMA • fn-args | EPSILON
    fn parse_fn_args(&mut self) -> Result<ast::core::FnArgs, ParseError> {
        let mut params: Vec<ast::core::Expr> = Vec::new(); // Epsilon

        // Epsilon
        if let Some(Token::ParenR) = self.peek() {
            return Ok(params);
        }

        // expr
        params.push(self.parse_expr()?);

        // expr • T_COMMA • fn-args
        while let Some(Token::Comma) = self.peek() {
            self.consume(Token::Comma)?;
            params.push(self.parse_expr()?);
        }

        Ok(params)
    }
}
