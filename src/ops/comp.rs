use crate::gc::Gc;
use crate::table::Table;
use crate::value::{Type, Value};
use crate::vm::Vm;
use crate::Result;

use super::{Op, OpErr};

impl Vm {
    pub fn eq(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        Ok(match lhs {
            Value::Nil => if let Value::Nil = rhs {true} else {false},
            Value::Bool(lhs) => if let Value::Bool(rhs) = rhs {lhs == rhs} else {false},
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => lhs == rhs,
                Value::Float(rhs) => lhs as f64 == rhs,
                _ => false
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => lhs == rhs as f64,
                Value::Float(rhs) => lhs == rhs,
                _ => false
            }

            Value::String(lhs) => if let Value::String(rhs) = rhs {*lhs == *rhs} else {false},

            Value::Table(mut lhs) => {
                if let Some(result) = lhs.meta_call(rhs, "__eq", self) {
                    try_to_bool(result?)?
                } else {
                    if let Value::Table(rhs) = rhs {
                        self.table_eq(lhs, rhs)?
                    } else {false}
                }
            }

            Value::Function(lhs) => if let Value::Function(rhs) = rhs {*lhs == *rhs} else {false},

            Value::UserData(mut lhs) => {
                if let Some(result) = lhs.meta_call(rhs, "__eq", self) {
                    try_to_bool(result?)?
                } else {
                    if let Value::Table(rhs) = rhs {
                        lhs.addres() == rhs.addres()
                    } else {false}
                }
            }

            Value::UpValue(_) => unreachable!()
        })
    }

    fn table_eq(&mut self,lhs:Gc<Table>,rhs:Gc<Table>) -> Result<bool> {

        if lhs.addres() == rhs.addres() {return Ok(true);}
        if lhs.array.len() != lhs.array.len() {return Ok(false);}
        if lhs.map.len() != lhs.map.len() {return Ok(false);}

        for i in 0..lhs.array.len() {
            if !self.eq(lhs.array[i],rhs.array[i])? {return Ok(false);}
        }

        for (lhs,rhs) in lhs.map.iter().zip(rhs.map.iter()) {
            if lhs.0 != rhs.0 {return Ok(false);}
            if !self.eq(*lhs.1, *rhs.1)? {return Ok(false);}
        }

        Ok(true)
    }

    pub fn less(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(lhs<rhs),
                Value::Float(rhs) => return Ok((lhs as f64 )<rhs),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(lhs<rhs as f64),
                Value::Float(rhs) => return Ok(lhs<rhs),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            _ => {}
        }

        Err(OpErr::TypeErr { op: Op::Less, lhs: Type::of_val(&lhs), rhs: Type::of_val(&rhs) }.into())
    }

    pub fn less_eq(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => match rhs {
                Value::Int(rhs) => return Ok(lhs<=rhs),
                Value::Float(rhs) => return Ok((lhs as f64 )<=rhs),
                _ => {}
            }

            Value::Float(lhs) => match rhs {
                Value::Int(rhs) => return Ok(lhs<=rhs as f64),
                Value::Float(rhs) => return Ok(lhs<=rhs),
                _ => {}
            }

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            _ => {}
        }

        Err(OpErr::TypeErr { op: Op::LessEq, lhs: Type::of_val(&lhs), rhs: Type::of_val(&rhs) }.into())
    }

    pub fn eqi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => Ok(lhs==rhs),
            Value::Float(lhs) => Ok(lhs==rhs as f64),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__eq", self) {try_to_bool(x?)} else {Ok(false)},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__eq", self) {try_to_bool(x?)} else {Ok(false)},
            _ => Ok(false)
        }
    }

    pub fn lessi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => return Ok(lhs<rhs),
            Value::Float(lhs) => return Ok(lhs<rhs as f64),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Less, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn less_eqi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => return Ok(lhs<=rhs),
            Value::Float(lhs) => return Ok(lhs<=rhs as f64),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::LessEq, lhs: Type::of_val(&lhs), rhs: Type::Int }.into())
    }

    pub fn eqf(&mut self,lhs:Value,rhs:f64) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => Ok((lhs as f64)==rhs),
            Value::Float(lhs) => Ok(lhs==rhs),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__eq", self) {try_to_bool(x?)} else {Ok(false)},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__eq", self) {try_to_bool(x?)} else {Ok(false)},
            _ => Ok(false)
        }
    }

    pub fn lessf(&mut self,lhs:Value,rhs:f64)-> Result<bool> {
        match lhs {
            Value::Int(lhs) => return Ok((lhs as f64)<rhs),
            Value::Float(lhs) => return Ok(lhs<rhs),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__less", self) {return try_to_bool(x?);},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::Less, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }

    pub fn less_eqf(&mut self,lhs:Value,rhs:f64) -> Result<bool> {
        match lhs {
            Value::Int(lhs) => return Ok((lhs as f64)<=rhs),
            Value::Float(lhs) => return Ok(lhs<=rhs),

            Value::Table(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            Value::UserData(mut lhs) => if let Some(x) = lhs.meta_call(rhs.into(), "__leq", self) {return try_to_bool(x?);},
            _ => {}
        }
        Err(OpErr::TypeErr { op: Op::LessEq, lhs: Type::of_val(&lhs), rhs: Type::Float }.into())
    }
}


fn try_to_bool(val:Value) -> Result<bool> {
    match val {
        Value::Bool(x) => Ok(x),
        _ => Err(OpErr::CompMetaFuncReturnedNonBool { got: Type::of_val(&val) }.into())
    }
}