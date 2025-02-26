const DELIM: &str = "(){}[]`;=\r\n\t\"\' ";

#[derive(Debug, PartialEq)]
pub enum Token {
    // for, while
    For,
    While,

    // int, str, float, fn
    Int,
    String,
    Float,
    Function,

    // (), {}, [], ``, "", '', ;
    ParenL,
    ParenR,
    BraceL,
    BraceR,
    BracketL,
    BracketR,
    BacktickL,
    BacktickR,
    QuotesL,
    QuotesR,
    QuoteL,
    QuoteR,
    Semicolon,
    Whitespace,

    // main, foo, bar, baz
    Identifier(String),

    // 33, `hello world!`, 5.1
    IntLit(i64),
    StringLit(String),
    FloatLit(f64),

    // =, +, -, *, /, %, ^, ++, --, ==
    AssignOp,
    AddOp,
    SubOp,
    MulOp,
    DivOp,
    ModOp,
    ExpOp,
    IncementOp,
    DecrementOp,
    EqualsOp,
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

fn consolidate_tokens(token_list: &mut Vec<Token>, t: &mut Token) {
    if token_list.len() == 0
        || *t != Token::AddOp
        || *t != Token::MulOp
        || *t != Token::SubOp
        || *t != Token::Whitespace
    {
        return;
    }

    let last_token = token_list.last().unwrap();

    if *last_token == *t {
        token_list.pop();
        *t = match *t {
            Token::AddOp => Token::IncementOp,
            Token::SubOp => Token::DecrementOp,
            Token::MulOp => Token::ExpOp,
            _ => return,
        }
    }
}

pub fn tokenize_src_code(src: &String) -> Result<Vec<Token>, &'static str> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut idx = 0;

    while idx < src.len() {
        let word = strtok(src, DELIM, &mut idx);
        let mut t = identify_token(word)?;
        consolidate_tokens(&mut token_list, &mut t);
        token_list.push(t);
    }

    Ok(token_list)
}

fn identify_token(_word: &str) -> Result<Token, &'static str> {
    // Start matching every regex string with extracted word/character
    // Select the largest matching && highest priority string
    // check for conflict/'ambiguity' -> error out or resolve
    Ok(Token::IntLit(24545))
}
