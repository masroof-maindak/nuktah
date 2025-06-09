use super::recurse::get_expr_type;
use crate::{
    parser::ast::core::*,
    semantics::{
        errors::TypeChkError,
        spaghetti::{Id, ScopeType, SpaghettiStack, SymType},
        utils::token_to_symtype,
    },
};

const ROOT_ID: Id = 0;

/// For a given scope of Id `n`, how many nested scopes of it have we encountered?
struct ScopeTypeCounter {
    _fn: usize,
    _for: usize,
    _if: usize,
}

impl ScopeTypeCounter {
    fn new() -> ScopeTypeCounter {
        ScopeTypeCounter {
            _fn: 0,
            _for: 0,
            _if: 0,
        }
    }
}

pub fn check_types(
    ast_root: &TranslationUnit,
    symbol_table: &SpaghettiStack,
) -> Result<(), TypeChkError> {
    let mut ctr = ScopeTypeCounter::new();

    for decl in ast_root {
        match decl {
            Decl::Var(v) => {
                check_var_decl(symbol_table, v, ROOT_ID)?;
            }

            Decl::Fn(f) => {
                ctr._fn += 1;
                let node_id =
                    get_nth_child_of_type(symbol_table, ROOT_ID, ctr._fn, ScopeType::FnBlock);
                check_fn_decl(symbol_table, f, node_id)?;
            }
        }
    }

    Ok(())
}

fn check_var_decl(spaghet: &SpaghettiStack, v: &VarDecl, node_id: Id) -> Result<(), TypeChkError> {
    let expr_type = get_expr_type(spaghet, &v.expr, node_id)?;
    let var_type = token_to_symtype(&v.type_tok, true);

    if var_type != expr_type {
        return Err(TypeChkError::ErroneousVarDecl);
    }

    Ok(())
}

fn check_fn_decl(spaghet: &SpaghettiStack, f: &FnDecl, node_id: Id) -> Result<(), TypeChkError> {
    let ret_type = token_to_symtype(&f.type_tok, false);
    check_block(spaghet, &f.block, &ret_type, node_id)?;
    Ok(())
}

fn check_block(
    spaghet: &SpaghettiStack,
    block: &Block,
    expected_ret_type: &SymType,
    node_id: Id,
) -> Result<(), TypeChkError> {
    let mut ctr = ScopeTypeCounter::new();
    for stmt in block {
        match stmt {
            Stmt::For(f) => {
                if get_expr_type(spaghet, &f.cond.expr, node_id)? != SymType::Bool {
                    return Err(TypeChkError::NonBooleanCondStmt);
                }

                get_expr_type(spaghet, &f.updt, node_id)?;

                ctr._for += 1;
                let for_child_id =
                    get_nth_child_of_type(spaghet, node_id, ctr._for, ScopeType::ForBlock);
                check_block(spaghet, &f.block, expected_ret_type, for_child_id)?;
            }

            Stmt::If(i) => {
                if get_expr_type(spaghet, &i.cond, node_id)? != SymType::Bool {
                    return Err(TypeChkError::NonBooleanCondStmt);
                }

                for block in [(&i.if_block), (&i.else_block)] {
                    ctr._if += 1;
                    let if_child_id =
                        get_nth_child_of_type(spaghet, node_id, ctr._if, ScopeType::IfBlock);
                    check_block(spaghet, block, expected_ret_type, if_child_id)?;
                }
            }

            Stmt::Ret(r) => {
                if get_expr_type(spaghet, &r.expr, node_id)? != *expected_ret_type {
                    return Err(TypeChkError::ErroneousReturnType);
                }
            }

            Stmt::VarDecl(v) => check_var_decl(spaghet, v, node_id)?,

            Stmt::Break => {
                // Iterate up to the top and check if we're in a for loop at any point in time
                let mut curr_id: Option<Id> = Some(node_id);
                while curr_id.is_some() {
                    match spaghet.get_scope_type(curr_id.unwrap()) {
                        ScopeType::Root => return Err(TypeChkError::ErroneousBreak),
                        ScopeType::ForBlock => break,
                        _ => curr_id = spaghet.get_node_parent_id(curr_id.unwrap()),
                    }
                }
            }

            Stmt::Expr(_) => {} // Empty expressions are kind of useless here
        }
    }
    Ok(())
}

fn get_nth_child_of_type(
    symbol_table: &SpaghettiStack,
    node_id: Id,
    ctr: usize,
    scope_type: ScopeType,
) -> Id {
    let Some(node_id) = symbol_table.get_nth_child_of_type(node_id, ctr, scope_type.clone()) else {
        unreachable!("couldn't find {:?} #{} in the root scope", scope_type, ctr)
    };

    node_id
}
