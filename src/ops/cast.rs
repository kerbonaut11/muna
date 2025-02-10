use crate::{gc::Gc, string::LuaString, value::{Type, Value}, vm::Vm, Result};

use super::{OpErr, UnaryOp};

impl Vm {

    pub fn to_bool(&mut self,val:Value) -> Result<bool> {
        match val {
            Value::Bool(x) => return Ok(x),
            Value::Int(x) => match x {
                0 => return Ok(false),
                1 => return Ok(true),
                _ => return Err(OpErr::InvalidBoolCast(val).into()),
            },
            Value::Float(x) => if x == 0.0 {
                return Ok(false);
            } else if x == 1.0 {
                return Ok(true);
            } else {
                return Err(OpErr::InvalidBoolCast(val).into());
            },
            Value::String(x) => {
                match x.to_str().parse::<bool>() {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(OpErr::InvalidBoolCast(val).into()),
                }
            },

            Value::Table(mut val) => {
                if let Some(x) = val.meta_call_unary("__bool", self) {
                    let x = x?;
                    return match x {
                        Value::Bool(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Bool, got: Type::of_val(&x).into() }.into())
                    };
                }
            },
            Value::UserData(mut val) => {
                if let Some(x) = val.meta_call_unary("__bool", self) {
                    let x = x?;
                    return match x {
                        Value::Bool(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Bool, got: Type::of_val(&x) }.into())
                    };
                }
            },
            _ => {}
        }
        Err(OpErr::UnaryTypeErr { op: UnaryOp::ToBool, ty: Type::of_val(&val) }.into())
    }

    pub fn to_int(&mut self,val:Value) -> Result<i64> {
        match val {
            Value::Bool(x) => return Ok(x as u8 as i64),
            Value::Int(x) => return Ok(x),
            Value::Float(x) => return Ok(x as i64),
            Value::String(x) => {
                match x.to_str().parse::<i64>() {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(OpErr::ParseIntErr(e).into())
                }
            },

            Value::Table(mut val) => {
                if let Some(x) = val.meta_call_unary("__int", self) {
                    let x = x?;
                    return match x {
                        Value::Int(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Int, got: Type::of_val(&x).into() }.into())
                    };
                }
            },
            Value::UserData(mut val) => {
                if let Some(x) = val.meta_call_unary("__int", self) {
                    let x = x?;
                    return match x {
                        Value::Int(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Int, got: Type::of_val(&x) }.into())
                    };
                }
            },
            _ => {}
        }
        Err(OpErr::UnaryTypeErr { op: UnaryOp::ToInt, ty: Type::of_val(&val) }.into())
    }

    pub fn to_float(&mut self,val:Value) -> Result<f64> {
        match val {
            Value::Bool(x) => return Ok(x as u8 as f64),
            Value::Int(x) => return Ok(x as f64),
            Value::Float(x) => return Ok(x),
            Value::String(x) => {
                match x.to_str().parse::<f64>() {
                    Ok(x) => return Ok(x),
                    Err(e) => return Err(OpErr::ParseFloatErr(e).into())
                }
            },

            Value::Table(mut val) => {
                if let Some(x) = val.meta_call_unary("__float", self) {
                    let x = x?;
                    return match x {
                        Value::Float(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Float, got: Type::of_val(&x).into() }.into())
                    };
                }
            },
            Value::UserData(mut val) => {
                if let Some(x) = val.meta_call_unary("__float", self) {
                    let x = x?;
                    return match x {
                        Value::Float(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Float, got: Type::of_val(&x) }.into())
                    };
                }
            },
            _ => {}
        }
        Err(OpErr::UnaryTypeErr { op: UnaryOp::ToFloat, ty: Type::of_val(&val) }.into())
    }

    pub fn to_str(&mut self,val:Value) -> Result<Gc<LuaString>> {
        match val {
            Value::Nil => return Ok(LuaString::new("nil").alloc()),
            Value::Bool(x) => return Ok(LuaString::new(if x {"true"} else {"false"}).alloc()),
            Value::Int(x) => return Ok(LuaString::new(&format!("{}",x)).alloc()),
            Value::Float(x) => return Ok(LuaString::new(&format!("{}",x)).alloc()),
            Value::String(x) => return Ok(x),

            Value::Table(mut val) => {
                if let Some(x) = val.meta_call_unary("__str", self) {
                    let x = x?;
                    return match x {
                        Value::String(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Float, got: Type::of_val(&x).into() }.into())
                    };
                } else {
                   return Ok(LuaString::new(&format!("Table {:#x}",val.addres())).alloc());
                }
            },
            Value::UserData(mut val) => {
                if let Some(x) = val.meta_call_unary("__str", self) {
                    let x = x?;
                    return match x {
                        Value::String(x) => Ok(x),
                        _ => Err(OpErr::CastMetaFuncReturnedWrongType { expected: Type::Float, got: Type::of_val(&x) }.into())
                    };
                } else {
                    return Ok(LuaString::new(&format!("UserData {:#x}",val.addres())).alloc());
                }
            },

            Value::Function(x) => return Ok(LuaString::new(&format!("Function {:#x}",(*x).addres())).alloc()),

            Value::UpValue(_) => unreachable!()
        }
    }
}