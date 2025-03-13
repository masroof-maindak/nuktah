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

pub fn parse_token_stream(tokens: &Vec<Token>) -> Result<ast::TranslationUnitNode, ParseError> {
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

    fn get_type_token(&self) -> Option<ast::TypeNode> {
        match self.peek() {
            Some(token) => match token {
                Token::Int => Some(Token::Int),
                Token::String => Some(Token::String),
                Token::Float => Some(Token::Float),
                _ => None,
            },
            None => None,
        }
    }

    // ----------------------------------- //

    // translation-unit -> decl-list
    fn parse_translation_unit(&mut self) -> Result<ast::TranslationUnitNode, ParseError> {
        self.parse_decl_list()
    }

    // decl-list -> decl | decl • decl-list
    fn parse_decl_list(&mut self) -> Result<ast::DeclListNode, ParseError> {
        let mut root = Vec::new();

        // CHECK: Not too sure about this one.
        while self.pos < self.token_stream.len() {
            match self.peek() {
                Some(_) => {
                    let decl = self.parse_decl()?;
                    root.push(decl);
                }
                None => {
                    break;
                }
            }
        }

        Ok(root)
    }

    // decl -> var-decl | fn-decl
    fn parse_decl(&mut self) -> Result<ast::DeclNode, ParseError> {
        if self.peek() == Some(&Token::Function) {
            Ok(ast::DeclNode::FnDeclNode(self.parse_fn_decl()?))
        } else {
            Ok(ast::DeclNode::VarDeclNode(self.parse_var_decl()?))
        }
    }

    // fn-decl -> T_FUNC • type • T_IDENTIFIER • T_PARENL • params • T_PARENR • block
    fn parse_fn_decl(&mut self) -> Result<ast::FnDeclNode, ParseError> {
        self.consume(Token::Function)?;
        let type_token = self.get_type_token().ok_or(ParseError::ExpectedTypeToken)?;
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let params = self.parse_params()?;
        self.consume(Token::ParenR)?;
        let block = self.parse_block()?;

        Ok(ast::FnDeclNode {
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
    fn parse_var_decl(&mut self) -> Result<ast::VarDeclNode, ParseError> {
        let type_token = self.get_type_token().ok_or(ParseError::ExpectedTypeToken)?;
        let ident = self.consume_identifier()?;
        self.consume(Token::AssignOp)?;
        let expr_stmt = self.parse_expr_stmt()?;

        Ok(ast::VarDeclNode {
            t: type_token,
            i: Token::Identifier(ident),
            a: Token::AssignOp,
            e: expr_stmt,
        })
    }

    // params -> param | param • T_COMMA • params | EPSILON
    fn parse_params(&mut self) -> Result<ast::ParamsNode, ParseError> {
        let mut params: Vec<ast::ParamNode> = Vec::new();

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
    fn parse_param(&mut self) -> Result<ast::ParamNode, ParseError> {
        let type_token = self.get_type_token().ok_or(ParseError::ExpectedTypeToken)?;
        let ident = self.consume_identifier()?;

        Ok(ast::ParamNode {
            t: type_token,
            i: Token::Identifier(ident),
        })
    }

    // block -> T_BRACE_L • stmts • T_BRACE_R
    fn parse_block(&mut self) -> Result<ast::BlockNode, ParseError> {
        self.consume(Token::BraceL)?;
        let stmts = self.parse_stmts()?;
        self.consume(Token::BraceR)?;

        Ok(ast::BlockNode {
            l: Token::BraceL,
            s: stmts,
            r: Token::BraceR,
        })
    }

    // stmts -> stmt • stmts | EPSILON
    // stmt -> for-stmt | if-stmt | ret-stmt | var-decl | expr-stmt
    fn parse_stmts(&mut self) -> Result<ast::StmtsNode, ParseError> {
        let mut stmts: Vec<ast::StmtNode> = Vec::new();

        while let Some(t) = self.peek() {
            match t {
                Token::For => stmts.push(ast::StmtNode::For(self.parse_for_stmt()?)),
                Token::If => stmts.push(ast::StmtNode::If(self.parse_if_stmt()?)),
                Token::Return => stmts.push(ast::StmtNode::Ret(self.parse_ret_stmt()?)),
                Token::Int | Token::String | Token::Float => {
                    stmts.push(ast::StmtNode::VarDecl(self.parse_var_decl()?))
                }
                Token::BraceR => break, // end of encapsulating block...
                _ => stmts.push(ast::StmtNode::ExprStmtNode(self.parse_expr_stmt()?)),
            }
        }

        Ok(stmts)
    }

    // for-stmt -> T_FOR • T_PARENL • expr-stmt • expr-stmt • expr • T_PARENR • block
    fn parse_for_stmt(&mut self) -> Result<ast::ForStmtNode, ParseError> {
        self.consume(Token::For)?;
        self.consume(Token::ParenL)?;
        let init = self.parse_expr_stmt()?;
        let cond = self.parse_expr_stmt()?;
        let incr = self.parse_expr()?;
        self.consume(Token::ParenR)?;
        let block = self.parse_block()?;

        Ok(ast::ForStmtNode {
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
    fn parse_if_stmt(&mut self) -> Result<ast::IfStmtNode, ParseError> {
        self.consume(Token::If)?;
        self.consume(Token::ParenL)?;
        let cond = self.parse_expr()?;
        self.consume(Token::ParenR)?;
        let if_block = self.parse_block()?;
        self.consume(Token::Else)?;
        let else_block = self.parse_block()?;

        Ok(ast::IfStmtNode {
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
    fn parse_ret_stmt(&mut self) -> Result<ast::RetStmtNode, ParseError> {
        self.consume(Token::Return)?;
        let expr_stmt = self.parse_expr_stmt()?;

        Ok(ast::RetStmtNode {
            r: Token::Return,
            e: expr_stmt,
        })
    }

    // expr-stmt -> expr • T_Dot
    fn parse_expr_stmt(&mut self) -> Result<ast::ExprStmtNode, ParseError> {
        let expr = self.parse_expr()?;
        self.consume(Token::Dot)?;

        Ok(ast::ExprStmtNode {
            e: expr,
            s: Token::Dot,
        })
    }

    // expr -> assign-expr
    fn parse_expr(&mut self) -> Result<ast::ExprNode, ParseError> {
        Ok(self.parse_assign_expr()?)
    }

    // assign-expr -> bool-expr | assign-expr • T_ASSIGN • bool-expr
    fn parse_assign_expr(&mut self) -> Result<ast::AssignExprNode, ParseError> {
        // CHECK: this should be right-associative
        todo!();
    }

    // bool-expr -> bitwise-or-expr | bool-expr • bool_op • bitwise-or-expr
    // bool-op -> T_BOOLEANOR | T_BOOLEANAND
    fn parse_bool_expr(&mut self) -> Result<ast::BoolExprNode, ParseError> {
        let mut left = ast::BoolExprNode::BitwiseOr(self.parse_bitwise_or_expr()?);

        while let Some(bool_op @ (Token::BooleanOr | Token::BooleanAnd)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_bitwise_or_expr()?;
            left = ast::BoolExprNode::Bool(
                Box::new(left),
                bool_op, // && or ||
                right,
            )
        }

        Ok(left)
    }

    // bitwise-or-expr -> bitwise-and-expr | bitwise-or-expr • T_BITWISE_OR • bitwise-and-expr
    fn parse_bitwise_or_expr(&mut self) -> Result<ast::BitwiseOrExprNode, ParseError> {
        let mut left = ast::BitwiseOrExprNode::BitwiseAnd(self.parse_bitwise_and_expr()?);

        while let Some(Token::BitwiseOr) = self.peek().cloned() {
            self.advance();
            let right = self.parse_bitwise_and_expr()?;
            left = ast::BitwiseOrExprNode::BitwiseOr(Box::new(left), Token::BitwiseOr, right)
        }

        Ok(left)
    }

    // bitwise-and-expr -> comp-expr | bitwise-and-expr • T_BITWISEAND • comp-expr
    fn parse_bitwise_and_expr(&mut self) -> Result<ast::BitwiseAndExprNode, ParseError> {
        let mut left = ast::BitwiseAndExprNode::Comp(self.parse_comp_expr()?);

        while let Some(Token::BitwiseAnd) = self.peek().cloned() {
            self.advance();
            let right = self.parse_comp_expr()?;
            left = ast::BitwiseAndExprNode::BitwiseAnd(Box::new(left), Token::BitwiseAnd, right)
        }

        Ok(left)
    }

    // comp-expr -> shift-expr | comp-expr • comp_op • shift-expr
    // comp-op -> T_LESSTHAN, T_GREATERTHAN, T_EQUALSOP
    fn parse_comp_expr(&mut self) -> Result<ast::CompExprNode, ParseError> {
        let mut left = ast::CompExprNode::Shift(self.parse_shift_expr()?);

        while let Some(comp_op @ (Token::LessThan | Token::GreaterThan | Token::EqualsOp)) =
            self.peek().cloned()
        {
            self.advance();
            let right = self.parse_shift_expr()?;
            left = ast::CompExprNode::Comp(
                Box::new(left),
                comp_op, // < or > or ==
                right,
            )
        }

        Ok(left)
    }

    // shift-expr -> add-expr | shift-expr • shift-op • add-expr
    // shift-op -> T_SHIFTLEFT | T_SHIFTRIGHT
    fn parse_shift_expr(&mut self) -> Result<ast::ShiftExprNode, ParseError> {
        let mut left = ast::ShiftExprNode::Add(self.parse_add_expr()?);

        while let Some(shift_op @ (Token::ShiftLeft | Token::ShiftRight)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_add_expr()?;
            left = ast::ShiftExprNode::Shift(
                Box::new(left),
                shift_op, // << or >>
                right,
            )
        }

        Ok(left)
    }

    // add-expr -> mul-expr | add-expr • add-op • mul-expr
    // add-op -> T_ADDOP | T_SUBOP
    fn parse_add_expr(&mut self) -> Result<ast::AddExprNode, ParseError> {
        let mut left = ast::AddExprNode::Mul(self.parse_mul_expr()?);

        while let Some(add_op @ (Token::AddOp | Token::SubOp)) = self.peek().cloned() {
            self.advance();
            let right = self.parse_mul_expr()?;
            left = ast::AddExprNode::Add(
                Box::new(left),
                add_op, // + or -
                right,
            )
        }

        Ok(left)
    }

    // mul-expr -> exp-expr | mul-expr • mul-op • exp-expr
    // mul-op -> T_MULOP | T_DIVOP | T_MODOP
    fn parse_mul_expr(&mut self) -> Result<ast::MulExprNode, ParseError> {
        let mut left = ast::MulExprNode::Exp(self.parse_exp_expr()?);

        while let Some(mul_op @ (Token::MulOp | Token::DivOp | Token::ModOp)) = self.peek().cloned()
        {
            self.advance();
            let right = self.parse_exp_expr()?;
            left = ast::MulExprNode::Mul(
                Box::new(left),
                mul_op, // * or / or %
                right,
            )
        }

        Ok(left)
    }

    // exp-expr -> unary-expr | exp-expr • T_EXPOP • unary-expr
    fn parse_exp_expr(&mut self) -> Result<ast::ExpExprNode, ParseError> {
        // CHECK: this should be right-associative
        todo!();
    }

    // unary-expr -> primary | unary-op • unary-expr
    // unary-op -> T_SUBOP | T_BOOLEANOT | T_BITWISENOT
    fn parse_unary_expr(&mut self) -> Result<ast::UnaryExprNode, ParseError> {
        // CHECK: this should be right-associative
        todo!();
    }

    // primary-expr -> T_IDENTIFIER | T_INTLIT | T_FLOATLIT | T_STRINGLIT | T_PARENL • expr • T_PARENR | fn-call
    fn parse_primary_expr(&mut self) -> Result<ast::PrimaryExprNode, ParseError> {
        if self.peek().is_none() {
            return Err(ParseError::UnexpectedEOF);
        }

        match self.peek().unwrap() {
            Token::Identifier(_) => {
                if self.peek_next() == Some(&Token::ParenL) {
                    return Ok(ast::PrimaryExprNode::Call(self.parse_fn_call()?));
                }

                let ident = self.consume_identifier()?;
                Ok(ast::PrimaryExprNode::Ident(Token::Identifier(ident)))
            }

            Token::IntLit(_) => {
                let intlit = self.consume_intlit()?;
                Ok(ast::PrimaryExprNode::IntLit(Token::IntLit(intlit)))
            }

            Token::FloatLit(_) => {
                let floatlit = self.consume_floatlit()?;
                Ok(ast::PrimaryExprNode::FloatLit(Token::FloatLit(floatlit)))
            }

            Token::StringLit(_) => {
                let str = self.consume_stringlit()?;
                Ok(ast::PrimaryExprNode::StringLit(Token::StringLit(str)))
            }

            Token::ParenL => {
                self.consume(Token::ParenL)?;
                let expr = self.parse_expr()?;
                self.consume(Token::ParenR)?;

                Ok(ast::PrimaryExprNode::Paren(
                    Token::ParenL,
                    Box::new(expr),
                    Token::ParenR,
                ))
            }

            _ => Err(ParseError::UnexpectedToken(self.peek().unwrap().clone())),
        }
    }

    // fn-call -> T_IDENTIFIER • T_PARENL • fn-args • T_PARENR
    fn parse_fn_call(&mut self) -> Result<ast::FnCallNode, ParseError> {
        let ident = self.consume_identifier()?;
        self.consume(Token::ParenL)?;
        let args = self.parse_fn_args()?;
        self.consume(Token::ParenR)?;

        Ok(ast::FnCallNode {
            i: Token::Identifier(ident),
            pl: Token::ParenL,
            a: args,
            pr: Token::ParenR,
        })
    }

    // fn-args -> expr | expr • T_COMMA • fn-args | EPSILON
    fn parse_fn_args(&mut self) -> Result<ast::FnArgsNode, ParseError> {
        let mut params: Vec<ast::ExprNode> = Vec::new(); // Epsilon

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
