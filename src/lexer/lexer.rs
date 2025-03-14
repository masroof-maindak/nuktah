use crate::lexer::token::Token;

const DELIM: &str = " \r\n\t\"\'\\&|;=(){}[]<>+-*/%^`!`.:~,$";

#[derive(Debug)]
pub enum LexerError {
    UnterminatedStringLit,
    InvalidIdentifier(String),
}

pub fn tokenize_src_code(src: &str) -> Result<Vec<Token>, LexerError> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut idx = 0;
    let mut quotes_started = false;
    let mut comment_started = false;

    while idx < src.len() {
        let word = strtok(src, DELIM, &mut idx);
        let mut t = identify_token(word, quotes_started)?;

        // if comment started, ignore all tokens until newline
        if comment_started {
            match t {
                Token::Newline => comment_started = false,
                _ => continue,
            }
        }

        if t == Token::Comment {
            comment_started = true;
            continue;
        }

        // TODO: combine `intlit • dot • intlit` into `floatlit`

        consolidate_tokens(&mut token_list, &mut t, quotes_started);

        if t == Token::Quotes {
            quotes_started = !quotes_started;
        }

        token_list.push(t);
    }

    token_list.retain(|t| ![Token::Whitespace, Token::Newline].contains(&t));
    Ok(token_list)
}

fn strtok<'a>(src: &'a str, delims: &str, idx: &mut usize) -> &'a str {
    let remaining_text = &src[*idx..];

    let first_char = remaining_text.chars().next().unwrap();
    if delims.contains(first_char) {
        *idx += first_char.len_utf8();
        return &remaining_text[0..first_char.len_utf8()];
    }

    let byte_count = remaining_text
        .chars()
        .take_while(|c| !delims.contains(*c))
        .map(|c| c.len_utf8())
        .sum();

    if byte_count == remaining_text.len() {
        *idx += byte_count;
        return remaining_text;
    }

    *idx += byte_count;
    return &remaining_text[..byte_count];
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
            // Int
            if let Ok(n) = word.parse::<i64>() {
                return Ok(Token::IntLit(n));
            }

            // Identifier
            // Start with a letter or underscore
            if !word.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                return Err(LexerError::InvalidIdentifier(word.to_string()));
            }

            // word should only contain letters, numbers, and underscores
            if !word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(LexerError::InvalidIdentifier(word.to_string()));
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
