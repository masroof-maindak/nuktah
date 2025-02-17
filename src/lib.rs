pub mod lexer;

pub enum CompilerError {
    Nil,
    LexerError,
}

pub fn compile_src(src_code: &mut String) -> Result<(), &'static str> {
    let tokens = lexer::lexer::tokenize_src_code(src_code)?;
    println!("{:?}", tokens);

    Ok(())
}
