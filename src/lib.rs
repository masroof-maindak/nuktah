pub mod lexer;
pub mod parser;

use parser::ast::*;

#[derive(Debug)]
pub enum CompilerError {
    LexerError(lexer::lexer::LexerError),
    ParserError(parser::parser::ParseError),
}

impl From<lexer::lexer::LexerError> for CompilerError {
    fn from(err: lexer::lexer::LexerError) -> CompilerError {
        CompilerError::LexerError(err)
    }
}

impl From<parser::parser::ParseError> for CompilerError {
    fn from(err: parser::parser::ParseError) -> CompilerError {
        CompilerError::ParserError(err) 
    }
}

fn print_expr(expr: &Expr, indent: &str) {
    println!("{}Expression:", indent);
    print_assign_expr(&expr, &format!("{}  ", indent));
}

fn print_assign_expr(expr: &AssignExpr, indent: &str) {
    match expr {
        AssignExpr::Bool(bool_expr) => print_bool_expr(bool_expr, indent),
        AssignExpr::Assign(left, op, right) => {
            println!("{}Assignment:", indent);
            print_bool_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_assign_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_bool_expr(expr: &BoolExpr, indent: &str) {
    match expr {
        BoolExpr::BitOr(bitwise) => print_bitwise_or_expr(bitwise, indent),
        BoolExpr::Bool(left, op, right) => {
            println!("{}Boolean Operation:", indent);
            print_bool_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_bitwise_or_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_bitwise_or_expr(expr: &BitOrExpr, indent: &str) {
    match expr {
        BitOrExpr::BitAnd(and_expr) => print_bitwise_and_expr(and_expr, indent),
        BitOrExpr::BitOr(left, op, right) => {
            println!("{}Bitwise OR:", indent);
            print_bitwise_or_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_bitwise_and_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_bitwise_and_expr(expr: &BitAndExpr, indent: &str) {
    match expr {
        BitAndExpr::Comp(comp_expr) => print_comp_expr(comp_expr, indent),
        BitAndExpr::BitAnd(left, op, right) => {
            println!("{}Bitwise AND:", indent);
            print_bitwise_and_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_comp_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_comp_expr(expr: &CompExpr, indent: &str) {
    match expr {
        CompExpr::Shift(shift_expr) => print_shift_expr(shift_expr, indent),
        CompExpr::Comp(left, op, right) => {
            println!("{}Comparison:", indent);
            print_comp_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_shift_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_shift_expr(expr: &ShiftExpr, indent: &str) {
    match expr {
        ShiftExpr::Add(add_expr) => print_add_expr(add_expr, indent),
        ShiftExpr::Shift(left, op, right) => {
            println!("{}Shift:", indent);
            print_shift_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_add_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_add_expr(expr: &AddExpr, indent: &str) {
    match expr {
        AddExpr::Mul(mul_expr) => print_mul_expr(mul_expr, indent),
        AddExpr::Add(left, op, right) => {
            println!("{}Addition/Subtraction:", indent);
            print_add_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_mul_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_mul_expr(expr: &MulExpr, indent: &str) {
    match expr {
        MulExpr::Exp(exp_expr) => print_exp_expr(exp_expr, indent),
        MulExpr::Mul(left, op, right) => {
            println!("{}Multiplication/Division/Modulo:", indent);
            print_mul_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_exp_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_exp_expr(expr: &ExpExpr, indent: &str) {
    match expr {
        ExpExpr::Unary(unary_expr) => print_unary_expr(unary_expr, indent),
        ExpExpr::Exp(left, op, right) => {
            println!("{}Exponentiation:", indent);
            print_unary_expr(left, &format!("{}  ", indent));
            println!("{}Operator: {:?}", indent, op);
            print_exp_expr(right, &format!("{}  ", indent));
        }
    }
}

fn print_unary_expr(expr: &UnaryExpr, indent: &str) {
    match expr {
        UnaryExpr::Primary(primary) => print_primary_expr(primary, indent),
        UnaryExpr::Unary(op, expr) => {
            println!("{}Unary Operation:", indent);
            println!("{}Operator: {:?}", indent, op);
            print_unary_expr(expr, &format!("{}  ", indent));
        }
    }
}

fn print_primary_expr(expr: &PrimaryExpr, indent: &str) {
    match expr {
        PrimaryExpr::Ident(id) => println!("{}Identifier: {:?}", indent, id),
        PrimaryExpr::IntLit(val) => println!("{}Integer Literal: {:?}", indent, val),
        PrimaryExpr::FloatLit(val) => println!("{}Float Literal: {:?}", indent, val),
        PrimaryExpr::StringLit(val) => println!("{}String Literal: {:?}", indent, val),
        PrimaryExpr::Paren(_, expr, _) => {
            println!("{}Parenthesized Expression:", indent);
            print_expr(expr, &format!("{}  ", indent));
        }
        PrimaryExpr::Call(fn_call) => {
            println!("{}Function Call:", indent);
            println!("{}  Name: {:?}", indent, fn_call.i);
            println!("{}  Arguments:", indent);
            for arg in &fn_call.a {
                print_expr(arg, &format!("{}    ", indent));
            }
        }
    }
}

fn print_stmt(stmt: &Stmt, indent: &str) {
    match stmt {
        Stmt::For(for_stmt) => {
            println!("{}For Loop:", indent);
            println!("{}  Initialization:", indent);
            print_expr(&for_stmt.init.e, &format!("{}    ", indent));
            println!("{}  Condition:", indent);
            print_expr(&for_stmt.cond.e, &format!("{}    ", indent));
            println!("{}  Increment:", indent);
            print_expr(&for_stmt.inc, &format!("{}    ", indent));
            println!("{}  Body:", indent);
            for s in &for_stmt.b.s {
                print_stmt(s, &format!("{}    ", indent));
            }
        }
        Stmt::If(if_stmt) => {
            println!("{}If Statement:", indent);
            println!("{}  Condition:", indent);
            print_expr(&if_stmt.e, &format!("{}    ", indent));
            println!("{}  If Block:", indent);
            for s in &if_stmt.ib.s {
                print_stmt(s, &format!("{}    ", indent));
            }
            println!("{}  Else Block:", indent);
            for s in &if_stmt.eb.s {
                print_stmt(s, &format!("{}    ", indent));
            }
        }
        Stmt::Ret(ret_stmt) => {
            println!("{}Return Statement:", indent);
            print_expr(&ret_stmt.e.e, &format!("{}  ", indent));
        }
        Stmt::VarDecl(var_decl) => {
            println!("{}Variable Declaration:", indent);
            println!("{}  Type: {:?}", indent, var_decl.t);
            println!("{}  Name: {:?}", indent, var_decl.i);
            println!("{}  Value:", indent);
            print_expr(&var_decl.e.e, &format!("{}    ", indent));
        }
        Stmt::ExprStmt(expr_stmt) => {
            println!("{}Expression Statement:", indent);
            print_expr(&expr_stmt.e, &format!("{}  ", indent));
        }
    }
}

fn print_ast_node(node: &TranslationUnit, indent: usize) {
    let indent_str = " ".repeat(indent);
    
    println!("{}TranslationUnit", indent_str);
    
    for decl in node {
        match decl {
            Decl::Fn(fn_decl) => {
                println!("{}Function Declaration:", indent_str);
                println!("{}  Return Type: {:?}", indent_str, fn_decl.t);
                println!("{}  Name: {:?}", indent_str, fn_decl.i);
                
                println!("{}  Parameters:", indent_str);
                for param in &fn_decl.p {
                    println!("{}    {:?} {:?}", indent_str, param.t, param.i);
                }
                
                println!("{}  Body:", indent_str);
                for stmt in &fn_decl.b.s {
                    print_stmt(stmt, &format!("{}    ", indent_str));
                }
            },
            Decl::Var(var_decl) => {
                println!("{}Variable Declaration:", indent_str);
                println!("{}  Type: {:?}", indent_str, var_decl.t);
                println!("{}  Name: {:?}", indent_str, var_decl.i);
                println!("{}  Value:", indent_str);
                print_expr(&var_decl.e.e, &format!("{}    ", indent_str));
            }
        }
    }
}

pub fn compile_src(src_code: &mut String) -> Result<(), CompilerError> {
    let tokens = lexer::lexer::tokenize_src_code(src_code)?;
    println!("Tokens:\n{:?}\n", tokens);

    let ast_root = parser::parser::parse_token_stream(&tokens)?;
    println!("AST:");
    print_ast_node(&ast_root, 4);

    Ok(())
}