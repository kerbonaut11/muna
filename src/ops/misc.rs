use crate::function::Function;
use crate::gc::Gc;
use crate::string::LuaString;
use crate::table::Table;
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

    pub fn get(&mut self,t:Value,k:Value) -> Result<Value> {
        match t {
            Value::Table(mut t) => {
                let val = t.get(k)?;
                if !val.is_nil() {return Ok(val);}

                let meta = match t.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                let val = meta.get(k)?;
                if !val.is_nil() {return Ok(val);}

                match t.meta_call(k,"__idx", self) {
                    Some(result) => result,
                    None => Ok(Value::Nil)
                }
            }

            Value::UserData(mut ud) => {

                let val = unsafe{match k {
                    Value::String(k) => (*ud.val).gets(k, self)?,
                    Value::Int(k) => (*ud.val).geti(k, self)?,
                    _ => (*ud.val).get(k, self)?,
                }};
                if !val.is_nil() {return Ok(val);}

                let meta = match ud.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                meta.get(k)
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn gets(&mut self,t:Value,k:Gc<LuaString>) -> Result<Value> {
        match t {
            Value::Table(mut t) => {
                let val = t.get_str(k.to_str());
                if !val.is_nil() {return Ok(val);}

                let meta = match t.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                let val = meta.get_str(k.to_str());
                if !val.is_nil() {return Ok(val);}

                match t.meta_call(k.into(),"__idx", self) {
                    Some(result) => result,
                    None => Ok(Value::Nil)
                }
            }

            Value::UserData(mut ud) => {

                let val = unsafe{
                    (*ud.val).gets(k, self)?
                };
                if !val.is_nil() {return Ok(val);}

                let meta = match ud.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                Ok(meta.get_str(k.to_str()))
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn geti(&mut self,t:Value,k:i64) -> Result<Value> {
        match t {
            Value::Table(mut t) => {
                let val = t.geti(k);
                if !val.is_nil() {return Ok(val);}

                let meta = match t.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                let val = meta.geti(k);
                if !val.is_nil() {return Ok(val);}

                match t.meta_call(k.into(),"__idx", self) {
                    Some(result) => result,
                    None => Ok(Value::Nil)
                }
            }

            Value::UserData(mut ud) => {

                let val = unsafe{
                    (*ud.val).geti(k, self)?
                };
                if !val.is_nil() {return Ok(val);}

                let meta = match ud.get_meta_table() {
                    Some(meta) => meta,
                    None => return Ok(Value::Nil)
                };
                Ok(meta.geti(k))
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn set(&mut self,t:Value,k:Value,v:Value) -> Result<()> {
        match t {
            Value::Table(mut t) => {
                let prev = t.set(k,v)?;
                if let None = prev {
                    if let Some(result) =  t.meta_call_newidx(k,v, self) {result?};
                }
                Ok(())
            }

            Value::UserData(mut ud) => {
                unsafe{match k {
                    Value::String(k) => (*ud.val).sets(k, v, self),
                    Value::Int(k) => (*ud.val).seti(k, v,self),
                    _ => (*ud.val).set(k, v, self),
                }}
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn sets(&mut self,t:Value,k:Gc<LuaString>,v:Value) -> Result<()> {
        match t {
            Value::Table(mut t) => {
                let prev = t.set_str(k.to_str(),v)?;
                if let None = prev {
                    if let Some(result) =  t.meta_call_newidx(k.into(),v, self) {result?};
                }
                Ok(())
            }

            Value::UserData(mut ud) => {
                unsafe{(*ud.val).sets(k, v, self)}
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn seti(&mut self,t:Value,k:i64,v:Value) -> Result<()> {
        match t {
            Value::Table(mut t) => {
                let prev = t.seti(k,v)?;
                if let None = prev {
                    if let Some(result) =  t.meta_call_newidx(k.into(),v, self) {result?};
                }
                Ok(())
            }

            Value::UserData(mut ud) => {
                unsafe{(*ud.val).seti(k, v, self)}
            }

            _ => Err(OpErr::IndexedInvalidType(Type::of_val(&t)).into())
        }
    }

    pub fn bool_and(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        Ok(self.truthy(lhs)? && self.truthy(rhs)?)
    }

    pub fn bool_or(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        Ok(self.truthy(lhs)? && self.truthy(rhs)?)
    }

    pub fn bool_not(&mut self,val:Value) -> Result<bool> {
        Ok(!self.truthy(val)?)
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

    pub fn get_meta_table(&mut self) -> Result<Value> {
        let opt_t = match self {
            Value::Table(t) => t.get_meta_table(),
            Value::UserData(ud) => ud.get_meta_table(),
            _ => todo!()
        };
        Ok(match opt_t {
            Some(t) => Value::Table(t),
            None => Value::Nil,
        })
    }

    pub fn set_meta_table(&mut self,t:Value) -> Result<()> {
        let meta = match t {
            Value::Table(t) => t,
            _ => panic!("not a table: {:?}",t),
        };
        match self {
            Value::Table(t) => t.set_meta_table(meta),
            Value::UserData(ud) => ud.set_meta_table(meta),
            _ => panic!("cannot set mt on {:?}",self)
        }
        Ok(())
    }
}
