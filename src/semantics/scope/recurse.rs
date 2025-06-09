use crate::parser::ast::core::*;
use crate::semantics::utils::find_info_in_table;
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
    let Some(assign_e) = expr else {
        return Ok(());
    };

    match assign_e {
        AssignExpr::Bool(bool_e) => check_bool_expr(spaghet, curr_id, bool_e)?,
        AssignExpr::Assign(bool_e, nested_a) => {
            check_bool_expr(spaghet, curr_id, bool_e)?;
            // What the literal FUCK
            let new_expr = &Some(nested_a.as_ref().clone());
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
        BoolExpr::BitOr(bit_or_e) => check_bit_or_expr(spaghet, curr_id, bit_or_e)?,
        BoolExpr::Bool(bool_e, _, bit_or_e) => {
            check_bool_expr(spaghet, curr_id, bool_e)?;
            check_bit_or_expr(spaghet, curr_id, bit_or_e)?;
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
        BitOrExpr::BitAnd(bit_and_e) => check_bit_and_expr(spaghet, curr_id, bit_and_e)?,
        BitOrExpr::BitOr(bit_or_e, bit_and_e) => {
            check_bit_or_expr(spaghet, curr_id, bit_or_e)?;
            check_bit_and_expr(spaghet, curr_id, bit_and_e)?;
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
        BitAndExpr::Comp(comp_e) => check_comp_expr(spaghet, curr_id, comp_e)?,
        BitAndExpr::BitAnd(bit_and_e, comp_e) => {
            check_bit_and_expr(spaghet, curr_id, bit_and_e)?;
            check_comp_expr(spaghet, curr_id, comp_e)?;
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
        CompExpr::Shift(shift_e) => check_shift_expr(spaghet, curr_id, shift_e)?,
        CompExpr::Comp(comp_e, _, shift_e) => {
            check_comp_expr(spaghet, curr_id, comp_e)?;
            check_shift_expr(spaghet, curr_id, shift_e)?;
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
        ShiftExpr::Add(add_e) => check_add_expr(spaghet, curr_id, add_e)?,
        ShiftExpr::Shift(shift_e, _, add_e) => {
            check_shift_expr(spaghet, curr_id, shift_e)?;
            check_add_expr(spaghet, curr_id, add_e)?;
        }
    }
    Ok(())
}

fn check_add_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &AddExpr) -> Result<(), ScopeError> {
    match expr {
        AddExpr::Mul(mul) => check_mul_expr(spaghet, curr_id, mul)?,
        AddExpr::Add(add_e, _, mul_e) => {
            check_add_expr(spaghet, curr_id, add_e)?;
            check_mul_expr(spaghet, curr_id, mul_e)?;
        }
    }
    Ok(())
}

fn check_mul_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &MulExpr) -> Result<(), ScopeError> {
    match expr {
        MulExpr::Exp(exp_e) => check_exp_expr(spaghet, curr_id, exp_e)?,
        MulExpr::Mul(mul_e, _, exp_e) => {
            check_mul_expr(spaghet, curr_id, mul_e)?;
            check_exp_expr(spaghet, curr_id, exp_e)?;
        }
    }
    Ok(())
}

fn check_exp_expr(spaghet: &SpaghettiStack, curr_id: Id, expr: &ExpExpr) -> Result<(), ScopeError> {
    match expr {
        ExpExpr::Unary(unary_e) => check_unary_expr(spaghet, curr_id, unary_e)?,
        ExpExpr::Exp(unary_e, exp_e) => {
            check_unary_expr(spaghet, curr_id, unary_e)?;
            check_exp_expr(spaghet, curr_id, exp_e)?;
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
        UnaryExpr::Primary(primary_e) => check_primary_expr(spaghet, curr_id, primary_e)?,
        UnaryExpr::Unary(_, unary_e) => check_unary_expr(spaghet, curr_id, unary_e)?,
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
            if find_info_in_table(spaghet, curr_id, ident, true).is_none() {
                return Err(ScopeError::UndeclaredVariableCalled);
            }
        }

        PrimaryExpr::Paren(nested_e) => check_for_undeclared_ident(spaghet, curr_id, nested_e)?,

        PrimaryExpr::Call(fn_call) => {
            if find_info_in_table(spaghet, curr_id, &fn_call.ident, false).is_none() {
                return Err(ScopeError::UndefinedFunctionCalled);
            }

            // Check all arguments
            for arg in &fn_call.args {
                check_for_undeclared_ident(spaghet, curr_id, arg)?;
            }
        }

        // Other literals don't need checking
        PrimaryExpr::IntLit(_)
        | PrimaryExpr::FloatLit(_)
        | PrimaryExpr::StringLit(_)
        | PrimaryExpr::BoolLit(_) => {}
    }

    Ok(())
}
