use crate::lexer::Token;
use crate::parser::ast::core::*;
use crate::semantics::spaghetti::{Id, SpaghettiStack, SymType};

#[derive(Debug)]
pub enum ScopeError {
    UndeclaredVariableUsed,
    VariableRedefinition,
    FunctionPrototypeRedefinition,
}

/// Traverses AST, generating a symbol table (spaghetti stack) as it goes.
pub fn analyse_scope(ast_root: &TranslationUnit) -> Result<SpaghettiStack, ScopeError> {
    let mut spaghet = SpaghettiStack::new();
    let root_id = spaghet.create_scope_map(None);

    for decl in ast_root {
        match decl {
            Decl::Var(d) => {
                insert_var_in_scope(&mut spaghet, root_id, d)?;
            }

            Decl::Fn(f) => {
                let fn_table_id = generate_function_scope(&mut spaghet, root_id, f)?;
                spaghet.add_node_as_child_of(root_id, fn_table_id);
            }
        }
    }

    Ok(spaghet)
}

/// Analyse a function for scope discrepancies, populating a new symbol table for it
/// NOTE: Function parameters will override other global identifiers.
fn generate_function_scope(
    spaghet: &mut SpaghettiStack,
    parent_id: Id,
    fn_node: &FnDecl,
) -> Result<Id, ScopeError> {
    let fn_table_id = spaghet.create_scope_map(Some(parent_id));

    for param in fn_node.params.iter() {
        let sym_type = extract_sym_type(&param.t);
        spaghet.insert_var_in_node(fn_table_id, &param.ident, sym_type);
    }

    analyse_block_scope(spaghet, fn_table_id, &fn_node.block)?;

    Ok(fn_table_id)
}

fn analyse_block_scope(
    spaghet: &mut SpaghettiStack,
    curr_id: Id,
    block: &Block,
) -> Result<(), ScopeError> {
    for stmt in block.iter() {
        match stmt {
            Stmt::For(f) => {
                let for_table_id = generate_for_scope(spaghet, curr_id, f)?;
                spaghet.add_node_as_child_of(curr_id, for_table_id);
            }

            Stmt::If(i) => {
                let if_table_ids = generate_if_scope(spaghet, curr_id, i)?;
                spaghet.add_node_as_child_of(curr_id, if_table_ids.0);
                spaghet.add_node_as_child_of(curr_id, if_table_ids.1);
            }

            Stmt::Expr(es) | Stmt::Ret(es) => {
                check_expr_ident_exists(spaghet, curr_id, &es.e)?;
            }

            Stmt::VarDecl(d) => {
                insert_var_in_scope(spaghet, curr_id, d)?;
            }

            Stmt::Break => {} // ignore
        }
    }

    Ok(())
}

fn generate_for_scope(
    spaghet: &mut SpaghettiStack,
    parent_id: Id,
    for_node: &ForStmt,
) -> Result<Id, ScopeError> {
    let for_table_id = spaghet.create_scope_map(Some(parent_id));

    if for_node.init.is_some() {
        insert_var_in_scope(spaghet, for_table_id, for_node.init.as_ref().unwrap())?;
    }

    check_expr_ident_exists(spaghet, parent_id, &for_node.cond.e)?;
    check_expr_ident_exists(spaghet, parent_id, &for_node.updt)?;

    analyse_block_scope(spaghet, for_table_id, &for_node.block)?;

    Ok(for_table_id)
}

fn generate_if_scope(
    spaghet: &mut SpaghettiStack,
    parent_id: Id,
    if_node: &IfStmt,
) -> Result<(Id, Id), ScopeError> {
    let if_table_id = spaghet.create_scope_map(Some(parent_id));
    let else_table_id = spaghet.create_scope_map(Some(parent_id));

    check_expr_ident_exists(spaghet, parent_id, &if_node.cond)?;

    analyse_block_scope(spaghet, if_table_id, &if_node.if_block)?;
    analyse_block_scope(spaghet, else_table_id, &if_node.else_block)?;

    Ok((if_table_id, else_table_id))
}

// if expr breaks down to PrimaryExpr::Ident, ensure that the identifier in question has been saved
fn check_expr_ident_exists(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &Expr,
) -> Result<(), ScopeError> {
    if let Some(AssignExpr::Bool(BoolExpr::BitOr(BitOrExpr::BitAnd(BitAndExpr::Comp(
        CompExpr::Shift(ShiftExpr::Add(AddExpr::Mul(MulExpr::Exp(ExpExpr::Unary(
            UnaryExpr::Primary(PrimaryExpr::Ident(ident)),
        ))))),
    ))))) = expr
    {
        if !sym_exists(spaghet, curr_id, ident) {
            return Err(ScopeError::UndeclaredVariableUsed);
        }
    }

    Ok(())
}

fn sym_exists(spaghet: &SpaghettiStack, curr_id: Id, ident: &str) -> bool {
    let mut node_id: Option<Id> = Some(curr_id);

    while node_id.is_some() {
        if spaghet.check_if_var_exists(node_id.unwrap(), ident) {
            return true;
        } else {
            node_id = spaghet.get_node_parent_id(node_id.unwrap());
        }
    }

    false
}

fn insert_var_in_scope(
    spaghet: &mut SpaghettiStack,
    scope_map_id: Id,
    d: &VarDecl,
) -> Result<(), ScopeError> {
    if sym_exists(spaghet, scope_map_id, &d.ident) {
        return Err(ScopeError::VariableRedefinition);
    }

    let sym_type = extract_sym_type(&d.t);
    spaghet.insert_var_in_node(scope_map_id, &d.ident, sym_type);
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
