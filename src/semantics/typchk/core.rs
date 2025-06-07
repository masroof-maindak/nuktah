use crate::{
    parser::ast::core::*,
    semantics::{errors::TypeChkError, spaghetti::SpaghettiStack},
};

pub fn check_types(
    _ast_root: &TranslationUnit,
    _symbol_table: &SpaghettiStack,
) -> Result<(), TypeChkError> {
    Ok(())
}
