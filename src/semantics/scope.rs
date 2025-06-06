use crate::lexer::Token;
use crate::parser::ast::core::*;
use crate::semantics::spaghetti::{ScopeMap, SymType};

#[derive(Debug)]
pub enum ScopeError {
    UndeclaredVariableUsed,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

/// Traverses AST, generating a symbol table (spaghetti stack) as it goes.
pub fn analyse_scope(ast_root: &TranslationUnit) -> Result<ScopeMap, ScopeError> {
    let mut sym_table_root = ScopeMap::new();

    for decl in ast_root {
        match decl {
            Decl::Var(d) => {
                insert_var_to_scope(&mut sym_table_root, d)?;
            }

            Decl::Fn(f) => {
                let fn_scope_map = analyse_fn_scope(&sym_table_root, f)?;
                sym_table_root.insert_child(fn_scope_map);
            }
        }
    }

    Ok(sym_table_root)
}

fn analyse_fn_scope(_parent: &ScopeMap, fn_node: &FnDecl) -> Result<ScopeMap, ScopeError> {
    let mut sym_table = ScopeMap::new();

    for param in fn_node.params.iter() {
        let sym_type = extract_sym_type(&param.t);
        // NOTE: Function parameters will override other global identifiers.
        sym_table.insert_val(&param.ident, sym_type);
    }

    analyse_block_scope(&mut sym_table, &fn_node.block)?;

    Ok(sym_table)
}

fn analyse_block_scope(sym_table: &mut ScopeMap, block: &Block) -> Result<(), ScopeError> {
    for stmt in block.iter() {
        match stmt {
            Stmt::For(f) => {
                let for_sym_table = analyse_for_scope(sym_table, f)?;
                sym_table.insert_child(for_sym_table);
            }

            Stmt::If(i) => {
                let sym_tables = analyse_if_scope(sym_table, i)?;
                sym_table.insert_child(sym_tables.0);
                sym_table.insert_child(sym_tables.1);
            }

            Stmt::Expr(es) | Stmt::Ret(es) => {
                check_expr_ident_exists(sym_table, &es.e)?;
            }

            Stmt::VarDecl(d) => {
                insert_var_to_scope(sym_table, d)?;
            }

            Stmt::Break => {} // ignore
        }
    }

    Ok(())
}

fn analyse_for_scope(parent: &ScopeMap, for_node: &ForStmt) -> Result<ScopeMap, ScopeError> {
    let mut sym_table = ScopeMap::new(); // TODO: use parent in constructor

    if for_node.init.is_some() {
        insert_var_to_scope(&mut sym_table, for_node.init.as_ref().unwrap())?;
    }

    check_expr_ident_exists(parent, &for_node.cond.e)?;
    check_expr_ident_exists(parent, &for_node.updt)?;

    analyse_block_scope(&mut sym_table, &for_node.block)?;

    Ok(sym_table)
}

fn analyse_if_scope(
    parent: &ScopeMap,
    if_node: &IfStmt,
) -> Result<(ScopeMap, ScopeMap), ScopeError> {
    let mut if_sym_table = ScopeMap::new(); // TODO: use parent in constructor
    let mut else_sym_table = ScopeMap::new(); // TODO: use parent in constructor

    check_expr_ident_exists(parent, &if_node.cond)?;

    analyse_block_scope(&mut if_sym_table, &if_node.if_block)?;
    analyse_block_scope(&mut else_sym_table, &if_node.else_block)?;

    Ok((if_sym_table, else_sym_table))
}

// if expr breaks down to PrimaryExpr::Ident, ensure that the identifier in question has been saved
fn check_expr_ident_exists(sym_table: &ScopeMap, expr: &Expr) -> Result<(), ScopeError> {
    if let Some(AssignExpr::Bool(BoolExpr::BitOr(BitOrExpr::BitAnd(BitAndExpr::Comp(
        CompExpr::Shift(ShiftExpr::Add(AddExpr::Mul(MulExpr::Exp(ExpExpr::Unary(
            UnaryExpr::Primary(PrimaryExpr::Ident(ident)),
        ))))),
    ))))) = expr
    {
        if !sym_exists(sym_table, ident) {
            return Err(ScopeError::UndeclaredVariableUsed);
        }
    }

    Ok(())
}

fn sym_exists(_sym_table: &ScopeMap, _ident: &str) -> bool {
    // TODO: iterate up scope maps until parent becomes None and return true if ident is found at any point
    // ScopeMap
    false
}

fn insert_var_to_scope(sym_table: &mut ScopeMap, d: &VarDecl) -> Result<(), ScopeError> {
    if sym_exists(sym_table, &d.ident) {
        return Err(ScopeError::VariableRedefinition);
    }

    let sym_type = extract_sym_type(&d.t);
    sym_table.insert_val(&d.ident, sym_type);
    Ok(())
}

fn extract_sym_type(token_type: &Token) -> SymType {
    match token_type {
        Token::Int => SymType::Int,
        Token::String => SymType::String,
        Token::Float => SymType::Float,
        _ => unreachable!("parser::consume_prim_type_tok failed..."),
    }
}
