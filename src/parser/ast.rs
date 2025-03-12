use crate::lexer::token::Token;

enum AstNode {
    // every single type of node
    // initialize w/ root node of type CompUnit
}

pub struct TranslationUnit {
    pub dl: DeclList,
}

pub struct DeclList {
    pub d: Vec<Decl>,
}

pub struct Decl {
    pub vd: VarDecl,
    pub fd: FnDecl,
}

pub struct VarDecl {
    pub t: Type,
    pub i: Token, // Identifier,
    pub a: Token, // AssignOp,
    pub e: Expr,
    pub s: Token, // Semicolon,
}
pub type VarDeclNode = (TypeNode, Token, ExprNode, Token, Token);

// parse function will shrimply match and predict variables
// composed within that struct and returns a result<node,parseerror> of that struct's type

pub struct FnDecl {
    pub t: Type,
    pub i: Token, // Identifier,
    pub a: Token, // ParenL,
    pub e: Params,
    pub s: Token, // ParenR,
    pub b: Block,
}

pub struct Type {
    pub t: Token, // {Int,String,/Float}Lit
}
type TypeNode = Token;

pub enum Params {
    Single(Param),
    Multiple(
        Param,
        Token, // Comma
        Box<Params>,
    ),
    None,
}

pub struct Param {
    pub t: Type,
    pub i: Token, // Identifier
}

pub struct Block {
    pub l: Token, // BraceL
    pub s: Stmts,
    pub r: Token, // BraceR
}

pub struct Stmts {
    pub s: Vec<Stmt>,
}

pub enum Stmt {
    For(ForStmt),
    If(IfStmt),
    Ret(RetStmt),
    Var(VarDecl),
    Expr(ExprStmt),
}

pub struct ForStmt {
    pub f: Token,  // For
    pub pl: Token, // ParenL
    pub init: ExprStmt,
    pub cond: ExprStmt,
    pub inc: Expr,
    pub pr: Token, // ParenR
    pub b: Block,
}

pub struct IfStmt {
    pub i: Token,  // If
    pub pl: Token, // ParenL
    pub e: Expr,
    pub pr: Token, // ParenR
    pub ib: Block,
    pub el: Token, // Else
    pub eb: Block,
}

pub struct RetStmt {
    pub r: Token, // Ret
    pub e: Option<Expr>,
    pub s: Token, // Semicolon
}

pub struct ExprStmt {
    pub e: Expr,
    pub s: Token, // Semicolon,
}

pub struct Expr {
    pub ae: AssignExpr,
}

pub enum AssignExpr {
    BitwiseOr(BitwiseOrExpr),
    Assign(
        Box<AssignExpr>,
        Token, // AssignOp
        BitwiseOrExpr,
    ),
}

pub enum BitwiseOrExpr {
    BitwiseAnd(BitwiseAndExpr),
    BitwiseOr(
        Box<BitwiseOrExpr>,
        Token, // BitwiseOr
        BitwiseAndExpr,
    ),
}

pub enum BitwiseAndExpr {
    Bool(BoolExpr),
    BitwiseAnd(
        Box<BitwiseAndExpr>,
        Token, // BitwiseAnd
        BoolExpr,
    ),
}

pub enum BoolExpr {
    Comp(CompExpr),
    Bool(
        Box<BoolExpr>,
        Token, // Boolean{And,Or}
        CompExpr,
    ),
}

pub enum CompExpr {
    Shift(ShiftExpr),
    Comp(
        Box<CompExpr>,
        Token, // {Greater,Less}Than, Equals
        ShiftExpr,
    ),
}

pub enum ShiftExpr {
    Add(AddExpr),
    Shift(
        Box<ShiftExpr>,
        Token, // Shift{Left,Right}
        AddExpr,
    ),
}

pub enum AddExpr {
    Mul(MulExpr),
    Add(
        Box<AddExpr>,
        Token, // {Add,Sub}Op
        MulExpr,
    ),
}

pub enum MulExpr {
    Unary(ExpExpr),
    Mul(
        Box<MulExpr>,
        Token, // {Mul,Div,Mod}Op
        ExpExpr,
    ),
}

pub enum ExpExpr {
    Unary(UnaryExpr),
    Exp(
        Box<ExpExpr>,
        Token, // ExpOp
        UnaryExpr,
    ),
}

pub enum UnaryExpr {
    Primary(PrimaryExpr),
    Unary(
        Token, // SubOp,{Boolean,Bitwise}Not
        Box<UnaryExpr>,
    ),
}

pub enum PrimaryExpr {
    IntLit(Token),
    FloatLit(Token),
    StringLit(Token),
    Ident(Token),
    Paren(
        Token, // ParenL
        Box<Expr>,
        Token, // ParenR
    ),
    Call(FnCall),
}

pub struct FnCall {
    pub i: Token,  // Identifier
    pub pl: Token, // ParenL
    pub a: FnArgs,
    pub pr: Token, // ParenR
}

pub enum FnArgs {
    Single(Box<Expr>),
    Multiple(
        Box<Expr>,
        Token, // Comma
        Box<FnArgs>,
    ),
    None,
}
