use crate::parser::ast::core::*;
use crate::semantics::{
    errors::ScopeError,
    spaghetti::{Id, SpaghettiStack},
};

// if expr breaks down to PrimaryExpr::Ident, ensure that the identifier in question was previously saved to the symbol table.

pub fn check_for_undeclared_ident(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &Expr,
) -> Result<(), ScopeError> {
    let Some(assign_expr) = expr else {
        return Ok(());
    };

    match assign_expr {
        AssignExpr::Bool(bool_expr) => check_bool_expr(spaghet, curr_id, bool_expr)?,
        AssignExpr::Assign(bool_expr, nested_assign_expr) => {
            check_bool_expr(spaghet, curr_id, bool_expr)?;
            // What the literal FUCK
            let new_expr = &Some(nested_assign_expr.as_ref().clone());
            check_for_undeclared_ident(spaghet, curr_id, new_expr)?;
        }
    }

    Ok(())
}

fn check_bool_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &BoolExpr,
) -> Result<(), ScopeError> {
    match expr {
        BoolExpr::BitOr(bit_or) => check_bit_or_expr(spaghet, curr_id, bit_or)?,
        BoolExpr::Bool(bool_expr, _, bit_or) => {
            check_bool_expr(spaghet, curr_id, bool_expr)?;
            check_bit_or_expr(spaghet, curr_id, bit_or)?;
        }
    }
    Ok(())
}

fn check_bit_or_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &BitOrExpr,
) -> Result<(), ScopeError> {
    match expr {
        BitOrExpr::BitAnd(bit_and) => check_bit_and_expr(spaghet, curr_id, bit_and)?,
        BitOrExpr::BitOr(bit_or, bit_and) => {
            check_bit_or_expr(spaghet, curr_id, bit_or)?;
            check_bit_and_expr(spaghet, curr_id, bit_and)?;
        }
    }
    Ok(())
}

fn check_bit_and_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &BitAndExpr,
) -> Result<(), ScopeError> {
    match expr {
        BitAndExpr::Comp(comp) => check_comp_expr(spaghet, curr_id, comp)?,
        BitAndExpr::BitAnd(bit_and, comp) => {
            check_bit_and_expr(spaghet, curr_id, bit_and)?;
            check_comp_expr(spaghet, curr_id, comp)?;
        }
    }
    Ok(())
}

fn check_comp_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &CompExpr,
) -> Result<(), ScopeError> {
    match expr {
        CompExpr::Shift(shift) => check_shift_expr(spaghet, curr_id, shift)?,
        CompExpr::Comp(comp, _, shift) => {
            check_comp_expr(spaghet, curr_id, comp)?;
            check_shift_expr(spaghet, curr_id, shift)?;
        }
    }
    Ok(())
}

fn check_shift_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &ShiftExpr,
) -> Result<(), ScopeError> {
    match expr {
        ShiftExpr::Add(add) => check_add_expr(spaghet, curr_id, add)?,
        ShiftExpr::Shift(shift, _, add) => {
            check_shift_expr(spaghet, curr_id, shift)?;
            check_add_expr(spaghet, curr_id, add)?;
        }
    }
    Ok(())
}

fn check_add_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &AddExpr) -> Result<(), ScopeError> {
    match expr {
        AddExpr::Mul(mul) => check_mul_expr(spaghet, curr_id, mul)?,
        AddExpr::Add(add, _, mul) => {
            check_add_expr(spaghet, curr_id, add)?;
            check_mul_expr(spaghet, curr_id, mul)?;
        }
    }
    Ok(())
}

fn check_mul_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &MulExpr) -> Result<(), ScopeError> {
    match expr {
        MulExpr::Exp(exp) => check_exp_expr(spaghet, curr_id, exp)?,
        MulExpr::Mul(mul, _, exp) => {
            check_mul_expr(spaghet, curr_id, mul)?;
            check_exp_expr(spaghet, curr_id, exp)?;
        }
    }
    Ok(())
}

fn check_exp_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &ExpExpr) -> Result<(), ScopeError> {
    match expr {
        ExpExpr::Unary(unary) => check_unary_expr(spaghet, curr_id, unary)?,
        ExpExpr::Exp(unary, exp) => {
            check_unary_expr(spaghet, curr_id, unary)?;
            check_exp_expr(spaghet, curr_id, exp)?;
        }
    }
    Ok(())
}

fn check_unary_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &UnaryExpr,
) -> Result<(), ScopeError> {
    match expr {
        UnaryExpr::Primary(primary) => check_primary_expr(spaghet, curr_id, primary)?,
        UnaryExpr::Unary(_, unary) => check_unary_expr(spaghet, curr_id, unary)?,
    }
    Ok(())
}

fn check_primary_expr(
    spaghet: &SpaghettiStack,
    curr_id: Id,
    expr: &PrimaryExpr,
) -> Result<(), ScopeError> {
    match expr {
        PrimaryExpr::Ident(ident) => {
            if !sym_exists(spaghet, curr_id, ident) {
                return Err(ScopeError::UndeclaredIdentifierUsed);
            }
        }

        PrimaryExpr::Paren(nested_expr) => {
            check_for_undeclared_ident(spaghet, curr_id, nested_expr)?
        }

        PrimaryExpr::Call(fn_call) => {
            // Check if function exists (function definitions are bound to be in the root scope)
            if !spaghet.does_identifier_exist(0, &fn_call.ident) {
                return Err(ScopeError::UndeclaredIdentifierUsed);
            }

            // Check all arguments
            for arg in &fn_call.args {
                check_for_undeclared_ident(spaghet, curr_id, arg)?;
            }
        }
        // Other literals don't need checking
        PrimaryExpr::IntLit(_) | PrimaryExpr::FloatLit(_) | PrimaryExpr::StringLit(_) => {}
    }
    Ok(())
}

pub fn sym_exists(spaghet: &SpaghettiStack, curr_id: Id, ident: &str) -> bool {
    let mut node_id: Option<Id> = Some(curr_id);

    while node_id.is_some() {
        if spaghet.does_identifier_exist(node_id.unwrap(), ident) {
            return true;
        }

        node_id = spaghet.get_node_parent_id(node_id.unwrap());
    }

    false
}
