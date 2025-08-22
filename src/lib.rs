pub mod lexer;
pub mod macros;
pub mod parser;
pub mod semantics;

#[derive(Debug)]
pub enum CompilerError {
    TokenizationErr(lexer::core::LexerError),
    ParseErr(parser::core::ParseError),
    SemanticErr(semantics::core::SemanticError),
}

convert_across_err!(lexer::core::LexerError, CompilerError, TokenizationErr);
convert_across_err!(parser::core::ParseError, CompilerError, ParseErr);
convert_across_err!(semantics::core::SemanticError, CompilerError, SemanticErr);

pub fn compile_src(src_code: &str) -> Result<(), CompilerError> {
    let tokens = lexer::core::tokenize_src_code(src_code)?;
    // println!("Tokens:\n{:?}\n", tokens);

    let ast_root = parser::core::parse_token_stream(&tokens)?;
    // println!("AST:\n{:#?}\n", ast_root);

    let sym_table = semantics::core::analyse_semantics(&ast_root)?;
    println!("Symbol Table:\n{sym_table:#?}\n");

    Ok(())
}
