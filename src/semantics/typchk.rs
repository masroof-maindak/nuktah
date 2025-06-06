use crate::{
    parser::ast,
    semantics::{errors::TypeChkError, spaghetti::SpaghettiStack},
};

pub fn check_types(
    _ast_root: &ast::core::TranslationUnit,
    _symbol_table: &SpaghettiStack,
) -> Result<(), TypeChkError> {
    Ok(())
}
