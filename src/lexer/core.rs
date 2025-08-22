use super::Token;

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
        let mut t = identify_token(word, quotes_started, comment_started)?;

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

        if !token_list.is_empty() {
            consolidate_tokens(&mut token_list, &mut t, quotes_started);
        }

        if t == Token::Quotes {
            quotes_started = !quotes_started;
        }

        token_list.push(t);
    }

    token_list.retain(|t| ![Token::Whitespace, Token::Newline].contains(t));
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

    *idx += byte_count;
    &remaining_text[..byte_count]
}

fn identify_token(
    word: &str,
    quotes_started: bool,
    comment_started: bool,
) -> Result<Token, LexerError> {
    if word == "\"" {
        return Ok(Token::Quotes);
    }

    if quotes_started {
        return Ok(Token::StringLit(word.to_string()));
    }

    match word {
        "duhrao" => Ok(Token::For),
        "agar" => Ok(Token::If),
        "warna" => Ok(Token::Else),
        "wapsi" => Ok(Token::Return),
        "dhancha" => Ok(Token::Struct),
        "toro" => Ok(Token::Break),

        "ginti" => Ok(Token::Int),
        "asharia" => Ok(Token::Float),
        "jumla" => Ok(Token::String),
        "boli" => Ok(Token::Bool),
        "khali" => Ok(Token::Void),
        "fn" => Ok(Token::Function),
        "sach" => Ok(Token::True),
        "jhoot" => Ok(Token::False),

        "(" => Ok(Token::ParenL),
        ")" => Ok(Token::ParenR),
        "{" => Ok(Token::BraceL),
        "}" => Ok(Token::BraceR),
        "[" => Ok(Token::BracketL),
        "]" => Ok(Token::BracketR),
        "`" => Ok(Token::Backtick),
        "\'" => Ok(Token::Quote),

        " " => Ok(Token::Whitespace),
        "\n" => Ok(Token::Newline),
        "\t" => Ok(Token::Whitespace),
        "\r" => Ok(Token::Whitespace),
        ":" => Ok(Token::Colon),
        ";" => Ok(Token::Semicolon),
        "$" => Ok(Token::Comment),

        "=" => Ok(Token::AssignOp),
        "+" => Ok(Token::AddOp),
        "-" => Ok(Token::SubOp),
        "*" => Ok(Token::MulOp),
        "/" => Ok(Token::DivOp),
        "%" => Ok(Token::ModOp),
        "^" => Ok(Token::ExpOp),

        "." => Ok(Token::Dot),
        "," => Ok(Token::Comma),
        "!" => Ok(Token::BooleanNot),
        "&" => Ok(Token::BitwiseAnd),
        "|" => Ok(Token::BitwiseOr),
        "~" => Ok(Token::BitwiseNot),
        "<" => Ok(Token::LessThan),
        ">" => Ok(Token::GreaterThan),

        _ => {
            // No need to attempt to verify
            if comment_started {
                return Ok(Token::Identifier(word.to_string()));
            }

            // Int
            if let Ok(n) = word.parse::<i64>() {
                return Ok(Token::IntLit(n));
            }

            // Identifier

            // Starts with a letter or underscore
            if !word.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                return Err(LexerError::InvalidIdentifier(word.to_string()));
            }

            // Contains letters, numbers, and underscores
            if !word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(LexerError::InvalidIdentifier(word.to_string()));
            }

            Ok(Token::Identifier(word.to_string()))
        }
    }
}

fn consolidate_tokens(token_list: &mut Vec<Token>, curr_token: &mut Token, quotes_started: bool) {
    let last_token = token_list.last().unwrap();

    // This has got to be some of the nastiest shit I've ever written...

    // Combine string literals
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

    // Form float literal from two integers and decimal
    if let [.., Token::IntLit(int_part), Token::Dot] = token_list[..] {
        if let Token::IntLit(frac_part) = *curr_token {
            token_list.pop();
            token_list.pop();
            let f = format!("{int_part}.{frac_part}");
            *curr_token = Token::FloatLit(f.parse::<f64>().unwrap());
            return;
        }
    }

    // `==, <<, >>, &&, ||` from `=, <, >, &, |`
    if *last_token == *curr_token
        && matches!(
            curr_token,
            Token::AssignOp
                | Token::Whitespace
                | Token::Quotes
                | Token::LessThan
                | Token::GreaterThan
                | Token::BitwiseAnd
                | Token::BitwiseOr
        )
    {
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
