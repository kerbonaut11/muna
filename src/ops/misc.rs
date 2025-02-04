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

    pub fn truty(&mut self,val:Value) -> Result<bool> {
        todo!()
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

    pub fn addres(&self) -> Option<usize> {
        match self {
            Value::String(gc) => Some(gc.addres()),
            Value::Table(gc) => Some(gc.addres()),
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
