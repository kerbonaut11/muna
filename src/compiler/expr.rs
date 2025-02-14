use crate::ops::{Op, UnaryOp};

use super::tokenizer::Token;

pub enum Expr {
    NilLiteral,
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(Box<str>),

    Binary{op:Op,lhs:Box<Self>,rhs:Box<Self>},
    Unary{op:UnaryOp,val:Box<Self>}
}

pub fn parse(tokens:&[Token]) -> Expr {
    todo!()
}

fn find_end_expr(tokens:&[Token]) -> i64 {
    let mut i = 1usize;
    loop {
        let prev = &tokens[i-1];
        let token = &tokens[i];
        i += 1;
    }
}