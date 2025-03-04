pub mod lexer;
pub mod parser;

// pub enum CompilerError {
//     Nil,
//     LexerError,
// }

pub fn compile_src(src_code: &mut String) -> Result<(), &'static str> {
    // CHECK: should I pass a symbol table to this too?
    let tokens = lexer::lexer::tokenize_src_code(src_code)?;

    println!("{:?}", tokens);

    let _ = parser::parser::parse_token_list(tokens);

    Ok(())
}
