use std::{hash::Hash, iter::Peekable, str::Bytes};

#[derive(Debug,Clone,PartialEq)]
pub enum Token {
    Invalid,

    Local,Function,Return,
    If,Elif,Else,
    While,For,IPairs,KVPairs,Range,In,
    Break,
    Endline,

    Ident(Box<str>),

    Nil,
    BoolLiteral(bool),
    IntLiteral(i32),
    FloatLiteral(f32),
    StrLiteral(Box<str>),

    Assing,

    Add,Sub,Div,Mul,IDiv,Mod,Pow,
    And,Or,Xor,Shr,Shl,
    BoolAnd,BoolOr,BoolNot,
    Not,Neg,Len,
    Eq,NotEq,Less,LessEq,Greater,GreaterEq,Is,

    RoundO,RoundC,CurlyO,CurlyC,SquareO,SquareC,
    Colon,Comma,Dot
}

impl Token {
    pub fn tag(&self) -> u8 {
        unsafe {
            use std::ptr;
            *(ptr::from_ref(self) as *const u8)
        }
    }
}

impl Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.tag());
    }
}

impl Eq for Token {}

#[derive(Debug)]
pub enum TokenizerErr {
    NonAscii,
    InvalidSymbol(u8),
    EarlyEOF,
}

type Result<T> = std::result::Result<T,TokenizerErr>;

pub fn parse(str:&str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    match parse_rec(&mut tokens, &mut str.bytes().peekable()) {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}",tokens);
            return Err(e);
        }
    }
    Ok(tokens)
}


fn parse_rec(tokens:&mut Vec<Token>,bytes:&mut Peekable<Bytes>) -> Result<()> {
    let byte = match bytes.next() {
        Some(byte) => byte,
        None => return Ok(()),
    };

    if !byte.is_ascii() {return Err(TokenizerErr::NonAscii);}

    let token = match byte {

        b'a'..=b'z' | b'A'..=b'Z' => {
            let name = parse_ident(byte,bytes)?;
            match name.as_str() {
                "local"    => Token::Local,
                "function" => Token::Function,

                "if"       => Token::If,
                "elif"     => Token::Elif,
                "else"     => Token::Else,
                "while"    => Token::While,
                "for"      => Token::For,
                "ipairs"   => Token::IPairs,
                "kvpairs"  => Token::KVPairs,
                "range"    => Token::Range,
                "in"       => Token::In,
                "break"    => Token::Break,
                "return"   => Token::Return,

                "and"      => Token::BoolAnd,
                "or"       => Token::BoolOr,
                "not"      => Token::BoolNot,
                "true"     => Token::BoolLiteral(true),
                "flase"    => Token::BoolLiteral(false),
                _ => Token::Ident(name.into())
            }
        },

        b'0'..=b'9' => parse_num(byte,bytes)?,
        b'"' => parse_str(bytes),

        b'(' => Token::RoundO,
        b')' => Token::RoundC,
        b'{' => Token::CurlyO,
        b'}' => Token::CurlyC,
        b'[' => Token::SquareO,
        b']' => Token::SquareC,

        b'.' => Token::Dot,
        b',' => Token::Comma,
        b':' => Token::Colon,
        b';' => Token::Endline,

        b'+' => Token::Add,
        b'-' => Token::Sub,
        b'*' => Token::Mul,
        b'/' => Token::Div,
        b'%' => Token::Mod,
        b'^' => Token::Pow,

        b'!' => Token::Not,
        b'#' => Token::Len,

        b'&' => Token::And,
        b'|' => Token::Or,
        b'~' => Token::Xor,

        b'=' => Token::Assing,
        b'<' => Token::Less,
        b'>' => Token::Greater,

        b' ' | b'\t' | b'\n' => return parse_rec(tokens, bytes),
        _ => return Err(TokenizerErr::InvalidSymbol(byte))
    };

    if tokens.is_empty() {
        tokens.push(token);
        return parse_rec(tokens, bytes);
    }

    let prev_token = tokens.pop().unwrap();
    match (&token,&prev_token) {
        (Token::Less,     Token::Less)    => tokens.push(Token::Shl),
        (Token::Greater,  Token::Greater) => tokens.push(Token::Shr),
        (Token::Assing,   Token::Assing)  => tokens.push(Token::Eq),
        (Token::Greater,  Token::Assing)  => tokens.push(Token::GreaterEq),
        (Token::Less,     Token::Assing)  => tokens.push(Token::LessEq),
        _ => {
            tokens.push(prev_token);
            tokens.push(token);
        },
    }

    parse_rec(tokens, bytes)
}

