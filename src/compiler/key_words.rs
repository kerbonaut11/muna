use std::collections::HashMap;
use lazy_static::lazy_static;
use super::parse::Token;

lazy_static! {
    pub static ref KEY_WORDS: HashMap<Box<str>,Token> = {
        let mut m = HashMap::new();
        m.insert("local".into(), Token::Local);
        m.insert("and".into(),   Token::BoolAnd);
        m.insert("or".into(),    Token::BoolOr);
        m.insert("not".into(),   Token::BoolNot);
        m.insert("true".into(),  Token::BoolLiteral(true));
        m.insert("flase".into(), Token::BoolLiteral(false));
        m
    };
}