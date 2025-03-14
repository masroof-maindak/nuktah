use crate::lexer::token::Token;
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
}

pub fn parse_token_stream(tokens: &Vec<Token>) -> Result<ast::TranslationUnit, ParseError> {
    let mut p = Parser::new(tokens);
    Ok(p.parse_translation_unit()?)
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
        if self.accept(&expected) {
            Ok(())
        } else {
            Err(ParseError::FailedToFindToken(expected.clone()))
        }
    }

    fn consume_type_token(&mut self) -> Result<ast::Type, ParseError> {
        if let Some(t) = self.peek().cloned() {
            if [Token::Int, Token::String, Token::Float].contains(&t) {
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
                Ok(name.clone())
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
    fn parse_translation_unit(&mut self) -> Result<ast::TranslationUnit, ParseError> {
        self.parse_decl_list()
    }

    // decl-list -> decl | decl • decl-list
    fn parse_decl_list(&mut self) -> Result<ast::DeclList, ParseError> {
        let mut root = Vec::new();

        while self.pos < self.token_stream.len() {
            let decl = self.parse_decl()?;
            root.push(decl);
        }

        Ok(root)
    }

    // decl -> var-decl | fn-decl
    fn parse_decl(&mut self) -> Result<ast::Decl, ParseError> {
        if let Some(Token::Function) = self.peek() {
            Ok(ast::Decl::Fn(self.parse_fn_decl()?))
        } else {
            Ok(ast::Decl::Var(self.parse_var_decl()?))
        }
    }

    // fn-decl -> T_FUNC • type • T_IDENTIFIER • T_PARENL • params • T_PARENR • block
    fn parse_fn_decl(&mut self) -> Result<ast::FnDecl, ParseError> {
        self.consume(Token::Function)?;
        let type_token = self.consume_type_token()?;
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let params = self.parse_params()?;
        self.consume(Token::ParenR)?;
        let block = self.parse_block()?;

        Ok(ast::FnDecl {
            f: Token::Function,
            t: type_token,
            i: Token::Identifier(ident),
            pl: Token::ParenL,
            p: params,
            pr: Token::ParenR,
            b: block,
        })
    }

    // var-decl -> type • T_IDENTIFIER • T_ASSIGN • expr-stmt
    fn parse_var_decl(&mut self) -> Result<ast::VarDecl, ParseError> {
        let type_token = self.consume_type_token()?;
        let ident = self.consume_identifier()?;
        self.consume(Token::AssignOp)?;
        let expr_stmt = self.parse_expr_stmt()?;

        Ok(ast::VarDecl {
            t: type_token,
            i: Token::Identifier(ident),
            a: Token::AssignOp,
            e: expr_stmt,
        })
    }

    // params -> param | param • T_COMMA • params | EPSILON
    fn parse_params(&mut self) -> Result<ast::Params, ParseError> {
        let mut params: Vec<ast::Param> = Vec::new();

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
    fn parse_param(&mut self) -> Result<ast::Param, ParseError> {
        let type_token = self.consume_type_token()?;
        let ident = self.consume_identifier()?;

        Ok(ast::Param {
            t: type_token,
            i: Token::Identifier(ident),
        })
    }

    // block -> T_BRACE_L • stmts • T_BRACE_R
    fn parse_block(&mut self) -> Result<ast::Block, ParseError> {
        self.consume(Token::BraceL)?;
        let stmts = self.parse_stmts()?;
        self.consume(Token::BraceR)?;

        Ok(ast::Block {
            l: Token::BraceL,
            s: stmts,
            r: Token::BraceR,
        })
    }

    // stmts -> stmt • stmts | EPSILON
    // stmt -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt
    fn parse_stmts(&mut self) -> Result<ast::Stmts, ParseError> {
        let mut stmts: Vec<ast::Stmt> = Vec::new();

        while let Some(t) = self.peek() {
            match t {
                Token::For => stmts.push(ast::Stmt::For(self.parse_for_stmt()?)),
                Token::If => stmts.push(ast::Stmt::If(self.parse_if_stmt()?)),
                Token::Return => stmts.push(ast::Stmt::Ret(self.parse_ret_stmt()?)),
                Token::Int | Token::String | Token::Float => {
                    stmts.push(ast::Stmt::VarDecl(self.parse_var_decl()?))
                }
                Token::BraceR => break, // end of encapsulating block...
                _ => stmts.push(ast::Stmt::ExprStmt(self.parse_expr_stmt()?)),
            }
        }

        Ok(stmts)
    }

    // for-stmt -> T_FOR • T_PARENL • expr-stmt • expr-stmt • expr • T_PARENR • block
    fn parse_for_stmt(&mut self) -> Result<ast::ForStmt, ParseError> {
        self.consume(Token::For)?;
        self.consume(Token::ParenL)?;
        let init = self.parse_expr_stmt()?;
        let cond = self.parse_expr_stmt()?;
        let incr = self.parse_expr()?;
        self.consume(Token::ParenR)?;
        let block = self.parse_block()?;

        Ok(ast::ForStmt {
            f: Token::For,
            pl: Token::ParenL,
            init,
            cond,
            inc: incr,
            pr: Token::ParenR,
            b: block,
        })
    }

    // if-stmt -> T_IF • T_PARENL • expr • T_PARENR • block • T_ELSE • block
    fn parse_if_stmt(&mut self) -> Result<ast::IfStmt, ParseError> {
        self.consume(Token::If)?;
        self.consume(Token::ParenL)?;
        let cond = self.parse_expr()?;
        self.consume(Token::ParenR)?;
        let if_block = self.parse_block()?;
        self.consume(Token::Else)?;
        let else_block = self.parse_block()?;

        Ok(ast::IfStmt {
            i: Token::If,
            pl: Token::ParenL,
            e: cond,
            pr: Token::ParenR,
            ib: if_block,
            el: Token::Else,
            eb: else_block,
        })
    }

    // ret-stmt -> T_RET • expr • T_DOT
    fn parse_ret_stmt(&mut self) -> Result<ast::RetStmt, ParseError> {
        self.consume(Token::Return)?;
        let expr_stmt = self.parse_expr_stmt()?;

        Ok(ast::RetStmt {
            r: Token::Return,
            e: expr_stmt,
        })
    }

    // expr-stmt -> expr • T_Dot
    fn parse_expr_stmt(&mut self) -> Result<ast::ExprStmt, ParseError> {
        let expr = self.parse_expr()?;
        self.consume(Token::Dot)?;

        Ok(ast::ExprStmt {
            e: expr,
            s: Token::Dot,
        })
    }

    // expr -> assign-expr
    fn parse_expr(&mut self) -> Result<ast::Expr, ParseError> {
        Ok(self.parse_assign_expr()?)
    }

    // assign-expr -> bool-expr | bool-expr • T_ASSIGN • assign-expr
    fn parse_assign_expr(&mut self) -> Result<ast::AssignExpr, ParseError> {
        let left = self.parse_bool_expr()?;
        let ret: ast::AssignExpr;

        // NOTE: the idea is that the right hand side, by virtue of a recursive call, will
        // automatically resolve another assignment expression -- and this would naturally
        // comprise the right tree

        if let Some(Token::AssignOp) = self.peek() {
            self.advance();
            let right = self.parse_assign_expr()?;
            ret = ast::AssignExpr::Assign(left, Token::AssignOp, Box::new(right));
        } else {
            ret = ast::AssignExpr::Bool(left);
        }

        Ok(ret)
    }

    // bool-expr -> bitwise-or-expr | bool-expr • bool_op • bitwise-or-expr
    // bool-op -> T_BOOLEANOR | T_BOOLEANAND
    fn parse_bool_expr(&mut self) -> Result<ast::BoolExpr, ParseError> {
        let mut left = ast::BoolExpr::BitwiseOr(self.parse_bitwise_or_expr()?);

        while let Some(bool_op @ (Token::BooleanOr | Token::BooleanAnd)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_bitwise_or_expr()?;
            left = ast::BoolExpr::Bool(
                Box::new(left),
                bool_op, // && or ||
                right,
            )
        }

        Ok(left)
    }

    // bitwise-or-expr -> bitwise-and-expr | bitwise-or-expr • T_BITWISE_OR • bitwise-and-expr
    fn parse_bitwise_or_expr(&mut self) -> Result<ast::BitwiseOrExpr, ParseError> {
        let mut left = ast::BitwiseOrExpr::BitwiseAnd(self.parse_bitwise_and_expr()?);

        while let Some(Token::BitwiseOr) = self.peek().cloned() {
            self.advance();
            let right = self.parse_bitwise_and_expr()?;
            left = ast::BitwiseOrExpr::BitwiseOr(Box::new(left), Token::BitwiseOr, right)
        }

        Ok(left)
    }

    // bitwise-and-expr -> comp-expr | bitwise-and-expr • T_BITWISEAND • comp-expr
    fn parse_bitwise_and_expr(&mut self) -> Result<ast::BitwiseAndExpr, ParseError> {
        let mut left = ast::BitwiseAndExpr::Comp(self.parse_comp_expr()?);

        while let Some(Token::BitwiseAnd) = self.peek().cloned() {
            self.advance();
            let right = self.parse_comp_expr()?;
            left = ast::BitwiseAndExpr::BitwiseAnd(Box::new(left), Token::BitwiseAnd, right)
        }

        Ok(left)
    }

    // comp-expr -> shift-expr | comp-expr • comp_op • shift-expr
    // comp-op -> T_LESSTHAN, T_GREATERTHAN, T_EQUALSOP
    fn parse_comp_expr(&mut self) -> Result<ast::CompExpr, ParseError> {
        let mut left = ast::CompExpr::Shift(self.parse_shift_expr()?);

        while let Some(comp_op @ (Token::LessThan | Token::GreaterThan | Token::EqualsOp)) =
            self.peek().cloned()
        {
            self.advance();
            let right = self.parse_shift_expr()?;
            left = ast::CompExpr::Comp(
                Box::new(left),
                comp_op, // < or > or ==
                right,
            )
        }

        Ok(left)
    }

    // shift-expr -> add-expr | shift-expr • shift-op • add-expr
    // shift-op -> T_SHIFTLEFT | T_SHIFTRIGHT
    fn parse_shift_expr(&mut self) -> Result<ast::ShiftExpr, ParseError> {
        let mut left = ast::ShiftExpr::Add(self.parse_add_expr()?);

        while let Some(shift_op @ (Token::ShiftLeft | Token::ShiftRight)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_add_expr()?;
            left = ast::ShiftExpr::Shift(
                Box::new(left),
                shift_op, // << or >>
                right,
            )
        }

        Ok(left)
    }

    // add-expr -> mul-expr | add-expr • add-op • mul-expr
    // add-op -> T_ADDOP | T_SUBOP
    fn parse_add_expr(&mut self) -> Result<ast::AddExpr, ParseError> {
        let mut left = ast::AddExpr::Mul(self.parse_mul_expr()?);

        while let Some(add_op @ (Token::AddOp | Token::SubOp)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_mul_expr()?;
            left = ast::AddExpr::Add(
                Box::new(left),
                add_op, // + or -
                right,
            )
        }

        Ok(left)
    }

    // mul-expr -> exp-expr | mul-expr • mul-op • exp-expr
    // mul-op -> T_MULOP | T_DIVOP | T_MODOP
    fn parse_mul_expr(&mut self) -> Result<ast::MulExpr, ParseError> {
        let mut left = ast::MulExpr::Exp(self.parse_exp_expr()?);

        while let Some(mul_op @ (Token::MulOp | Token::DivOp | Token::ModOp)) = self.peek().cloned()
        {
            self.advance();
            let right = self.parse_exp_expr()?;
            left = ast::MulExpr::Mul(
                Box::new(left),
                mul_op, // * or / or %
                right,
            )
        }

        Ok(left)
    }

    // exp-expr -> unary-expr | unary-expr • T_EXPOP • exp-expr
    fn parse_exp_expr(&mut self) -> Result<ast::ExpExpr, ParseError> {
        let left = self.parse_unary_expr()?;
        let ret: ast::ExpExpr;

        if let Some(Token::ExpOp) = self.peek() {
            self.advance();
            let right = self.parse_exp_expr()?;
            ret = ast::ExpExpr::Exp(left, Token::AssignOp, Box::new(right));
        } else {
            ret = ast::ExpExpr::Unary(left);
        }

        Ok(ret)
    }

    // unary-expr -> primary | unary-op • unary-expr
    // unary-op -> T_SUBOP | T_BOOLEANOT | T_BITWISENOT
    fn parse_unary_expr(&mut self) -> Result<ast::UnaryExpr, ParseError> {
        match self.peek().cloned() {
            Some(t @ (Token::SubOp | Token::BooleanNot | Token::BitwiseNot)) => {
                self.advance();
                let right = self.parse_unary_expr()?;
                return Ok(ast::UnaryExpr::Unary(t, Box::new(right)));
            }
            _ => return Ok(ast::UnaryExpr::Primary(self.parse_primary_expr()?)),
        }
    }

    // primary-expr -> T_IDENTIFIER | T_INTLIT | T_FLOATLIT | T_STRINGLIT | T_PARENL • expr • T_PARENR | fn-call
    fn parse_primary_expr(&mut self) -> Result<ast::PrimaryExpr, ParseError> {
        if self.peek().is_none() {
            return Err(ParseError::UnexpectedEOF);
        }

        match self.peek().unwrap() {
            Token::Identifier(_) => {
                if self.peek_next() == Some(&Token::ParenL) {
                    return Ok(ast::PrimaryExpr::Call(self.parse_fn_call()?));
                }

                let ident = self.consume_identifier()?;
                Ok(ast::PrimaryExpr::Ident(Token::Identifier(ident)))
            }

            Token::IntLit(_) => {
                let intlit = self.consume_intlit()?;
                Ok(ast::PrimaryExpr::IntLit(Token::IntLit(intlit)))
            }

            Token::FloatLit(_) => {
                let floatlit = self.consume_floatlit()?;
                Ok(ast::PrimaryExpr::FloatLit(Token::FloatLit(floatlit)))
            }

            Token::StringLit(_) => {
                let str = self.consume_stringlit()?;
                Ok(ast::PrimaryExpr::StringLit(Token::StringLit(str)))
            }

            Token::ParenL => {
                self.consume(Token::ParenL)?;
                let expr = self.parse_expr()?;
                self.consume(Token::ParenR)?;

                Ok(ast::PrimaryExpr::Paren(
                    Token::ParenL,
                    Box::new(expr),
                    Token::ParenR,
                ))
            }

            _ => Err(ParseError::UnexpectedToken(self.peek().unwrap().clone())),
        }
    }

    // fn-call -> T_IDENTIFIER • T_PARENL • fn-args • T_PARENR
    fn parse_fn_call(&mut self) -> Result<ast::FnCall, ParseError> {
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let args = self.parse_fn_args()?;
        self.consume(Token::ParenR)?;

        Ok(ast::FnCall {
            i: Token::Identifier(ident),
            pl: Token::ParenL,
            a: args,
            pr: Token::ParenR,
        })
    }

    // fn-args -> expr | expr • T_COMMA • fn-args | EPSILON
    fn parse_fn_args(&mut self) -> Result<ast::FnArgs, ParseError> {
        let mut params: Vec<ast::Expr> = Vec::new(); // Epsilon

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
