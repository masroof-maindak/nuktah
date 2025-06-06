use crate::lexer::Token;

pub type TranslationUnit = DeclList;

pub type DeclList = Vec<Decl>;

#[derive(Debug)]
pub enum Decl {
    Var(VarDecl),
    Fn(FnDecl),
}

#[derive(Debug)]
pub struct VarDecl {
    pub t: Type,
    pub ident: String, // Identifier,
    // AssignOp,
    pub e: Expr,
}

#[derive(Debug)]
pub struct FnDecl {
    // Fn
    pub t: Type,
    pub ident: String, // Identifier,
    // ParenL,
    pub params: Vec<Param>,
    // ParenR,
    pub block: Block,
    // Dot
}

pub type Type = Token; // {Int,String,Float}Lit

#[derive(Debug)]
pub struct Param {
    pub t: Type,
    pub ident: String, // Identifier
}

pub type Block = Vec<Stmt>;

#[derive(Debug)]
pub enum Stmt {
    For(ForStmt),
    If(IfStmt),
    Ret(RetStmt),
    VarDecl(VarDecl),
    Expr(ExprStmt),
    Break,
}

#[derive(Debug)]
pub struct ForStmt {
    // For
    // ParenL
    pub init: Option<VarDecl>,
    pub cond: ExprStmt,
    pub updt: Expr,
    // ParenR
    pub block: Block,
}

#[derive(Debug)]
pub struct IfStmt {
    // If
    // ParenL
    pub cond: Expr,
    // ParenR
    pub if_block: Block,
    // Else
    pub else_block: Block,
}

pub type RetStmt = ExprStmt;

#[derive(Debug)]
pub struct ExprStmt {
    pub e: Expr,
    // Dot
}

pub type Expr = Option<AssignExpr>;

pub enum AssignExpr {
    Bool(BoolExpr),
    Assign(
        BoolExpr,
        // AssignOp
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
        // BitwiseOr
        BitAndExpr,
    ),
}

pub enum BitAndExpr {
    Comp(CompExpr),
    BitAnd(
        Box<BitAndExpr>,
        // BitwiseAnd
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
        // ExpOp
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
        // ParenL
        Box<Expr>,
        // ParenR
    ),
    Call(FnCall),
}

#[derive(Debug)]
pub struct FnCall {
    pub ident: String, // Identifier
    // ParenL
    pub args: FnArgs,
    // ParenR
}

pub type FnArgs = Vec<Expr>;
