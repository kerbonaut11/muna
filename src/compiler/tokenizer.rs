use std::{iter::Peekable, str::Bytes,};

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

enum TokenizerErr {
    NonAscii,
    InvalidSymbol,
    EarlyEOF,
}

type Result<T> = std::result::Result<T,TokenizerErr>;

pub fn parse(tokens:&mut Vec<Token>,bytes:&mut Peekable<Bytes>) -> Result<()> {
    let byte = match bytes.next() {
        Some(byte) => byte,
        None => return Ok(()),
    };

    if !byte.is_ascii() {return Err(TokenizerErr::NonAscii);}

    let token = match byte {

        b'a'..b'z' | b'A'..b'Z' => parse_ident(byte,bytes),
        b'0'..b'9' => parse_num(byte,bytes),
        b'"' => parse_str(bytes),

        b'+' => Token::Add,


        b' ' | b'\t' | b'\n' => return parse(tokens, bytes),
        _ => return Err(TokenizerErr::InvalidSymbol)
    }

    parse(tokens, bytes)
}

fn parse_ident(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    let mut str = String::new();
    loop {
        let byte = match bytes.next() {
            Some(byte) => byte,
            None => return Ok(Token::Ident(str.into_boxed_str())),
        };

        if !byte.is_ascii() {return Err(TokenizerErr::NonAscii);}

        match byte {
            b'a'..b'z' | b'A'..b'Z' | b'0'..b'9' | b'_' => str.push(byte as char),
            _ => return Ok(Token::Ident(str.into_boxed_str())),
        }
    }
}

fn parse_num(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    if first == b'0' && bytes.peek().ok_or(TokenizerErr::EarlyEOF)? == b'x' {
        let _ = bytes.next();
        parse_hex(first, bytes)
    } else if first == b'0' && bytes.peek().ok_or(TokenizerErr::EarlyEOF)? == b'b' {
        let _ = bytes.next();
        parse_binary(first, bytes)
    } else {
        todo!()
    }
}

fn parse_hex(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    todo!()
}

fn parse_binary(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    todo!()
}

fn parse_str(bytes:&mut Peekable<Bytes>) -> Token {
    let mut str = String::new();
    loop {
        let byte = match bytes.next() {
            Some(byte) => byte,
            None => return Token::StrLiteral(str.into_boxed_str()),
        };

        if byte == b'"' {
            return Token::StrLiteral(str.into_boxed_str());
        } else {
            str.push(byte as char);
        }
    }
}


#[test]
fn test() {
    let str = "x:f(32.4)";
    let tokens = &mut vec![];
    parse(&mut str.bytes().peekable(), tokens);
    println!("{:?}",tokens);
}
