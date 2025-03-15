use crate::lexer::token::Token;

pub type TranslationUnit = DeclList;

#[derive(Debug)]
pub struct DeclList {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Var(VarDecl),
    Fn(FnDecl),
}

#[derive(Debug)]
pub struct VarDecl {
    pub t: Type,
    pub i: Token, // Identifier,
    pub a: Token, // AssignOp,
    pub e: ExprStmt,
}

#[derive(Debug)]
pub struct FnDecl {
    pub f: Token, // Func
    pub t: Type,
    pub i: Token,  // Identifier,
    pub pl: Token, // ParenL,
    pub p: Params,
    pub pr: Token, // ParenR,
    pub b: Block,
    pub d: Token, // Dot
}

pub type Type = Token; // {Int,String,Float}Lit

pub type Params = Vec<Param>;

#[derive(Debug)]
pub struct Param {
    pub t: Type,
    pub i: Token, // Identifier
}

#[derive(Debug)]
pub struct Block {
    pub l: Token, // BraceL
    pub s: Stmts,
    pub r: Token, // BraceR
}

pub type Stmts = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    For(ForStmt),
    If(IfStmt),
    Ret(RetStmt),
    VarDecl(VarDecl),
    ExprStmt(ExprStmt),
}

#[derive(Debug)]
pub struct ForStmt {
    pub f: Token,  // For
    pub pl: Token, // ParenL
    pub init: VarDecl,
    pub cond: ExprStmt,
    pub inc: Expr,
    pub pr: Token, // ParenR
    pub b: Block,
}

#[derive(Debug)]
pub struct IfStmt {
    pub i: Token,  // If
    pub pl: Token, // ParenL
    pub e: Expr,
    pub pr: Token, // ParenR
    pub ib: Block,
    pub el: Token, // Else
    pub eb: Block,
}

#[derive(Debug)]
pub struct RetStmt {
    pub r: Token, // Ret
    pub e: ExprStmt,
}

#[derive(Debug)]
pub struct ExprStmt {
    pub e: Expr,
    pub s: Token, // Dot,
}

pub type Expr = AssignExpr;

pub enum AssignExpr {
    Bool(BoolExpr),
    Assign(
        BoolExpr,
        Token, // AssignOp
        Box<AssignExpr>,
    ),
}

pub enum BoolExpr {
    BitOr(BitOrExpr),
    Bool(
        Box<BoolExpr>,
        Token, // Boolean{And,Or}
        BitOrExpr,
    ),
}

pub enum BitOrExpr {
    BitAnd(BitAndExpr),
    BitOr(
        Box<BitOrExpr>,
        Token, // BitwiseOr
        BitAndExpr,
    ),
}

pub enum BitAndExpr {
    Comp(CompExpr),
    BitAnd(
        Box<BitAndExpr>,
        Token, // BitwiseAnd
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
    Exp(ExpExpr),
    Mul(
        Box<MulExpr>,
        Token, // {Mul,Div,Mod}Op
        ExpExpr,
    ),
}

pub enum ExpExpr {
    Unary(UnaryExpr),
    Exp(
        UnaryExpr,
        Token, // ExpOp
        Box<ExpExpr>,
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

#[derive(Debug)]
pub struct FnCall {
    pub i: Token,  // Identifier
    pub pl: Token, // ParenL
    pub args: FnArgs,
    pub pr: Token, // ParenR
}

pub type FnArgs = Vec<Expr>;
