use crate::vm::Vm;
use crate::Result;
use crate::value::{Type, Value};

use super::err::Op;
use super::OpErr;


impl Vm {
    pub fn add(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Int(lhs+rhs)),
                Value::Float(rhs) => return Ok(Value::Float(lhs as f64+rhs)),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs+rhs as f64)),
                Value::Float(rhs) => return Ok(Value::Float(lhs+rhs)),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__add", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__add", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Add, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn sub(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Int(lhs-rhs)),
                Value::Float(rhs) => return Ok(Value::Float(lhs as f64-rhs)),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs-rhs as f64)),
                Value::Float(rhs) => return Ok(Value::Float(lhs-rhs)),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__sub", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__sub", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Sub, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn mul(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Int(lhs*rhs)),
                Value::Float(rhs) => return Ok(Value::Float(lhs as f64*rhs)),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs*rhs as f64)),
                Value::Float(rhs) => return Ok(Value::Float(lhs*rhs)),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__mul", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__mul", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mul, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn div(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => {
                    let x = lhs as f64/rhs as f64;
                    return Ok(match x % 1.0 == 0.0 {
                        true => Value::Int(x as i64),
                        false => Value::Float(x)
                    });
                },
                Value::Float(rhs) => return Ok(Value::Float(lhs as f64/rhs)),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs/rhs as f64)),
                Value::Float(rhs) => return Ok(Value::Float(lhs/rhs)),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__div", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__div", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Div, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn idiv(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => if let Value::Int(rhs) = rhs {
                return Ok(Value::Int(lhs/rhs));
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__idiv", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__idiv", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::IDiv, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn rem(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Int(lhs%rhs)),
                Value::Float(rhs) => return Ok(Value::Float(lhs as f64%rhs)),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs%rhs as f64)),
                Value::Float(rhs) => return Ok(Value::Float(lhs%rhs)),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__mod", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__mod", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mod, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn pow(&mut self, lhs:Value, rhs: Value) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float((lhs as f64).powi(rhs as i32))),
                Value::Float(rhs) => return Ok(Value::Float((lhs as f64).powf(rhs))),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(Value::Float(lhs.powi(rhs as i32))),
                Value::Float(rhs) => return Ok(Value::Float(lhs.powf(rhs))),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__pow", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs, "__pow", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::IDiv, lhs:Type::of_val(&lhs), rhs:Type::of_val(&rhs) }.into())
    }

    pub fn neg(&mut self, val:Value) -> Result<Value> {
        match val {
            Value::Int(x) => return Ok(Value::Int(-x)),
            Value::Float(x) => return Ok(Value::Float(-x)),

            Value::Table(mut val) => if let Some(x) = val.meta_call_unary("__neg", self) {return x;},
            Value::UserData(mut val) => if let Some(x) = val.meta_call_unary("__neg", self) {return x;},
            _ => {},
        }
        Err(OpErr::UnaryTypeErr { op: super::err::UnaryOp::Not, ty: Type::of_val(&val) }.into())
    }

    pub fn addi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs+rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs+rhs as f64)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__add", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__add", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Add, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn subi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs-rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs-rhs as f64)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__sub", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__sub", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Sub, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn muli(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs*rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs*rhs as f64)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mul", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mul", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mul, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn divi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64/rhs as f64)),
            Value::Float(lhs) => return Ok(Value::Float(lhs/rhs as f64)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Div, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn idivi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs/rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::IDiv, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn remi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Int(lhs%rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs%rhs as f64)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mod", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mod", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mod, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn powi(&mut self,lhs:Value,rhs:i64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float((lhs as f64).powi(rhs as i32))),
            Value::Float(lhs) => return Ok(Value::Float(lhs.powi(rhs as i32))),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__pow", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__pow", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Pow, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn addf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64+rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs+rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__add", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__add", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Add, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn subf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64-rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs-rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__sub", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__sub", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Sub, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn mulf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64*rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs*rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mul", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mul", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mul, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn divf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64/rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs/rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__div", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Div, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn remf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float(lhs as f64%rhs)),
            Value::Float(lhs) => return Ok(Value::Float(lhs%rhs)),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mod", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__mod", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Mod, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn powf(&mut self,lhs:Value,rhs:f64) -> Result<Value> {
        match lhs {
            Value::Int(lhs) => return Ok(Value::Float((lhs as f64).powf(rhs))),
            Value::Float(lhs) => return Ok(Value::Float(lhs.powf(rhs))),
            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__pow", self) {return x;},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__pow", self) {return x;},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Pow, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }
}
