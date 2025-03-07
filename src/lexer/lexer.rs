use crate::lexer::token::Token;

const DELIM: &[u8] = b" \r\n\t\"\'\\&|;=(){}[]<>+-*/%^`!`.:~,$";

pub fn tokenize_src_code(src: &String) -> Result<Vec<Token>, &'static str> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut idx = 0;
    let mut quotes_started = false;

    while idx < src.len() {
        let word = strtok(src, DELIM, &mut idx);
        let mut t = identify_token(word, quotes_started)?;
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

fn identify_token(word: &str, quotes_started: bool) -> Result<Token, &'static str> {
    if word == "\"" {
        return Ok(Token::Quotes);
    }

    if quotes_started {
        return Ok(Token::StringLit(word.to_string()));
    }

    match word {
        "for" => return Ok(Token::For),
        "while" => return Ok(Token::While),
        "if" => return Ok(Token::If),
        "else" => return Ok(Token::Else),
        "elif" => return Ok(Token::ElseIf),
        "ret" => return Ok(Token::Return),

        "int" => return Ok(Token::Int),
        "float" => return Ok(Token::Float),
        "char" => return Ok(Token::Char),
        "string" => return Ok(Token::String),
        "bool" => return Ok(Token::Bool),
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

        "TRUE" => return Ok(Token::BooleanLit(true)),
        "FALSE" => return Ok(Token::BooleanLit(false)),

        _ => {
            if let Ok(n) = word.parse::<i64>() {
                return Ok(Token::IntLit(n));
            }

            if let Ok(n) = word.parse::<f64>() {
                return Ok(Token::FloatLit(n));
            }

            return Ok(Token::Identifier(word.to_string()));
        }
    }
}

fn consolidate_tokens(token_list: &mut Vec<Token>, t: &mut Token, quotes_started: bool) {
    if token_list.is_empty()
        || ![
            Token::AddOp,
            Token::SubOp,
            Token::AssignOp,
            Token::Whitespace,
            Token::Quotes,
            Token::LessThan,
            Token::GreaterThan,
            Token::BitwiseAnd,
            Token::BitwiseOr,
        ]
        .contains(t)
    {
        return;
    }

    let last_token = token_list.last().unwrap();

    // Literal quotes
    if *last_token == Token::StringLit("\\".to_string()) && *t == Token::Quotes {
        token_list.pop();
        *t = Token::StringLit("\\\"".to_string());
        return;
    }

    // ++, --, ==, <<, >>, &&, ||
    if *last_token == *t {
        //  qs - !ws => POP
        // !qs - !ws => POP
        // !qs -  ws => POP
        //  qs -  ws => NO POP

        if *t != Token::Whitespace || !quotes_started {
            token_list.pop();
        }

        *t = match *t {
            Token::AddOp => Token::IncrementOp,
            Token::SubOp => Token::DecrementOp,
            Token::AssignOp => Token::EqualsOp,
            Token::LessThan => Token::ShiftLeft,
            Token::GreaterThan => Token::ShiftRight,
            Token::BitwiseAnd => Token::BooleanAnd,
            Token::BitwiseOr => Token::BooleanOr,
            _ => return,
        }
    }
}
