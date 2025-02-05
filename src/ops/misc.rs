use crate::function::Function;
use crate::upval::UpVal;
use crate::vm::Vm;
use crate::Result;
use crate::value::{Type, Value};
use super::OpErr;

impl Vm {
    pub fn len(&mut self,val:Value) -> Result<Value> {
        match val {
            Value::String(x) => Ok(Value::Int(x.len() as i64)),
            Value::Table(x) => Ok(Value::Int(x.array.len() as i64)),
            _ => Err(OpErr::UnaryTypeErr { op: super::err::UnaryOp::Len, ty: Type::of_val(&val) }.into())
        }
    }

    pub fn truthy(&mut self,val:Value) -> Result<bool> {
        match val {
            Value::Nil => Ok(false),
            Value::Bool(x) => Ok(x),
            Value::Table(mut val) => if let Some(x) = val.meta_call_unary("__tty", self) {return try_to_bool(x?);} else {Ok(true)},
            Value::UserData(mut val) => if let Some(x) = val.meta_call_unary("__tty", self) {return try_to_bool(x?);} else {Ok(true)},
            _ => Ok(true)
        }
    }
}

fn try_to_bool(val:Value) -> Result<bool> {
    match val {
        Value::Bool(x) => Ok(x),
        _ => Err(OpErr::TruthyMetaFuncReturnedNonBool { got: Type::of_val(&val) }.into())
    }
}


impl Value {
    pub fn open_upval(&mut self) {
        UpVal::open(self);
    }

    pub fn get_upval(&self) -> Value {
        if let Value::UpValue(up_val) = *self {
            up_val.get()
        } else { unreachable!() }
    }

    pub fn set_upval(&mut self,val:Value) {
        if let Value::UpValue(mut up_val) = *self {
            up_val.set(val);
        } else { unreachable!() }
    }

    fn addres(&self) -> Option<usize> {
        match self {
            Value::String(gc) => Some(gc.addres()),
            Value::Table(gc) => Some(gc.addres()),
            Value::Function(gc) => Some(gc.addres()),
            Value::UserData(gc) => Some(gc.addres()),
            _ => None
        }
    }

    pub fn is(&self,b:&Value) -> bool {
        let a = match self.addres() {
            Some(a) => a,
            None => return false
        };
        let b = match b.addres() {
            Some(b) => b,
            None => return false
        };
        a == b
    }
}
