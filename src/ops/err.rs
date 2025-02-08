use std::num::{ParseFloatError, ParseIntError};

use crate::value::Type;
use crate::table::InvalidKey;

#[repr(u8)]
#[derive(Debug)]
pub enum OpErr {
    TypeErr{op:Op,lhs:Type,rhs:Type},
    UnaryTypeErr{op:UnaryOp,ty:Type},
    CompMetaFuncReturnedNonBool{got:Type},
    TruthyMetaFuncReturnedNonBool{got:Type},
    CastMetaFuncReturnedWrongType{expected:Type, got:Type},
    InvalidKey(InvalidKey),
    IndexedInvalidType(Type),
    InvalidUdIndex(Type),
    DivZero,
    ParseIntErr(ParseIntError),
    ParseFloatErr(ParseFloatError),
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
    Shl,

    Eq,
    Less,
    LessEq,
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    Len,

    ToInt,
    ToFloat,
    ToString
}

impl From<InvalidKey> for OpErr {
    fn from(value: InvalidKey) -> Self {
        Self::InvalidKey(value)
    }
}