pub mod ir;
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
    // println!("AST:\n{:#?}\n\n\n", ast_root);

    let _ = semantics::core::analyse_semantics(&ast_root)?;
    // println!("\n\nSymbol Table:\n{:#?}\n", sym_table);

    // Generate TAC blocks and code
    let tac_blocks = ir::generate_tac_blocks(ast_root);
    let tac_code = ir::generate_tac_code(tac_blocks);

    // Print TAC code
    println!("\n=== TAC CODE ===");
    for (i, code) in tac_code.iter().enumerate() {
        println!("{}", code.format_instruction(i));
    }

    Ok(())
}
