const DELIM: &str = " \r\n\t\"\'\\&|;=(){}[]<>+-*/%^`!`.:~";

#[derive(Debug, PartialEq)]
pub enum Token {
    // for, while
    For,
    While,
    If,
    Else,
    Return,

    // int, str, char, float, bool, fn
    Int,
    String,
    Char, // CHECK: how to handle char literals?
    Float,
    Bool,
    Function,

    // (), {}, [], `, ", ', , ;, \
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    Backtick,
    Quotes,
    Quote,

    Colon,
    Semicolon,
    Backslash,
    Whitespace,

    // main, foo, bar, baz
    Identifier(String),

    // 33, `hello world!`, 5.1, true, false
    IntLit(i64),
    StringLit(String),
    FloatLit(f64),
    BooleanLit(bool),

    // =, +, -, *, /, %, ^, ++, --, ==
    AssignOp,
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    ModOp,
    ExpOp,
    IncrementOp,
    DecrementOp,
    EqualsOp,

    // ., !, &, |, &&, ||, ~, <, >, <<, >>
    Dot,
    BooleanNot,
    BitwiseAnd,
    BitwiseOr,
    BooleanAnd,
    BooleanOr,
    BitwiseNot,
    LessThan,
    GreaterThan,
    ShiftLeft,
    ShiftRight,
}

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

fn strtok<'a>(src: &'a String, delims: &str, idx: &mut usize) -> &'a str {
    let tmp = &src[*idx..];
    let mut delim_offset = std::usize::MAX;

    for c in delims.chars() {
        match tmp.find(c) {
            Some(i) => {
                delim_offset = std::cmp::min(delim_offset, i);
                if delim_offset == 0 {
                    break;
                }
            }
            None => continue,
        }
    }

    if delim_offset == 0 {
        *idx += 1;
        return &tmp[0..1];
    }

    if delim_offset == std::usize::MAX {
        *idx = delim_offset;
        return tmp;
    }

    *idx += delim_offset;
    return &tmp[..delim_offset];
}

fn identify_token(word: &str, quotes_started: bool) -> Result<Token, &'static str> {
    if word == "\"" {
        return Ok(Token::Quotes);
    }

    if quotes_started {
        return Ok(Token::StringLit(word.to_string()));
    }

    match word {
        "=" => return Ok(Token::AssignOp),
        "+" => return Ok(Token::AddOp),
        "-" => return Ok(Token::SubOp),
        "*" => return Ok(Token::MulOp),
        "/" => return Ok(Token::DivOp),
        "%" => return Ok(Token::ModOp),
        "^" => return Ok(Token::ExpOp),
        "<" => return Ok(Token::LessThan),
        ">" => return Ok(Token::GreaterThan),
        "!" => return Ok(Token::BooleanNot),
        "&" => return Ok(Token::BitwiseAnd),
        "|" => return Ok(Token::BitwiseOr),
        "~" => return Ok(Token::BitwiseNot),
        "." => return Ok(Token::Dot),

        "(" => return Ok(Token::ParenL),
        ")" => return Ok(Token::ParenR),
        "{" => return Ok(Token::BraceL),
        "}" => return Ok(Token::BraceR),
        "[" => return Ok(Token::BracketL),
        "]" => return Ok(Token::BracketR),
        "`" => return Ok(Token::Backtick),
        "\'" => return Ok(Token::Quote),

        ":" => return Ok(Token::Colon),
        ";" => return Ok(Token::Semicolon),
        "\\" => return Ok(Token::Backslash),

        " " => return Ok(Token::Whitespace),
        "\n" => return Ok(Token::Whitespace),
        "\t" => return Ok(Token::Whitespace),
        "\r" => return Ok(Token::Whitespace),

        "int" => return Ok(Token::Int),
        "float" => return Ok(Token::Float),
        "char" => return Ok(Token::Char),
        "string" => return Ok(Token::String),
        "bool" => return Ok(Token::Bool),
        "fn" => return Ok(Token::Function),

        "for" => return Ok(Token::For),
        "while" => return Ok(Token::While),
        "if" => return Ok(Token::If),
        "else" => return Ok(Token::Else),
        "return" => return Ok(Token::Return),

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
