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
    pub type_tok: Type,
    pub ident: String, // Identifier,
    // AssignOp,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct FnDecl {
    // Fn
    pub type_tok: Type,
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
    pub type_tok: Type,
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
    pub expr: Expr,
    // Dot
}

pub type Expr = Option<AssignExpr>;
#[derive(Clone)]
pub enum AssignExpr {
    Bool(BoolExpr),
    Assign(
        BoolExpr,
        // AssignOp
        Box<AssignExpr>,
    ),
}
#[derive(Clone)]
pub enum BoolExpr {
    BitOr(BitOrExpr),
    Bool(
        Box<BoolExpr>,
        Token, // Boolean{And,Or}
        BitOrExpr,
    ),
}
#[derive(Clone)]
pub enum BitOrExpr {
    BitAnd(BitAndExpr),
    BitOr(
        Box<BitOrExpr>,
        // BitwiseOr
        BitAndExpr,
    ),
}
#[derive(Clone)]
pub enum BitAndExpr {
    Comp(CompExpr),
    BitAnd(
        Box<BitAndExpr>,
        // BitwiseAnd
        CompExpr,
    ),
}
#[derive(Clone)]
pub enum CompExpr {
    Shift(ShiftExpr),
    Comp(
        Box<CompExpr>,
        Token, // {Greater,Less}Than, Equals
        ShiftExpr,
    ),
}
#[derive(Clone)]
pub enum ShiftExpr {
    Add(AddExpr),
    Shift(
        Box<ShiftExpr>,
        Token, // Shift{Left,Right}
        AddExpr,
    ),
}
#[derive(Clone)]
pub enum AddExpr {
    Mul(MulExpr),
    Add(
        Box<AddExpr>,
        Token, // {Add,Sub}Op
        MulExpr,
    ),
}
#[derive(Clone)]
pub enum MulExpr {
    Exp(ExpExpr),
    Mul(
        Box<MulExpr>,
        Token, // {Mul,Div,Mod}Op
        ExpExpr,
    ),
}
#[derive(Clone)]
pub enum ExpExpr {
    Unary(UnaryExpr),
    Exp(
        UnaryExpr,
        // ExpOp
        Box<ExpExpr>,
    ),
}
#[derive(Clone)]
pub enum UnaryExpr {
    Primary(PrimaryExpr),
    Unary(
        Token, // SubOp,{Boolean,Bitwise}Not
        Box<UnaryExpr>,
    ),
}
#[derive(Clone)]
pub enum PrimaryExpr {
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    Ident(String),
    Paren(
        // ParenL
        Box<Expr>,
        // ParenR
    ),
    Call(FnCall),
}
#[derive(Clone, Debug)]
pub struct FnCall {
    pub ident: String, // Identifier
    // ParenL
    pub args: FnArgs,
    // ParenR
}

pub type FnArgs = Vec<Expr>;
