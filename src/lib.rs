pub mod lexer;
pub mod parser;
pub mod semantics;

#[derive(Debug)]
pub enum CompilerError {
    LexerError(lexer::core::LexerError),
    ParserError(parser::core::ParseError),
    SemanticError(semantics::core::SemanticError),
}

// TODO: more elegant way to do this?

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

impl From<semantics::core::SemanticError> for CompilerError {
    fn from(err: semantics::core::SemanticError) -> CompilerError {
        CompilerError::SemanticError(err)
    }
}

pub fn compile_src(src_code: &str) -> Result<(), CompilerError> {
    let tokens = lexer::core::tokenize_src_code(src_code)?;
    println!("Tokens:\n{:?}\n", tokens);

    let ast_root = parser::core::parse_token_stream(&tokens)?;
    println!("AST:\n{:#?}\n", ast_root);

    // let sym_table = semantics::core::analyse_semantics(&ast_root)?;
    // println!("AST:\n{:#?}\n", sym_table);

    Ok(())
}
