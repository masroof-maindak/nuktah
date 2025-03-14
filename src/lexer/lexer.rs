use crate::lexer::token::Token;

const DELIM: &[u8] = b" \r\n\t\"\'\\&|;=(){}[]<>+-*/%^`!`.:~,$";

#[derive(Debug)]
pub enum LexerError {
    UnterminatedStringLit,
    InvalidCharacter(String)
}

pub fn tokenize_src_code(src: &String) -> Result<Vec<Token>, LexerError> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut idx = 0;
    let mut quotes_started = false;

    while idx < src.len() {
        let word = strtok(src, DELIM, &mut idx);
        let mut t = identify_token(word, quotes_started)?;

        // TODO: combine `intlit • dot • intlit` into `floatlit`

        // TODO: implement comment consolidation and removal at this part
        if t == Token::Whitespace || t == Token::Newline {
            continue;
        }

        consolidate_tokens(&mut token_list, &mut t, quotes_started);

        if t == Token::Quotes {
            quotes_started = !quotes_started;
        }

        token_list.push(t);
    }

    Ok(token_list)
}

fn strtok<'a>(src: &'a String, delims: &[u8], idx: &mut usize) -> &'a str {
    let remaining_text = &src[*idx..];
    let (delim_offset, _) = remaining_text
        .bytes()
        .enumerate()
        .find(|(_, c)| delims.contains(c))
        .unwrap_or_else(|| (std::usize::MAX, b'~'));

    if delim_offset == 0 {
        *idx += 1;
        return &remaining_text[0..1];
    }

    if delim_offset == std::usize::MAX {
        *idx = delim_offset;
        return remaining_text;
    }

    *idx += delim_offset;
    return &remaining_text[..delim_offset];
}

fn identify_token(word: &str, quotes_started: bool) -> Result<Token, LexerError> {
    if word == "\"" {
        return Ok(Token::Quotes);
    }

    if quotes_started {
        return Ok(Token::StringLit(word.to_string()));
    }

    match word {
        "for" => return Ok(Token::For),
        "if" => return Ok(Token::If),
        "else" => return Ok(Token::Else),
        "ret" => return Ok(Token::Return),

        "int" => return Ok(Token::Int),
        "float" => return Ok(Token::Float),
        "string" => return Ok(Token::String),
        "fn" => return Ok(Token::Function),

        "(" => return Ok(Token::ParenL),
        ")" => return Ok(Token::ParenR),
        "{" => return Ok(Token::BraceL),
        "}" => return Ok(Token::BraceR),
        "[" => return Ok(Token::BracketL),
        "]" => return Ok(Token::BracketR),
        "`" => return Ok(Token::Backtick),
        "\'" => return Ok(Token::Quote),

        " " => return Ok(Token::Whitespace),
        "\n" => return Ok(Token::Newline),
        "\t" => return Ok(Token::Whitespace),
        "\r" => return Ok(Token::Whitespace),
        ":" => return Ok(Token::Colon),
        ";" => return Ok(Token::Semicolon),
        "$" => return Ok(Token::Comment),

        "=" => return Ok(Token::AssignOp),
        "+" => return Ok(Token::AddOp),
        "-" => return Ok(Token::SubOp),
        "*" => return Ok(Token::MulOp),
        "/" => return Ok(Token::DivOp),
        "%" => return Ok(Token::ModOp),
        "^" => return Ok(Token::ExpOp),

        "." => return Ok(Token::Dot),
        "," => return Ok(Token::Comma),
        "!" => return Ok(Token::BooleanNot),
        "&" => return Ok(Token::BitwiseAnd),
        "|" => return Ok(Token::BitwiseOr),
        "~" => return Ok(Token::BitwiseNot),
        "<" => return Ok(Token::LessThan),
        ">" => return Ok(Token::GreaterThan),

        _ => {
            if let Ok(n) = word.parse::<i64>() {
                return Ok(Token::IntLit(n));
            }

            if !word.chars().all(|c| c.is_alphabetic()) {
                return Err(LexerError::InvalidCharacter(word.to_string()));
            }

            return Ok(Token::Identifier(word.to_string()));
        }
    }
}

fn consolidate_tokens(token_list: &mut Vec<Token>, curr_token: &mut Token, quotes_started: bool) {
    if token_list.is_empty()
        || match curr_token {
            Token::AddOp
            | Token::SubOp
            | Token::AssignOp
            | Token::Whitespace
            | Token::Quotes
            | Token::LessThan
            | Token::GreaterThan
            | Token::BitwiseAnd
            | Token::BitwiseOr
            | Token::StringLit(_) => false,
            _ => true,
        }
    {
        return;
    }

    let last_token = token_list.last().unwrap();

    // If the last token is a string literal
    if let Token::StringLit(last_str) = last_token.clone() {
        // if the current token is a string literal, combine them
        if let Token::StringLit(curr_str) = curr_token {
            token_list.pop();
            *curr_token = Token::StringLit(last_str + curr_str);
            return;
        }

        // if the current token is a quote, and the last string literal ends with a backslash, combine them
        if *curr_token == Token::Quotes && last_str.ends_with('\\') {
            token_list.pop();
            *curr_token = Token::StringLit(last_str + "\"");
            return;
        }
    }

    // ==, <<, >>, &&, || from =, <, >, &, |
    if *last_token == *curr_token {
        //  qs - !ws => POP
        // !qs - !ws => POP
        // !qs -  ws => POP
        //  qs -  ws => NO POP

        if *curr_token != Token::Whitespace || !quotes_started {
            token_list.pop();
        }

        *curr_token = match *curr_token {
            Token::AssignOp => Token::EqualsOp,
            Token::LessThan => Token::ShiftLeft,
            Token::GreaterThan => Token::ShiftRight,
            Token::BitwiseAnd => Token::BooleanAnd,
            Token::BitwiseOr => Token::BooleanOr,
            _ => return,
        }
    }
}
