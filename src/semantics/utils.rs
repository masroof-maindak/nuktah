use super::spaghetti::{Id, SpaghettiStack, SymInfo, SymType};
use crate::lexer::Token;

/// Convert's a Token to a SymType
pub fn token_to_symtype(type_tok: &Token, is_var: bool) -> SymType {
    match type_tok {
        Token::Int => SymType::Int,
        Token::String => SymType::String,
        Token::Float => SymType::Float,

        // Unreachable as these shouldn't have made it past the parsing stage
        Token::Void => {
            if !is_var {
                SymType::Void
            } else {
                unreachable!("variable declaration w/ type void is illegal");
            }
        }
        _ => unreachable!("invalid function type found"),
    }
}

/// Travels up the spaghetti stack, looking for a provided identifier, stopping if it reaches the
/// root, in which case it returns None.
pub fn find_info_in_table(
    symbol_table: &SpaghettiStack,
    node_id: Id,
    ident: &str,
    is_var: bool,
) -> Option<SymInfo> {
    // TODO: add this as a method of SpaghettiStack
    let mut curr_id: Option<Id> = Some(node_id);

    while curr_id.is_some() {
        if let Some(ret) = symbol_table.get_ident_info(curr_id.unwrap(), ident) {
            if ret.is_var() == is_var {
                return Some(ret.clone());
            }
        } else {
            curr_id = symbol_table.get_node_parent_id(curr_id.unwrap());
        }
    }

    None
}
