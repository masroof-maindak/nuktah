use super::recurse::{check_for_undeclared_ident, sym_exists};
use crate::lexer::Token;
use crate::parser::ast::core::*;
use crate::semantics::spaghetti::SymInfo;
use crate::semantics::{
    errors::ScopeError,
    spaghetti::{Id, ScopeType, SpaghettiStack, SymType},
};

/// Traverses AST, generating a symbol table (spaghetti stack) as it goes.
pub fn analyse_scope(ast_root: &TranslationUnit) -> Result<SpaghettiStack, ScopeError> {
    let mut spaghet: SpaghettiStack = Default::default();
    let root_id = spaghet.create_scope_map(None, ScopeType::Root);

    for decl in ast_root {
        match decl {
            Decl::Var(v) => {
                insert_var_to_scope(&mut spaghet, root_id, v)?;
            }

            Decl::Fn(f) => {
                insert_fn_to_scope(&mut spaghet, root_id, f)?;
                let fn_table_id = generate_function_scope(&mut spaghet, root_id, f)?;
                spaghet.add_child(root_id, fn_table_id);
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
    let fn_table_id = spaghet.create_scope_map(Some(parent_id), ScopeType::FnBlock);

    for param in fn_node.params.iter() {
        let sym_type = extract_sym_type(&param.type_tok);
        spaghet.insert_ident_in_node(fn_table_id, &param.ident, SymInfo::new(true, sym_type));
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
                spaghet.add_child(curr_id, for_table_id);
            }

            Stmt::If(i) => {
                let if_table_ids = generate_if_scope(spaghet, curr_id, i)?;
                spaghet.add_child(curr_id, if_table_ids.0);
                spaghet.add_child(curr_id, if_table_ids.1);
            }

            Stmt::Expr(es) | Stmt::Ret(es) => {
                check_for_undeclared_ident(spaghet, curr_id, &es.expr)?;
            }

            Stmt::VarDecl(d) => {
                insert_var_to_scope(spaghet, curr_id, d)?;
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
    let for_table_id = spaghet.create_scope_map(Some(parent_id), ScopeType::ForBlock);

    if for_node.init.is_some() {
        insert_var_to_scope(spaghet, for_table_id, for_node.init.as_ref().unwrap())?;
    }

    // New variables could have been allocated in this for's scope
    check_for_undeclared_ident(spaghet, for_table_id, &for_node.cond.expr)?;
    check_for_undeclared_ident(spaghet, for_table_id, &for_node.updt)?;

    analyse_block_scope(spaghet, for_table_id, &for_node.block)?;

    Ok(for_table_id)
}

fn generate_if_scope(
    spaghet: &mut SpaghettiStack,
    parent_id: Id,
    if_node: &IfStmt,
) -> Result<(Id, Id), ScopeError> {
    let if_table_id = spaghet.create_scope_map(Some(parent_id), ScopeType::IfBlock);
    let else_table_id = spaghet.create_scope_map(Some(parent_id), ScopeType::IfBlock);

    // New variables can not be declared in this if's condition
    check_for_undeclared_ident(spaghet, parent_id, &if_node.cond)?;

    analyse_block_scope(spaghet, if_table_id, &if_node.if_block)?;
    analyse_block_scope(spaghet, else_table_id, &if_node.else_block)?;

    Ok((if_table_id, else_table_id))
}

fn insert_var_to_scope(
    spaghet: &mut SpaghettiStack,
    scope_map_id: Id,
    v: &VarDecl,
) -> Result<(), ScopeError> {
    if sym_exists(spaghet, scope_map_id, &v.ident) {
        return Err(ScopeError::VariableRedefinition);
    }

    check_for_undeclared_ident(spaghet, scope_map_id, &v.expr)?;

    let sym_type = extract_sym_type(&v.type_tok);
    spaghet.insert_ident_in_node(scope_map_id, &v.ident, SymInfo::new(true, sym_type));
    Ok(())
}

fn insert_fn_to_scope(
    spaghet: &mut SpaghettiStack,
    scope_map_id: Id,
    f: &FnDecl,
) -> Result<(), ScopeError> {
    if sym_exists(spaghet, scope_map_id, &f.ident) {
        return Err(ScopeError::VariableRedefinition);
    }

    let sym_type = extract_sym_type(&f.type_tok);
    spaghet.insert_ident_in_node(scope_map_id, &f.ident, SymInfo::new(false, sym_type));
    Ok(())
}

fn extract_sym_type(type_tok: &Token) -> SymType {
    match type_tok {
        Token::Int => SymType::Int,
        Token::String => SymType::String,
        Token::Float => SymType::Float,
        _ => unreachable!("type token didn't contain a type..."),
    }
}
