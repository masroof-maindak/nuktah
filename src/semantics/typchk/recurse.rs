use crate::{
    lexer::Token,
    parser::ast::core::*,
    semantics::{
        errors::TypeChkError,
        spaghetti::{Id, SpaghettiStack, SymInfo, SymType},
        utils::find_info_in_table,
    },
};

/// Get the type of the expression `expr`, encountered in node (ScopeMap) w/ Id `node_id`
pub fn get_expr_type(
    spaghet: &SpaghettiStack,
    expr: &Expr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    let Some(assign_expr) = expr else {
        return Ok(SymType::Void);
    };

    match assign_expr {
        AssignExpr::Bool(bool_e) => get_bool_expr_type(spaghet, bool_e, node_id),

        AssignExpr::Assign(bool_e, nested_assign) => {
            let lhs_type = get_bool_expr_type(spaghet, bool_e, node_id)?;
            let new_expr = &Some(nested_assign.as_ref().clone());
            let rhs_type = get_expr_type(spaghet, new_expr, node_id)?;

            // NOTE: type conversion rules would apply here, had I chosen to support them in
            // Nuktah. Probably via a call to another function that validates if the RHS type can
            // safely be converted to the LHS type.

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            Ok(lhs_type)
        }
    }
}

fn get_bool_expr_type(
    spaghet: &SpaghettiStack,
    expr: &BoolExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        BoolExpr::BitOr(bit_or_e) => get_bit_or_expr_type(spaghet, bit_or_e, node_id),

        BoolExpr::Bool(bool_e, _, bit_or_e) => {
            if get_bool_expr_type(spaghet, bool_e, node_id)? != SymType::Bool
                || get_bit_or_expr_type(spaghet, bit_or_e, node_id)? != SymType::Bool
            {
                return Err(TypeChkError::AttemptedBoolOpOnNonBools);
            }

            Ok(SymType::Bool)
        }
    }
}

fn get_bit_or_expr_type(
    spaghet: &SpaghettiStack,
    expr: &BitOrExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        BitOrExpr::BitAnd(bit_and_e) => get_bit_and_expr_type(spaghet, bit_and_e, node_id),
        BitOrExpr::BitOr(bit_or_e, bit_and_e) => {
            let lhs_type = get_bit_and_expr_type(spaghet, bit_and_e, node_id)?;
            let rhs_type = get_bit_or_expr_type(spaghet, bit_or_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if ![SymType::Int, SymType::Float].contains(&lhs_type) {
                return Err(TypeChkError::AttemptedBitOpOnNonNumeric);
            }

            Ok(lhs_type)
        }
    }
}

fn get_bit_and_expr_type(
    spaghet: &SpaghettiStack,
    expr: &BitAndExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        BitAndExpr::Comp(comp_e) => get_comp_expr_type(spaghet, comp_e, node_id),
        BitAndExpr::BitAnd(bit_and_e, comp_e) => {
            let lhs_type = get_comp_expr_type(spaghet, comp_e, node_id)?;
            let rhs_type = get_bit_and_expr_type(spaghet, bit_and_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if ![SymType::Int, SymType::Float].contains(&lhs_type) {
                return Err(TypeChkError::AttemptedBitOpOnNonNumeric);
            }

            Ok(lhs_type)
        }
    }
}

fn get_comp_expr_type(
    spaghet: &SpaghettiStack,
    expr: &CompExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        CompExpr::Shift(shift_e) => get_shift_expr_type(spaghet, shift_e, node_id),
        CompExpr::Comp(comp_e, _, shift_e) => {
            let lhs_type = get_shift_expr_type(spaghet, shift_e, node_id)?;
            let rhs_type = get_comp_expr_type(spaghet, comp_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            // Comparison always produces a bool
            Ok(SymType::Bool)
        }
    }
}

fn get_shift_expr_type(
    spaghet: &SpaghettiStack,
    expr: &ShiftExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        ShiftExpr::Add(add_e) => get_add_expr_type(spaghet, add_e, node_id),
        ShiftExpr::Shift(shift_e, _, add_e) => {
            let lhs_type = get_add_expr_type(spaghet, add_e, node_id)?;
            let rhs_type = get_shift_expr_type(spaghet, shift_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if lhs_type != SymType::Int {
                return Err(TypeChkError::AttemptedShiftOnNonInt);
            }

            Ok(lhs_type)
        }
    }
}

