use std::collections::HashMap;
use lazy_static::lazy_static;
use super::tokenizer::Token;

lazy_static! {
    pub static ref KEY_WORDS: HashMap<Box<str>,Token> = {
        let mut m = HashMap::new();
        m.insert("local".into(),    Token::Local);
        m.insert("function".into(), Token::Function);
        m.insert("if".into(),       Token::If);
        m.insert("else".into(),     Token::Else);
        m.insert("while".into(),    Token::While);
        m.insert("do".into(),       Token::Do);
        m.insert("end".into(),      Token::End);

        m.insert("and".into(),   Token::BoolAnd);
        m.insert("or".into(),    Token::BoolOr);
        m.insert("not".into(),   Token::BoolNot);
        m.insert("true".into(),  Token::BoolLiteral(true));
        m.insert("flase".into(), Token::BoolLiteral(false));
        m
    };
}

lazy_static! {
    pub static ref MERGE_PATTERNS: HashMap<(u8,u8),Token> = {
        let mut m = HashMap::new();
        m.insert((Token::Less.tag(),      Token::Less.tag()),    Token::Shl);
        m.insert((Token::Greater.tag(),   Token::Greater.tag()), Token::Shr);
        m.insert((Token::Assing.tag(),    Token::Assing.tag()),  Token::Eq);
        m.insert((Token::Greater.tag(),   Token::Assing.tag()),  Token::GreaterEq);
        m.insert((Token::Less.tag(),      Token::Assing.tag()),  Token::LessEq);
        m
    };
}