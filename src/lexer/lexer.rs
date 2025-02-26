const DELIM: &str = "(){}[]`;=\r\n\t\"\' ";

#[derive(Debug)]
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

/// Fuck me, I spent way too long before realising the strtok approach probably won't work
/// because of `++` and company, i.e cases where a delimiter itself is part of a token

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

}

pub fn tokenize_src_code(src: &String) -> Result<Vec<Token>, &'static str> {
    let mut token_list: Vec<Token> = Vec::new();
    let mut idx = 0;

    while idx < src.len() {
        let word = strtok(src, DELIM, &mut idx);
        let t = identify_token(word)?;
        token_list.push(t)
    }

    Ok(token_list)
}

fn identify_token(_word: &str) -> Result<Token, &'static str> {
    // Start matching every regex string with extracted word/character
    // Select the largest matching && highest priority string
    // check for conflict/'ambiguity' -> error out or resolve
    Ok(Token::IntLit(24545))
}
