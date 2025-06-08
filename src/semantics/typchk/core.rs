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

                let Some(node_id) =
                    symbol_table.get_nth_child_of_type(ROOT_ID, ctr._fn, ScopeType::FnBlock)
                else {
                    unreachable!("couldn't find fn #{} in the root scope", ctr._fn)
                };

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
                // TODO: type-check `cond` (boolean) and `updt`
                ctr._for += 1;
                let Some(for_child_id) =
                    spaghet.get_nth_child_of_type(node_id, ctr._for, ScopeType::ForBlock)
                else {
                    unreachable!("couldn't find for #{} in scope #{}", ctr._for, node_id)
                };
                check_block(spaghet, &f.block, expected_ret_type, for_child_id)?;
            }

            Stmt::If(i) => {
                // TODO: type-check `cond` (boolean)

                for block in [(&i.if_block), (&i.else_block)] {
                    ctr._if += 1;

                    let Some(if_child_id) =
                        spaghet.get_nth_child_of_type(node_id, ctr._if, ScopeType::IfBlock)
                    else {
                        unreachable!("couldn't find if block #{} in scope #{}", ctr._if, node_id)
                    };

                    check_block(spaghet, block, expected_ret_type, if_child_id)?;
                }
            }

            Stmt::Ret(es) => {
                if get_expr_type(spaghet, &es.expr, node_id)? != *expected_ret_type {
                    return Err(TypeChkError::ErroneousReturnType);
                }
            }

            Stmt::VarDecl(v) => check_var_decl(spaghet, v, node_id)?,

            Stmt::Break => {
                // Iterate up to the top and check if we're in a for loop at any point in time
                // TODO: add this as a method of SpaghettiStack
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
