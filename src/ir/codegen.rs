use super::instructions::{TacInstr, TacCode};
use super::values::TacValue;
use super::blocks::TacBlock;
use crate::lexer::Token;

impl TacCode {
    /// Display TAC code instruction in a readable format
    pub fn format_instruction(&self, index: usize) -> String {
        match self {
            TacCode::Assign(dest, src) => {
                format!("{:3}: {} = {}", index, dest, src)
            }
            TacCode::BinOp(dest, lhs, op, rhs) => {
                format!("{:3}: {} = {} {} {}", index, dest, lhs, op, rhs)
            }
            TacCode::UnaryOp(dest, op, operand) => {
                format!("{:3}: {} = {} {}", index, dest, op, operand)
            }
            TacCode::Label(label) => {
                format!("{:3}: {}:", index, label)
            }
            TacCode::Goto(label) => {
                format!("{:3}: goto {}", index, label)
            }
            TacCode::IfFalse(cond, label) => {
                format!("{:3}: ifFalse {} goto {}", index, cond, label)
            }
            TacCode::IfTrue(cond, label) => {
                format!("{:3}: ifTrue {} goto {}", index, cond, label)
            }
            TacCode::BeginFunc(name) => {
                format!("{:3}: BeginFunc {}", index, name)
            }
            TacCode::EndFunc => {
                format!("{:3}: EndFunc", index)
            }
            TacCode::Return(value) => {
                match value {
                    Some(val) => format!("{:3}: return {}", index, val),
                    None => format!("{:3}: return", index),
                }
            }
            TacCode::Call(dest, func, args) => {
                format!("{:3}: {} = call {}({})", index, dest, func, args.join(", "))
            }
            TacCode::Nop => {
                format!("{:3}: nop", index)
            }
        }
    }
}

/// Convert Token operator to string representation
pub fn token_to_op_string(token: &Token) -> String {
    match token {
        Token::AddOp => "+".to_string(),
        Token::SubOp => "-".to_string(),
        Token::MulOp => "*".to_string(),
        Token::DivOp => "/".to_string(),
        Token::ModOp => "%".to_string(),
        Token::ExpOp => "^".to_string(),
        Token::EqualsOp => "==".to_string(),
        Token::LessThan => "<".to_string(),
        Token::GreaterThan => ">".to_string(),
        Token::BitwiseAnd => "&".to_string(),
        Token::BitwiseOr => "|".to_string(),
        Token::BooleanAnd => "&&".to_string(),
        Token::BooleanOr => "||".to_string(),
        Token::ShiftLeft => "<<".to_string(),
        Token::ShiftRight => ">>".to_string(),
        Token::BooleanNot => "!".to_string(),
        Token::BitwiseNot => "~".to_string(),
        _ => format!("{:?}", token), // Fallback for other tokens
    }
}

/// Convert TacValue to string representation
fn tac_value_to_string(value: &TacValue) -> String {
    match value {
        TacValue::Var(name) => name.clone(),
        TacValue::IntLit(val) => val.to_string(),
        TacValue::FloatLit(val) => val.to_string(),
        TacValue::StringLit(val) => format!("\"{}\"", val),
    }
}

/// Convert TAC instructions to simplified TAC code
fn tac_instr_to_code(instr: &TacInstr) -> Vec<TacCode> {
    match instr {
        TacInstr::AssignOp(dest, src) => {
            vec![TacCode::Assign(dest.clone(), tac_value_to_string(src))]
        }
        
        TacInstr::BinOp(dest, lhs, op, rhs) => {
            vec![TacCode::BinOp(
                dest.clone(),
                tac_value_to_string(lhs),
                token_to_op_string(op),
                tac_value_to_string(rhs)
            )]
        }
        
        TacInstr::UnaryOp(dest, op, operand) => {
            vec![TacCode::UnaryOp(
                dest.clone(),
                token_to_op_string(op),
                tac_value_to_string(operand)
            )]
        }
        
        TacInstr::Label(label) => {
            vec![TacCode::Label(label.clone())]
        }
        
        TacInstr::Jump(label) => {
            vec![TacCode::Goto(label.clone())]
        }
        
        TacInstr::CondJump(cond, true_label, false_label) => {
            // Generate both true and false jumps
            vec![
                TacCode::IfTrue(tac_value_to_string(cond), true_label.clone()),
                TacCode::Goto(false_label.clone())
            ]
        }
        
        TacInstr::FnDecl(name) => {
            vec![TacCode::BeginFunc(name.clone())]
        }
        
        TacInstr::Return(value) => {
            vec![TacCode::Return(value.as_ref().map(tac_value_to_string))]
        }
        
        TacInstr::Call(dest, func, args) => {
            let arg_strings: Vec<String> = args.iter().map(tac_value_to_string).collect();
            vec![TacCode::Call(dest.clone(), func.clone(), arg_strings)]
        }
        
        TacInstr::Nop => {
            vec![TacCode::Nop]
        }
    }
}

/// Convert TAC blocks to simplified TAC code
pub fn generate_tac_code(tac_blocks: Vec<TacBlock>) -> Vec<TacCode> {
    let mut code = Vec::new();
    
    for block in tac_blocks {
        for instr in block.instrs {
            let mut instr_code = tac_instr_to_code(&instr);
            code.append(&mut instr_code);
        }
    }
    
    code
}