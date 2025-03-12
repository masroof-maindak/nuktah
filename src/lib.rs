pub mod lexer;
pub mod parser;

#[derive(Debug)]
pub enum CompilerError {
    LexerError(lexer::lexer::LexerError),
    ParserError(parser::parser::ParseError),
}

impl From<lexer::lexer::LexerError> for CompilerError {
    fn from(err: lexer::lexer::LexerError) -> CompilerError {
        CompilerError::LexerError(err)
    }
}

impl From<parser::parser::ParseError> for CompilerError {
    fn from(err: parser::parser::ParseError) -> CompilerError {
        CompilerError::ParserError(err)
    }
}

pub fn compile_src(src_code: &mut String) -> Result<(), CompilerError> {
    let tokens = lexer::lexer::tokenize_src_code(src_code)?;
    println!("{:?}", tokens);

    let ast_root = parser::parser::parse_token_stream(&tokens)?;
    println!("{:?}", ast_root);

    Ok(())
}
