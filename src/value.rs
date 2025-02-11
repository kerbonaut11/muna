use std::hash::Hash;
use std::mem;
use derive_more::derive::From;

use crate::function::{CallErr, Function};
use crate::gc::{Gc, MarkDown, Marked};
use crate::string::LuaString;
use crate::table::Table;
use crate::upval::UpVal;
use crate::user_data::UserData;
use crate::vm::Vm;
use crate::Error;

#[derive(Clone, Debug, Copy, From)]
#[repr(u8)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Gc<LuaString>),
    Table(Gc<Table>),
    Function(Gc<Function>),
    UpValue(Gc<UpVal>),
    UserData(Gc<UserData>),
}

impl Value {
    pub fn is_nil(&self) -> bool {
        if let Value::Nil = *self {true} else {false}
    }

    pub fn assert_bool(&self,b:bool) {
        match self {
            Value::Bool(a) => assert_eq!(*a,b),
            _ => panic!("assert failed:\n expected Bool got {:?}",Type::of_val(self))
        }
    }
}

impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(unsafe{*mem::transmute::<&Self,*const u8>(self)});
        match self {
            Value::Nil => {}
            Value::Bool(x) => state.write_u8(*x as u8),
            Value::Int(x) => state.write_i64(*x),
            Value::Float(x) => state.write_i64(*x as i64),
            Value::String(x) => state.write(x.to_str().as_bytes()),
            Value::Table(x) => state.write_usize(x.addres()),
            Value::UpValue(x) => todo!(),
            Value::Function(x) => (*x).hash(state),
            Value::UserData(x) => state.write_usize(x.addres()),
        }
    }
}

impl MarkDown for Value {
    fn mark_down(&mut self,vm:&mut Vm) {
        match self {
            Value::Table(x) => x.mark_down(vm),
            Value::String(x) => x.mark(),
            Value::UserData(x) => x.mark_down(vm),
            _ => {}
        }
    }
}

#[derive(Clone,PartialEq, Debug, Copy)]
#[repr(u8)]
pub enum Type {
    Nil,
    Bool,
    Int,
    Float,
    String,
    Table,
    Function,
    UserData,
}

impl Type {
    pub fn of_val(val:&Value) -> Self {
        match val {
            Value::Nil => Self::Nil,
            Value::Bool(_) => Self::Bool,
            Value::Int(_) => Self::Int,
            Value::Float(_) => Self::Float,
            Value::String(_) => Self::String,
            Value::Table(_) => Self::Table,
            Value::Function(_) => Self::Function,
            Value::UpValue(_) => unreachable!(),
            Value::UserData(_) => Type::UserData
        }
    }
}

impl TryInto<bool> for Value {
    type Error = Error;
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Bool(x) => Ok(x),
            _ => Err(CallErr::UdTypeMismatch { expected: Type::Bool, got: Type::of_val(&self) }.into())
        }
    }
}

impl TryInto<i64> for Value {
    type Error = Error;
    fn try_into(self) -> Result<i64, Self::Error> {
        match self {
            Value::Int(x) => Ok(x),
            Value::Float(x) => Ok(x as i64),
            _ => Err(CallErr::UdTypeMismatch { expected: Type::Bool, got: Type::of_val(&self) }.into())
        }
    }
}

impl TryInto<f64> for Value {
    type Error = Error;
    fn try_into(self) -> Result<f64, Self::Error> {
        match self {
            Value::Int(x) => Ok(x as f64),
            Value::Float(x) => Ok(x),
            _ => Err(CallErr::UdTypeMismatch { expected: Type::Bool, got: Type::of_val(&self) }.into())
        }
    }
}