fn parse_ident(first:u8,bytes:&mut Peekable<Bytes>) -> Result<String> {
    let mut name = String::from(first as char);
    loop {
        let byte = match bytes.peek() {
            Some(byte) => *byte,
            None => return Ok(name),
        };

        if !byte.is_ascii() {return Err(TokenizerErr::NonAscii);}

        match byte {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => name.push(byte as char),
            _ => return Ok(name),
        }

        let _ = bytes.next();
    }
}

fn parse_num(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    if first == b'0' && *bytes.peek().ok_or(TokenizerErr::EarlyEOF)? == b'x' {
        let _ = bytes.next();
        parse_hex(bytes)
    } else if first == b'0' && *bytes.peek().ok_or(TokenizerErr::EarlyEOF)? == b'b' {
        let _ = bytes.next();
        parse_binary(bytes)
    } else {
        parse_int(first, bytes)
    }
}

fn parse_hex(bytes:&mut Peekable<Bytes>) -> Result<Token> {
    let mut x = 0i32;
    loop {
        let byte = *bytes.peek().ok_or(TokenizerErr::EarlyEOF)?;
        match byte {
            b'0'..=b'9' => x = (x << 0xF) | (byte-b'0')as i32,
            b'a'..=b'f' => x = (x << 0xF) | (byte-b'a')as i32,
            b'A'..=b'F' => x = (x << 0xF) | (byte-b'A')as i32,
            _ => return Ok(Token::IntLiteral(x)),
        }
        let _ = bytes.next();
    }
}

fn parse_binary(bytes:&mut Peekable<Bytes>) -> Result<Token> {
    let mut x = 0i32;
    loop {
        let byte = *bytes.peek().ok_or(TokenizerErr::EarlyEOF)?;
        match byte {
            b'0' => x =  x << 1, 
            b'1' => x = (x << 1) | 1,
            _ => return Ok(Token::IntLiteral(x)),
        }
        let _ = bytes.next();
    }
}

fn parse_int(first:u8,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    let mut x = (first-b'0') as i32;
    loop {
        let byte = *bytes.peek().ok_or(TokenizerErr::EarlyEOF)?;
        match byte {
            b'0'..=b'9' => x = (x*10) + (byte-b'0') as i32, 
            b'.' => {
                let _ = bytes.next();
                return parse_float_decimal(x, bytes);
            },
            _ => return Ok(Token::IntLiteral(x)),
        }
        let _ = bytes.next();
    }
}

fn parse_float_decimal(whole:i32,bytes:&mut Peekable<Bytes>) -> Result<Token> {
    let mut x = whole as f32;
    let mut pow = 0.1;
    loop {
        let byte = *bytes.peek().ok_or(TokenizerErr::EarlyEOF)?;
        match byte {
            b'0'..b'9' => {
                x += (byte-b'0') as f32 * pow;
                pow *= 0.1;
            }, 
            _ => return Ok(Token::FloatLiteral(x)),
        }
        let _ = bytes.next();
    }
}

fn parse_str(bytes:&mut Peekable<Bytes>) -> Token {
    let mut str = String::new();
    loop {
        let byte = match bytes.next() {
            Some(byte) => byte,
            None => return Token::StrLiteral(str.into()),
        };

        if byte == b'"' {
            return Token::StrLiteral(str.into());
        } else {
            str.push(byte as char);
        }
    }
}


#[test]
fn test() {
    match parse( "local foo = bar.baz:f(32.42 + (4<<i), \"abc\")") {
        Ok(tokens) => println!("{:?}",tokens),
        Err(e) => panic!("{:?}",e)
    }
}
