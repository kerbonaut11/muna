use crate::value::Type;
use crate::table::InvalidKey;

#[repr(u8)]
#[derive(Debug)]
pub enum OpErr {
    TypeErr{op:Op,lhs:Type,rhs:Type},
    UnaryTypeErr{op:UnaryOp,ty:Type},
    CompMetaFuncReturnedNonBool{got:Type},
    TruthyMetaFuncReturnedNonBool{got:Type},
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
    Shl,

    Eq,
    Less,
    LessEq
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
    Len,
}

impl From<InvalidKey> for OpErr {
    fn from(value: InvalidKey) -> Self {
        Self::InvalidKey(value)
    }
}