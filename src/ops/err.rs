use derive_more::derive::From;

use crate::value::Type;
use crate::table::InvalidKey;

#[repr(u8)]
#[derive(From,Debug)]
pub enum OpErr {
    TypeErr{op:Op,lhs:Type,rhs:Type},
    UnaryTypeErr{op:UnaryOp,ty:Type},
    InvalidKey(InvalidKey),
    DivZero,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    IDiv,
    Mod,
    Pow,

    And,
    Or,
    Shr,
    Shl
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    Len,
}