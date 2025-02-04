use std::fmt::Debug;
use std::hash::Hash;
use std::str;

use crate::function::Function;
use crate::gc::Gc;
use crate::user_data::UserData;
use crate::value::Value;
use crate::{Error,Result};

use super::Table;
use super::raw_str_slice::RawStrSlice;

#[derive(Clone, Debug)]
#[repr(u8)]
pub enum Key {
    Bool(bool),
    Int(i64),
    FloatBytes(u64),
    String(RawStrSlice),
    Table(Gc<Table>),
    Function(Gc<Function>),
    UserData(Gc<UserData>),
}

#[derive(Debug)]
pub enum InvalidKey {
    Nil,
    Nan,
}

impl Key {

    pub fn new(val: Value) -> Result<Self> {
        match val {
            Value::Nil => Err(Error::OpErr(InvalidKey::Nil.into())),
            Value::Bool(x) => Ok(Key::Bool(x)),
            Value::Int(x) => Ok(Key::Int(x)),
            Value::Float(x) => match x.is_nan() {
                false => Ok(Key::FloatBytes(unsafe{std::mem::transmute(x)})),
                true => Err(Error::OpErr(InvalidKey::Nan.into()))
            }
            Value::String(x) => Ok(Key::String(RawStrSlice::new_temp(x.to_str()))),
            Value::Table(x) => Ok(Key::Table(x)),
            Value::UpValue(_) => unreachable!(),
            Value::Function(x) => Ok(Key::Function(x)),
            Value::UserData(x) => Ok(Key::UserData(x))
        }
    }
    
    pub fn new_clone(val: Value) -> Result<Self> {
        match val {
            Value::Nil => Err(Error::OpErr(InvalidKey::Nil.into())),
            Value::Bool(x) => Ok(Key::Bool(x)),
            Value::Int(x) => Ok(Key::Int(x)),
            Value::Float(x) => match x.is_nan() {
                false => {
                    if x % 1.0 == 0.0 {
                        Ok(Key::Int(x as i64))
                    } else {
                        Ok(Key::FloatBytes(unsafe{std::mem::transmute(x)}))
                    }
                },
                true => Err(Error::OpErr(InvalidKey::Nan.into()))
            }
            Value::String(x) => Ok(Key::String(RawStrSlice::new_clone(x.to_str()))),
            Value::Table(x) => Ok(Key::Table(x)),
            Value::UpValue(_) => unreachable!(),
            Value::Function(x) => Ok(Key::Function(x)),
            Value::UserData(x) => Ok(Key::UserData(x))
        }
    }

    pub fn new_from_str(str:&str) -> Self {
        Self::String(RawStrSlice::new_temp(str))
    }

    pub fn clone_from_str(str:&str) -> Self {
        Self::String(RawStrSlice::new_clone(str))
    }
}


impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        let a = self;
        let b = other;
        match a {
            Key::Bool(a) => if let Key::Bool(b) = b {a==b} else {false},
            Key::Int(a) => if let Key::Int(b) = b {a==b} else {false},
            Key::FloatBytes(a) => if let Key::FloatBytes(b) = b {a==b} else {false},
            Key::String(a) => if let Key::String(b) = b {a==b} else {false},
            Key::Table(a) => if let Key::Table(b) = b {a.addres()==b.addres()} else {false},
            Key::Function(a) => if let Key::Function(b) = b {a.addres()==b.addres()} else {false},
            Key::UserData(a) => if let Key::UserData(b) = b {a.addres()==b.addres()} else {false},
        }
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(unsafe{*std::mem::transmute::<&Self,*const u8>(self)});
        match self {
            Key::Bool(x) => state.write_u8(*x as u8),
            Key::Int(x) => state.write_i64(*x),
            Key::FloatBytes(x) => state.write_i64(*x as i64),
            Key::String(x) => state.write(x.slice()),
            Key::Table(x) => state.write_usize(x.addres()),
            Key::Function(x) => (*x).hash(state),
            Key::UserData(x) => state.write_usize(x.addres()),
        }
    }
}

