pub mod lexer;
pub mod macros;
pub mod parser;
pub mod semantics;

#[derive(Debug)]
pub enum CompilerError {
    Lexer(lexer::core::LexerError),
    Parser(parser::core::ParseError),
    Semantics(semantics::core::SemanticError),
}

convert_across_err!(lexer::core::LexerError, CompilerError, Lexer);
convert_across_err!(parser::core::ParseError, CompilerError, Parser);
convert_across_err!(semantics::core::SemanticError, CompilerError, Semantics);

pub fn compile_src(src_code: &str) -> Result<(), CompilerError> {
    let tokens = lexer::core::tokenize_src_code(src_code)?;
    // println!("Tokens:\n{:?}\n", tokens);

    let ast_root = parser::core::parse_token_stream(&tokens)?;
    // println!("AST:\n{:#?}\n", ast_root);

    let sym_table = semantics::core::analyse_semantics(&ast_root)?;
    println!("Symbol Table:\n{:#?}\n", sym_table);

    Ok(())
}
