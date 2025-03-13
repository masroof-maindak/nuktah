use crate::lexer::token::Token;
use crate::parser::ast;

#[derive(Debug)]
pub enum ParseError {
    FailedToFindToken(Token),
    ExpectedTypeToken,
    ExpectedIdentifier,
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
                Token::BraceR => break, // end of block...
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

    // assign-expr -> bitwise-or-expr | assign-expr • T_ASSIGN • bitwise-or-expr
    fn parse_assign_expr(&mut self) -> Result<ast::AssignExprNode, ParseError> {
        todo!();
    }

    // bitwise-or-expr -> bitwise-and-expr | bitwise-or-expr • T_BITWISE_OR • bitwise-and-expr
    fn parse_bitwise_or_expr(&mut self) -> Result<ast::BitwiseOrExprNode, ParseError> {
        todo!();
    }

    // bitwise-and-expr -> bool-expr | bitwise-and-expr • T\_BITWISEAND • bool-expr
    fn parse_bitwise_and_expr(&mut self) -> Result<ast::BitwiseAndExprNode, ParseError> {
        todo!();
    }

    // bool-expr -> comp-expr | bool-expr • bool_op • comp-expr
    // bool-op -> T_BOOLEANOR | T_BOOLEANAND
    fn parse_bool_expr(&mut self) -> Result<ast::BoolExprNode, ParseError> {
        todo!();
    }

    // comp-expr -> shift-expr | comp-expr • comp_op • shift-expr
    // comp-op -> T_LESSTHAN, T_GREATERTHAN, T_EQUALSOP
    fn parse_comp_expr(&mut self) -> Result<ast::CompExprNode, ParseError> {
        todo!();
    }

    // shift-expr -> add-expr | shift-expr • shift-op • add-expr
    // shift-op -> T\_SHIFTLEFT | T\_SHIFTRIGHT
    fn parse_shift_expr(&mut self) -> Result<ast::ShiftExprNode, ParseError> {
        todo!();
    }

    // add-expr -> mul-expr | add-expr • add-op • mul-expr
    // add-op -> T\_ADDOP | T\_SUBOP
    fn parse_add_expr(&mut self) -> Result<ast::AddExprNode, ParseError> {
        todo!();
    }

    // mul-expr -> exp-expr | mul-expr • mul-op • exp-expr
    // mul-op -> T\_MULOP | T\_DIVOP | T\_MODOP
    fn parse_mul_expr(&mut self) -> Result<ast::MulExprNode, ParseError> {
        todo!();
    }

    // exp-expr -> unary-expr | exp-expr • T\_EXPOP • unary-expr
    fn parse_exp_expr(&mut self) -> Result<ast::ExpExprNode, ParseError> {
        todo!();
    }

    // unary-expr -> primary | unary-op • unary-expr
    // unary-op -> T\_SUBOP | T\_BOOLEANOT | T\_BITWISENOT
    fn parse_unary_expr(&mut self) -> Result<ast::UnaryExprNode, ParseError> {
        todo!();
    }

    // primary-expr -> T_IDENTIFIER | T_INTLIT | T_FLOATLIT | T_STRINGLIT | T_PARENL • expr • T_PARENR | fn-call
    fn parse_primary_expr(&mut self) -> Result<ast::PrimaryExprNode, ParseError> {
        todo!();
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
        let mut params: Vec<ast::ExprNode> = Vec::new();

        if let Some(Token::ParenR) = self.peek() {
            return Ok(params);
        }

        params.push(self.parse_expr()?);

        while let Some(Token::Comma) = self.peek() {
            self.consume(Token::Comma)?;
            params.push(self.parse_expr()?);
        }

        Ok(params)
    }
}
