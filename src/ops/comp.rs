use crate::value::Value;
use crate::vm::Vm;
use crate::Result;

impl Vm {
    pub fn eq(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        todo!()
    }


    pub fn less(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        todo!()
    }

    pub fn less_eq(&mut self,lhs:Value,rhs:Value) -> Result<bool> {
        todo!()
    }

    pub fn eqi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        todo!()
    }

    pub fn lessi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        todo!()
    }

    pub fn less_eqi(&mut self,lhs:Value,rhs:i64) -> Result<bool> {
        todo!()
    }

    pub fn eqf(&mut self,lhs:Value,rhs:f64) -> Result<bool> {
        todo!()
    }

    pub fn lessf(&mut self,lhs:Value,rhs:f64)-> Result<bool> {
        todo!()
    }

    pub fn less_eqf(&mut self,lhs:Value,rhs:f64) -> Result<bool> {
        todo!()
    }
}
