use super::instructions::TacInstr;
use crate::parser::ast::core::Stmt;

/// A labeled basic block of TAC instructions
#[derive(Debug)]
pub struct TacBlock {
    pub label: String,
    pub instrs: Vec<TacInstr>,
}

/// A "basic block" consisting of a sequence of statements.
#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

/// Loop context for tracking nested loops and their exit labels
#[derive(Debug, Clone)]
pub(crate) struct LoopContext {
    pub end_label: String,
}