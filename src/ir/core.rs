use super::blocks::{TacBlock};
use super::instructions::TacInstr;
use super::values::TacValue;
use crate::lexer::Token;
use crate::parser::ast::core::*;

/// Loop context for tracking nested loops and their exit labels
type LoopContext = String;

/// TAC Block Generator state aka our Control Flow Graph (CFG)
struct TacGenerator {
    blocks: Vec<TacBlock>,
    current_block: Vec<TacInstr>,
    current_label: String,
    temp_counter: usize,
    label_counter: usize,
    loop_stack: Vec<LoopContext>, // Stack to track nested loops and break statements
}

impl TacGenerator {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            current_block: Vec::new(),
            current_label: "entry".to_string(),
            temp_counter: 0,
            label_counter: 0,
            loop_stack: Vec::new(),
        }
    }

    fn new_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    fn new_label(&mut self) -> String {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    fn emit(&mut self, instr: TacInstr) {
        self.current_block.push(instr);
    }

    fn start_block(&mut self, label: String) {
        // Finish curr_block block if it has instructions
        if !self.current_block.is_empty() || !self.blocks.is_empty() {
            self.blocks.push(TacBlock {
                label: self.current_label.clone(),
                instrs: std::mem::take(&mut self.current_block),
            });
        }

        self.current_label = label.clone();
        self.emit(TacInstr::Label(label));
    }

    fn finish(&mut self) {
        if !self.current_block.is_empty() {
            self.blocks.push(TacBlock {
                label: self.current_label.clone(),
                instrs: std::mem::take(&mut self.current_block),
            });
        }
    }

    /// Push a new loop context onto the stack
    fn push_loop(&mut self, end_label: String) {
        self.loop_stack.push(end_label);
    }

    /// Pop the curr_block loop context from the stack
    fn pop_loop(&mut self) {
        self.loop_stack.pop();
    }

    /// Get the curr_block loop's end label for break statements
    fn current_loop_end(&self) -> Option<&String> {
        self.loop_stack.last().map(|ctx| ctx)
    }

    fn generate_expr(&mut self, expr: &Option<AssignExpr>) -> Option<TacValue> {
        match expr {
            Some(assign_expr) => Some(self.generate_assign_expr(assign_expr)),
            None => None,
        }
    }

    fn generate_assign_expr(&mut self, expr: &AssignExpr) -> TacValue {
        match expr {
            AssignExpr::Bool(bool_expr) => self.generate_bool_expr(bool_expr),
            AssignExpr::Assign(lhs, rhs) => {
                let rhs_val = self.generate_assign_expr(rhs);
                // For assignment, lhs should be a variable
                if let BoolExpr::BitOr(BitOrExpr::BitAnd(BitAndExpr::Comp(CompExpr::Shift(
                    ShiftExpr::Add(AddExpr::Mul(MulExpr::Exp(ExpExpr::Unary(UnaryExpr::Primary(
                        PrimaryExpr::Ident(var_name),
                    ))))),
                )))) = lhs
                {
                    self.emit(TacInstr::AssignOp(var_name.clone(), rhs_val.clone()));
                    rhs_val
                } else {
                    // For complex lhs, we need to handle it differently
                    let lhs_val = self.generate_bool_expr(lhs);
                    if let TacValue::Var(var_name) = lhs_val {
                        self.emit(TacInstr::AssignOp(var_name.clone(), rhs_val.clone()));
                        rhs_val
                    } else {
                        rhs_val // Fallback
                    }
                }
            }
        }
    }

    fn generate_bool_expr(&mut self, expr: &BoolExpr) -> TacValue {
        match expr {
            BoolExpr::BitOr(bit_or) => self.generate_bit_or_expr(bit_or),
            BoolExpr::Bool(lhs, op, rhs) => {
                let lhs_val = self.generate_bool_expr(lhs);
                let rhs_val = self.generate_bit_or_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    op.clone(),
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_bit_or_expr(&mut self, expr: &BitOrExpr) -> TacValue {
        match expr {
            BitOrExpr::BitAnd(bit_and) => self.generate_bit_and_expr(bit_and),
            BitOrExpr::BitOr(lhs, rhs) => {
                let lhs_val = self.generate_bit_or_expr(lhs);
                let rhs_val = self.generate_bit_and_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    Token::BitwiseOr,
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_bit_and_expr(&mut self, expr: &BitAndExpr) -> TacValue {
        match expr {
            BitAndExpr::Comp(comp) => self.generate_comp_expr(comp),
            BitAndExpr::BitAnd(lhs, rhs) => {
                let lhs_val = self.generate_bit_and_expr(lhs);
                let rhs_val = self.generate_comp_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    Token::BitwiseAnd,
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_comp_expr(&mut self, expr: &CompExpr) -> TacValue {
        match expr {
            CompExpr::Shift(shift) => self.generate_shift_expr(shift),
            CompExpr::Comp(lhs, op, rhs) => {
                let lhs_val = self.generate_comp_expr(lhs);
                let rhs_val = self.generate_shift_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    op.clone(),
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_shift_expr(&mut self, expr: &ShiftExpr) -> TacValue {
        match expr {
            ShiftExpr::Add(add) => self.generate_add_expr(add),
            ShiftExpr::Shift(lhs, op, rhs) => {
                let lhs_val = self.generate_shift_expr(lhs);
                let rhs_val = self.generate_add_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    op.clone(),
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_add_expr(&mut self, expr: &AddExpr) -> TacValue {
        match expr {
            AddExpr::Mul(mul) => self.generate_mul_expr(mul),
            AddExpr::Add(lhs, op, rhs) => {
                let lhs_val = self.generate_add_expr(lhs);
                let rhs_val = self.generate_mul_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    op.clone(),
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_mul_expr(&mut self, expr: &MulExpr) -> TacValue {
        match expr {
            MulExpr::Exp(exp) => self.generate_exp_expr(exp),
            MulExpr::Mul(lhs, op, rhs) => {
                let lhs_val = self.generate_mul_expr(lhs);
                let rhs_val = self.generate_exp_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    op.clone(),
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_exp_expr(&mut self, expr: &ExpExpr) -> TacValue {
        match expr {
            ExpExpr::Unary(unary) => self.generate_unary_expr(unary),
            ExpExpr::Exp(lhs, rhs) => {
                let lhs_val = self.generate_unary_expr(lhs);
                let rhs_val = self.generate_exp_expr(rhs);
                let result = self.new_temp();
                self.emit(TacInstr::BinOp(
                    result.clone(),
                    lhs_val,
                    Token::ExpOp,
                    rhs_val,
                ));
                TacValue::Var(result)
            }
        }
    }

    fn generate_unary_expr(&mut self, expr: &UnaryExpr) -> TacValue {
        match expr {
            UnaryExpr::Primary(primary) => self.generate_primary_expr(primary),
            UnaryExpr::Unary(op, expr) => {
                let expr_val = self.generate_unary_expr(expr);
                let result = self.new_temp();
                self.emit(TacInstr::UnaryOp(result.clone(), op.clone(), expr_val));
                TacValue::Var(result)
            }
        }
    }

    fn generate_primary_expr(&mut self, expr: &PrimaryExpr) -> TacValue {
        match expr {
            PrimaryExpr::IntLit(val) => TacValue::IntLit(*val),
            PrimaryExpr::FloatLit(val) => TacValue::FloatLit(*val),
            PrimaryExpr::StringLit(val) => TacValue::StringLit(val.clone()),
            PrimaryExpr::BoolLit(val) => TacValue::BoolLit(*val),
            PrimaryExpr::Ident(name) => TacValue::Var(name.clone()),
            PrimaryExpr::Paren(expr) => self.generate_expr(expr).unwrap_or(TacValue::IntLit(0)),
            PrimaryExpr::Call(call) => {
                let mut args = Vec::new();
                for arg in &call.args {
                    if let Some(val) = self.generate_expr(arg) {
                        args.push(val);
                    }
                }
                let result = self.new_temp();
                self.emit(TacInstr::Call(result.clone(), call.ident.clone(), args));
                TacValue::Var(result)
            }
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::VarDecl(var_decl) => {
                if let Some(val) = self.generate_expr(&var_decl.expr) {
                    self.emit(TacInstr::AssignOp(var_decl.ident.clone(), val));
                }
            }
            Stmt::Expr(expr_stmt) => {
                self.generate_expr(&expr_stmt.expr);
            }
            Stmt::Ret(ret_stmt) => {
                let val = self.generate_expr(&ret_stmt.expr);
                self.emit(TacInstr::Return(val));
            }
            Stmt::If(if_stmt) => {
                let cond_val = self
                    .generate_expr(&if_stmt.cond)
                    .unwrap_or(TacValue::IntLit(0));
                let then_label = self.new_label();
                let else_label = self.new_label();
                let end_label = self.new_label();

                self.emit(TacInstr::CondJump(
                    cond_val,
                    then_label.clone(),
                    else_label.clone(),
                ));

                // Generate then block
                self.start_block(then_label);
                for stmt in &if_stmt.if_block {
                    self.generate_stmt(stmt);
                }
                self.emit(TacInstr::Jump(end_label.clone()));

                // Generate else block
                self.start_block(else_label);
                for stmt in &if_stmt.else_block {
                    self.generate_stmt(stmt);
                }

                // Continue after if-else
                self.start_block(end_label);
            }
            Stmt::For(for_stmt) => {
                // Generate initialization
                if let Some(init) = &for_stmt.init {
                    if let Some(val) = self.generate_expr(&init.expr) {
                        self.emit(TacInstr::AssignOp(init.ident.clone(), val));
                    }
                }

                let loop_label = self.new_label();
                let body_label = self.new_label();
                let end_label = self.new_label();

                // Push loop context for break statements
                self.push_loop(end_label.clone());

                // Loop condition check
                self.start_block(loop_label.clone());
                let cond_val = self
                    .generate_expr(&for_stmt.cond.expr)
                    .unwrap_or(TacValue::IntLit(1));
                self.emit(TacInstr::CondJump(
                    cond_val,
                    body_label.clone(),
                    end_label.clone(),
                ));

                // Loop body
                self.start_block(body_label);
                for stmt in &for_stmt.block {
                    self.generate_stmt(stmt);
                }

                // Update expression
                self.generate_expr(&for_stmt.updt);
                self.emit(TacInstr::Jump(loop_label));

                // Pop loop context
                self.pop_loop();

                // Continue after loop
                self.start_block(end_label);
            }
            Stmt::Break => {
                // Handle break statement properly
                if let Some(end_label) = self.current_loop_end() {
                    self.emit(TacInstr::Jump(end_label.clone()));
                } else {
                    // Break outside of loop - this could be a compile-time error
                    // For now, emit a comment or warning
                    eprintln!("Warning: break statement outside of loop");
                    self.emit(TacInstr::Nop);
                }
            }
        }
    }
}

/// Convert AST declarations into TAC blocks
pub fn generate_tac_blocks(decls: Vec<Decl>) -> Vec<TacBlock> {
    let mut generator = TacGenerator::new();

    for decl in decls {
        match decl {
            Decl::Fn(fn_decl) => {
                let fn_label = format!("fn_{}", fn_decl.ident);
                generator.start_block(fn_label.clone());
                generator.emit(TacInstr::FnDecl(fn_decl.ident.clone()));

                for stmt in fn_decl.block {
                    generator.generate_stmt(&stmt);
                }
            }
            Decl::Var(var_decl) => {
                if let Some(val) = generator.generate_expr(&var_decl.expr) {
                    generator.emit(TacInstr::AssignOp(var_decl.ident.clone(), val));
                }
            }
        }
    }

    generator.finish();
    generator.blocks
}

/// Consume your Vec<Decl>, find each FnDecl, and recursively split
/// nested blocks (For, If, Ret) into a flat Vec<Block>.
pub fn split_into_blocks(decls: TranslationUnit) -> Vec<Block> {
    let mut block_list = Vec::new();
    
    for decl in decls {
        // TODO: account for global variables
        match decl {
            Decl::Fn(f) => {
                split_stmts(f.block, &mut block_list)
            }

            Decl::Var(_v) => {
                todo!()
            }
        }
    }

    block_list
}

/// Recursively walk a Vec<Stmt>, emitting new Blocks whenever a control statement
/// is encountered, and recursing into nested block fields.
fn split_stmts(parent_block: Block, block_list: &mut Vec<Block>) {
    let mut curr_block: Block = Vec::new();

    for stmt in parent_block {
        match stmt {
            Stmt::For(for_stmt) => {
                // flush curr_block non-control statements
                if !curr_block.is_empty() {
                    block_list.push(std::mem::take(&mut curr_block));
                }

                // emit the loop header as its own block
                block_list.push(vec![Stmt::For(ForStmt {
                    init: for_stmt.init,
                    cond: for_stmt.cond,
                    updt: for_stmt.updt,
                    block: Vec::new(), // drop nested for now
                })]);

                // recurse into the loop body
                split_stmts(for_stmt.block, block_list);
            }

            Stmt::If(if_stmt) => {
                if !curr_block.is_empty() {
                    block_list.push(std::mem::take(&mut curr_block));
                }

                // emit the if condition as its own block
                block_list.push(vec![Stmt::If(IfStmt {
                    cond: if_stmt.cond.clone(),
                    if_block: Vec::new(),
                    else_block: Vec::new(),
                })]);

                // recurse into 'then' block
                split_stmts(if_stmt.if_block, block_list);

                // recurse into 'else' block if present
                if !if_stmt.else_block.is_empty() {
                    split_stmts(if_stmt.else_block, block_list);
                }
            }

            Stmt::Ret(ret_stmt) => {
                if !curr_block.is_empty() {
                    block_list.push(std::mem::take(&mut curr_block));
                }
                
                // emit the return as its own block
                block_list.push(vec![Stmt::Ret(ret_stmt)]);
            }

            Stmt::Break => {
                if !curr_block.is_empty() {
                    block_list.push(std::mem::take(&mut curr_block));
                }
                
                // emit the break as its own block
                block_list.push(vec![Stmt::Break]);
            }

            other => {
                curr_block.push(other);
            }
        }
    }

    if !curr_block.is_empty() {
        block_list.push(curr_block);
    }
}
