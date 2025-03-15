pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum CompilerError {
    LexerError(lexer::core::LexerError),
    ParserError(parser::core::ParseError),
}

impl From<lexer::core::LexerError> for CompilerError {
    fn from(err: lexer::core::LexerError) -> CompilerError {
        CompilerError::LexerError(err)
    }
}

impl From<parser::core::ParseError> for CompilerError {
    fn from(err: parser::core::ParseError) -> CompilerError {
        CompilerError::ParserError(err)
    }
}

pub fn compile_src(src_code: &str) -> Result<(), CompilerError> {
    let tokens = lexer::core::tokenize_src_code(src_code)?;
    println!("Tokens:\n{:?}\n", tokens);

    let ast_root = parser::core::parse_token_stream(&tokens)?;
    println!("AST:\n{:#?}\n", ast_root);

    Ok(())
}
