use crate::vm::Vm;
use crate::Result;
use crate::value::{Type, Value};

use super::err::Op;
use super::OpErr;

impl Vm {
    pub fn bitand(&mut self,lhs:Value,rhs:Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs&rhs));
            }
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__and", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__and", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::And, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn bitor(&mut self,lhs:Value,rhs:Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs|rhs));
            }
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__or", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__or", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Or, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn bitxor(&mut self,lhs:Value,rhs:Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs^rhs));
            }
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__xor", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__xor", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Or, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn shl(&mut self,lhs:Value,rhs:Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs<<rhs));
            }
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__shl", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__shl", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Shl, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn shr(&mut self,lhs:Value,rhs:Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs>>rhs));
            }
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__shr", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__shr", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Shr, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn not(&mut self,val:Value) -> Result<Value> {
        match val {
            Value::Int(x) => return Ok(Value::Int(!x)),
            Value::Table(mut val) => if let Some(x) = val.meta_call_unary("__not", self) {return x;},
            Value::UserData(mut val) => if let Some(x) = val.meta_call_unary("__not", self) {return x;},
            _ => {},
        }
        Err(OpErr::UnaryTypeErr { op: super::err::UnaryOp::Neg, ty: Type::of_val(&val) }.into())
    }

    pub fn bitandi(&mut self,lhs:Value,rhs: i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs&rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__and", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__and", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::And, lhs:Type::of_val(&lhs), rhs:Type::Int }.into())
    }

    pub fn bitori(&mut self,lhs:Value,rhs: i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs|rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__or", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__or", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Or, lhs:Type::of_val(&lhs), rhs:Type::Int }.into())
    }

    pub fn bitxori(&mut self,lhs:Value,rhs: i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs^rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__xor", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__xor", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Or, lhs:Type::of_val(&lhs), rhs:Type::Int }.into())
    }

    pub fn shri(&mut self,lhs:Value,rhs: u8) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs>>rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call((rhs as i64).into(), "__shr", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call((rhs as i64).into(), "__shr", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Shr, lhs:Type::of_val(&lhs), rhs:Type::Int }.into())
    }


    pub fn shli(&mut self,lhs:Value,rhs: u8) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs<<rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call((rhs as i64).into(), "__shl", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call((rhs as i64).into(), "__shl", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Shl, lhs:Type::of_val(&lhs), rhs:Type::Int }.into())
    }
}