fn get_add_expr_type(
    spaghet: &SpaghettiStack,
    expr: &AddExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        AddExpr::Mul(mul_e) => get_mul_expr_type(spaghet, mul_e, node_id),
        AddExpr::Add(add_e, _, mul_e) => {
            let lhs_type = get_mul_expr_type(spaghet, mul_e, node_id)?;
            let rhs_type = get_add_expr_type(spaghet, add_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if ![SymType::Int, SymType::Float].contains(&lhs_type) {
                return Err(TypeChkError::AttemptedAddOpOnNonNumeric);
            }

            Ok(lhs_type)
        }
    }
}

fn get_mul_expr_type(
    spaghet: &SpaghettiStack,
    expr: &MulExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        MulExpr::Exp(exp_e) => get_exp_expr_type(spaghet, exp_e, node_id),
        MulExpr::Mul(mul_e, _, exp_e) => {
            let lhs_type = get_exp_expr_type(spaghet, exp_e, node_id)?;
            let rhs_type = get_mul_expr_type(spaghet, mul_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if ![SymType::Int, SymType::Float].contains(&lhs_type) {
                return Err(TypeChkError::AttemptedBitOpOnNonNumeric);
            }

            Ok(lhs_type)
        }
    }
}

fn get_exp_expr_type(
    spaghet: &SpaghettiStack,
    expr: &ExpExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        ExpExpr::Unary(unary_e) => get_unary_expr_type(spaghet, unary_e, node_id),
        ExpExpr::Exp(unary_e, exp_e) => {
            let lhs_type = get_unary_expr_type(spaghet, unary_e, node_id)?;
            let rhs_type = get_exp_expr_type(spaghet, exp_e, node_id)?;

            if lhs_type != rhs_type {
                return Err(TypeChkError::ExpressionTypeMismatch);
            }

            if ![SymType::Int, SymType::Float].contains(&lhs_type) {
                return Err(TypeChkError::AttemptedExponentiationOfNonNumeric);
            }

            Ok(lhs_type)
        }
    }
}

fn get_unary_expr_type(
    spaghet: &SpaghettiStack,
    expr: &UnaryExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        UnaryExpr::Primary(primary_e) => get_primary_expr_type(spaghet, primary_e, node_id),

        UnaryExpr::Unary(tok, unary_e) => {
            let primary_e_type = get_unary_expr_type(spaghet, unary_e, node_id)?;

            match tok {
                Token::SubOp => {
                    if ![SymType::Int, SymType::Float].contains(&primary_e_type) {
                        return Err(TypeChkError::AttemptedAddOpOnNonNumeric);
                    }
                }

                Token::BooleanNot | Token::BitwiseNot => {
                    if primary_e_type != SymType::Bool {
                        return Err(TypeChkError::AttemptedBitOpOnNonNumeric);
                    }
                }

                _ => unreachable!("unexpected operation's token in unary expression: {tok:?}"),
            }

            Ok(primary_e_type)
        }
    }
}

fn get_primary_expr_type(
    spaghet: &SpaghettiStack,
    expr: &PrimaryExpr,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    match expr {
        PrimaryExpr::Ident(ident) => {
            let sym_info = fetch_guaranteed_info_from_table(spaghet, ident, node_id, true);
            Ok(sym_info.get_type())
        }
        PrimaryExpr::Paren(nested_e) => get_expr_type(spaghet, nested_e, node_id),
        PrimaryExpr::Call(fn_call) => get_fn_call_type(spaghet, fn_call, node_id),
        PrimaryExpr::IntLit(_) => Ok(SymType::Int),
        PrimaryExpr::FloatLit(_) => Ok(SymType::Float),
        PrimaryExpr::StringLit(_) => Ok(SymType::String),
        PrimaryExpr::BoolLit(_) => Ok(SymType::Bool),
    }
}

fn get_fn_call_type(
    spaghet: &SpaghettiStack,
    fn_call: &FnCall,
    node_id: Id,
) -> Result<SymType, TypeChkError> {
    let fn_type = fetch_guaranteed_info_from_table(spaghet, &fn_call.ident, node_id, false);
    let param_types = spaghet.get_fn_param_types(&fn_call.ident);

    if param_types.len() != fn_call.args.len() {
        return Err(TypeChkError::FnCallParamCount);
    }

    for (i, arg) in fn_call.args.iter().enumerate() {
        if arg.is_none() {
            unreachable!("argument to function is of type None");
        }

        if get_expr_type(spaghet, arg, node_id)? != param_types[i] {
            return Err(TypeChkError::FnCallParamType);
        }
    }

    Ok(fn_type.get_type())
}

fn fetch_guaranteed_info_from_table(
    spaghet: &SpaghettiStack,
    ident: &str,
    node_id: Id,
    is_var: bool,
) -> SymInfo {
    let Some(info) = find_info_in_table(spaghet, node_id, ident, is_var) else {
        unreachable!("identifier `{ident}` doesn't exist in symbol table");
    };

    info
}
