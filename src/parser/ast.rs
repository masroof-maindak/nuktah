use crate::lexer::token::Token;

#[derive(Debug)]
pub enum AstNode {
    TranslationUnitNode(TranslationUnitNode),
}

pub type TranslationUnitNode = DeclListNode;

pub type DeclListNode = Vec<DeclNode>;

#[derive(Debug)]
pub enum DeclNode {
    VarDeclNode(VarDeclNode),
    FnDeclNode(FnDeclNode),
}

#[derive(Debug)]
pub struct VarDeclNode {
    pub t: TypeNode,
    pub i: Token, // Identifier,
    pub a: Token, // AssignOp,
    pub e: ExprStmtNode,
}

// parse function will shrimply match and predict variables
// composed within that struct and returns a result<node,parseerror> of that struct's type

#[derive(Debug)]
pub struct FnDeclNode {
    pub f: Token, // Func
    pub t: TypeNode,
    pub i: Token,  // Identifier,
    pub pl: Token, // ParenL,
    pub p: ParamsNode,
    pub pr: Token, // ParenR,
    pub b: BlockNode,
}

pub struct Type {
    pub t: Token, // {Int,String,/Float}Lit
}
pub type TypeNode = Token;

pub type ParamsNode = Vec<ParamNode>;
#[derive(Debug)]
pub struct ParamNode {
    pub t: TypeNode,
    pub i: Token, // Identifier
}

#[derive(Debug)]
pub struct BlockNode {
    pub l: Token, // BraceL
    pub s: StmtsNode,
    pub r: Token, // BraceR
}

pub type StmtsNode = Vec<StmtNode>;

#[derive(Debug)]
pub enum StmtNode {
    For(ForStmtNode),
    If(IfStmtNode),
    Ret(RetStmtNode),
    VarDecl(VarDeclNode),
    ExprStmtNode(ExprStmtNode),
}

#[derive(Debug)]
pub struct ForStmtNode {
    pub f: Token,  // For
    pub pl: Token, // ParenL
    pub init: ExprStmtNode,
    pub cond: ExprStmtNode,
    pub inc: ExprNode,
    pub pr: Token, // ParenR
    pub b: BlockNode,
}

#[derive(Debug)]
pub struct IfStmtNode {
    pub i: Token,  // If
    pub pl: Token, // ParenL
    pub e: ExprNode,
    pub pr: Token, // ParenR
    pub ib: BlockNode,
    pub el: Token, // Else
    pub eb: BlockNode,
}

#[derive(Debug)]
pub struct RetStmtNode {
    pub r: Token, // Ret
    pub e: ExprStmtNode,
}

#[derive(Debug)]
pub struct ExprStmtNode {
    pub e: ExprNode,
    pub s: Token, // Dot,
}

pub type ExprNode = AssignExprNode;

#[derive(Debug)]
pub enum AssignExprNode {
    Bool(BoolExprNode),
    Assign(
        Box<AssignExprNode>,
        Token, // AssignOp
        BoolExprNode,
    ),
}

#[derive(Debug)]
pub enum BoolExprNode {
    BitwiseOr(BitwiseOrExprNode),
    Bool(
        Box<BoolExprNode>,
        Token, // Boolean{And,Or}
        BitwiseOrExprNode,
    ),
}

#[derive(Debug)]
pub enum BitwiseOrExprNode {
    BitwiseAnd(BitwiseAndExprNode),
    BitwiseOr(
        Box<BitwiseOrExprNode>,
        Token, // BitwiseOr
        BitwiseAndExprNode,
    ),
}

#[derive(Debug)]
pub enum BitwiseAndExprNode {
    Comp(CompExprNode),
    BitwiseAnd(
        Box<BitwiseAndExprNode>,
        Token, // BitwiseAnd
        CompExprNode,
    ),
}

#[derive(Debug)]
pub enum CompExprNode {
    Shift(ShiftExprNode),
    Comp(
        Box<CompExprNode>,
        Token, // {Greater,Less}Than, Equals
        ShiftExprNode,
    ),
}

#[derive(Debug)]
pub enum ShiftExprNode {
    Add(AddExprNode),
    Shift(
        Box<ShiftExprNode>,
        Token, // Shift{Left,Right}
        AddExprNode,
    ),
}

#[derive(Debug)]
pub enum AddExprNode {
    Mul(MulExprNode),
    Add(
        Box<AddExprNode>,
        Token, // {Add,Sub}Op
        MulExprNode,
    ),
}

#[derive(Debug)]
pub enum MulExprNode {
    Exp(ExpExprNode),
    Mul(
        Box<MulExprNode>,
        Token, // {Mul,Div,Mod}Op
        ExpExprNode,
    ),
}

#[derive(Debug)]
pub enum ExpExprNode {
    Unary(UnaryExprNode),
    Exp(
        Box<ExpExprNode>,
        Token, // ExpOp
        UnaryExprNode,
    ),
}

#[derive(Debug)]
pub enum UnaryExprNode {
    Primary(PrimaryExprNode),
    Unary(
        Token, // SubOp,{Boolean,Bitwise}Not
        Box<UnaryExprNode>,
    ),
}

#[derive(Debug)]
pub enum PrimaryExprNode {
    IntLit(Token),
    FloatLit(Token),
    StringLit(Token),
    Ident(Token),
    Paren(
        Token, // ParenL
        Box<ExprNode>,
        Token, // ParenR
    ),
    Call(FnCallNode),
}

#[derive(Debug)]
pub struct FnCallNode {
    pub i: Token,  // Identifier
    pub pl: Token, // ParenL
    pub a: FnArgsNode,
    pub pr: Token, // ParenR
}

pub type FnArgsNode = Vec<ExprNode>;