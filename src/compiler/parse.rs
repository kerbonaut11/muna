use std::str::Chars;
use multipeek::{IteratorExt, MultiPeek};

use super::key_words::KEY_WORDS;

#[derive(Debug,Clone)]
pub enum Token {
    Local,

    Ident(Box<str>),

    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(Box<str>),

    Add,Sub,Div,Mul,IDiv,Mod,Pow,
    And,Or,Shr,Shl,
    BoolAnd,BoolOr,BoolNot,
    Not,Neg,Len,
    Eq,NotEq,Less,LessEq,Greater,GreaterEq,Is,

    BracketO,BracketC,CurlyO,CurlyC,SquareO,SquareC,
    Colon,Comma,Dot
}

enum CharType {
    WhiteSpace,
    Symbol(Token),
    AlphaNummeric(char),
}

impl CharType {
    pub fn new(ch:char) -> Option<Self> {
        if Self::is_ident_char(ch) {
            Some(Self::AlphaNummeric(ch))
        } else if ch.is_whitespace() {
            Some(Self::WhiteSpace)
        } else {
            Some(Self::Symbol(match ch {
                '(' => Token::BracketO,
                ')' => Token::BracketC,
                '{' => Token::CurlyO,
                '}' => Token::CurlyC,
                '[' => Token::SquareO,
                ']' => Token::SquareC,
                ',' => Token::Comma,
                ':' => Token::Colon,
                _ => return None
            }))
        }
    }

    pub fn is_ident_char(ch:char) -> bool {
        ch.is_alphanumeric() || ch == '"'
    }
}

pub fn parse(chars:&mut MultiPeek<Chars<'_>>,tokens:&mut Vec<Token>) {
    let ch = match chars.next() {
        Some(ch) => ch,
        None => return,
    };

    match CharType::new(ch).unwrap() {
        CharType::WhiteSpace => parse(chars, tokens),
        CharType::AlphaNummeric(ch) => {
            let indent_str = parse_ident(chars, ch);
            tokens.push(indent_try_to_literal_or_keyword(indent_str));
            parse(chars, tokens);
        },
        CharType::Symbol(t) => {
            tokens.push(t);
            parse(chars, tokens);
        }
    }
}

pub fn parse_ident(chars:&mut MultiPeek<Chars<'_>>,first_ch:char) -> Box<str> {
    let mut str = String::from(first_ch);
    loop {
        let ch = *match chars.peek() {
            Some(ch) => ch,
            None => return str.into_boxed_str(),
        };

        if !CharType::is_ident_char(ch) && !(ch == '.' && chars.peek_nth(1).unwrap().is_numeric() ) {
            return str.into_boxed_str();  
        } else {
           str.push(ch);
        }
        let _ = chars.next();
    }
}

pub fn indent_try_to_literal_or_keyword(name:Box<str>) -> Token {
    let mut iter = name.chars();
    if iter.next().unwrap() == '"' {
        let _ = iter.next_back();
        return Token::StrLiteral(name);
    }

    match KEY_WORDS.get(&name) {
        Some(t) => return t.clone(),
        None => {}
    };

    match name.parse::<f64>() {
        Ok(x) => return if x % 1.0 == 0.0 {
            Token::IntLiteral(x as i64)
            } else {
                Token::FloatLiteral(x)
            },
        Err(_) =>  {}
    };

    return Token::Ident(name);
}

#[test]
fn test() {
    let str = "x:f(32.4)";
    let tokens = &mut vec![];
    parse(&mut str.chars().multipeek(), tokens);
    println!("{:?}",tokens);
}
