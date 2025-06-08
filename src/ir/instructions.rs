use crate::lexer::Token;
use super::values::TacValue;

/// TAC Instruction types
#[derive(Debug, Clone)]
pub enum TacInstr {
    // Basic operations
    AssignOp(String, TacValue),                    // x = y
    BinOp(String, TacValue, Token, TacValue),      // x = y op z
    UnaryOp(String, Token, TacValue),              // x = op y
    
    // Control flow
    Label(String),                                 // label:
    Jump(String),                                  // goto label
    CondJump(TacValue, String, String),            // if cond goto label1 else label2
    
    // Function related
    FnDecl(String),                                // function name
    Return(Option<TacValue>),                      // return [value]
    Call(String, String, Vec<TacValue>),           // result = call func(args)
    
    // Other
    Nop,                                           // no operation
}

/// Simplified TAC instruction representation for code generation
#[derive(Debug, Clone)]
pub enum TacCode {
    // Assignment operations
    Assign(String, String),                        // x = y
    BinOp(String, String, String, String),         // x = y op z  
    UnaryOp(String, String, String),               // x = op y
    
    // Control flow
    Label(String),                                 // label:
    Goto(String),                                  // goto label
    IfFalse(String, String),                       // ifFalse x goto label
    IfTrue(String, String),                        // ifTrue x goto label
    
    // Function operations
    BeginFunc(String),                             // BeginFunc name
    EndFunc,                                       // EndFunc
    Return(Option<String>),                        // return [x]
    Call(String, String, Vec<String>),             // x = call f(args)
    
    // Special
    Nop,                                           // nop
}
