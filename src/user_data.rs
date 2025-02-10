use std::{any::Any, fmt::Debug, mem};

use crate::{gc::{Gc, MarkDown, Marked}, ops::OpErr, string::LuaString, table::Table, value::{Type, Value}, vm::Vm, Result};

pub trait IUserData: Any {
    fn get (&mut self, key:Value,        vm: &mut Vm) -> Result<Value> {Err(OpErr::InvalidUdIndex(Type::of_val(&key)).into())}
    fn gets(&mut self, key:Gc<LuaString>,vm: &mut Vm) -> Result<Value> {self.get(key.into(), vm)}
    fn geti(&mut self, key:i64,          vm: &mut Vm) -> Result<Value> {self.get(key.into(), vm)}

    fn set (&mut self, key:Value,        val:Value,vm: &mut Vm) -> Result<()> {Err(OpErr::InvalidUdIndex(Type::of_val(&key)).into())}
    fn sets(&mut self, key:Gc<LuaString>,val:Value,vm: &mut Vm) -> Result<()> {self.set(key.into(), val,vm)}
    fn seti(&mut self, key:i64,          val:Value,vm: &mut Vm) -> Result<()> {self.set(key.into(), val,vm)}
}


pub struct UserData {
    pub val:*mut dyn IUserData,
    meta_table_bits:u64,
}

impl UserData {
    pub fn get_meta_table(&self) -> Option<Gc<Table>> {
        if self.meta_table_bits & !1 == 0 {
            None
        } else {
            Some(unsafe{mem::transmute(self.meta_table_bits)})
        }
    }

    pub fn set_meta_table(&mut self,t:Gc<Table>) {
        let meta_table_bits:u64 = unsafe{mem::transmute(t)};
        self.meta_table_bits &= 1;
        self.meta_table_bits |= meta_table_bits;
    }
}

impl Marked for UserData {
    fn is_marked(&self) -> bool {
        self.meta_table_bits & 1 == 1
    }

    fn mark(&mut self) {
        self.meta_table_bits |= 1;
    }

    fn unmark(&mut self) {
        self.meta_table_bits &= !1;
    }
}

impl MarkDown for UserData {
    fn mark_down(&mut self,vm:&mut Vm) {
        if self.is_marked() {return;}
        self.mark();
        let _ = self.meta_call_unary("__gc",vm);
    }
}

impl Debug for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}",self.val))?;
        Ok(())
    }
}

impl PartialEq for UserData {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::addr_eq(self.val, other.val)
    }
}

impl Eq for UserData {}

impl UserData {
    pub fn meta_call_unary(&mut self,name:&str,vm:&mut Vm) -> Option<Result<Value>> {
        let meta = self.get_meta_table()?;
        let val = meta.get_str(name);
        if let Value::Function(func) = val {

            let args = vm.regs.arg_regs();
            vm.regs[0] = unsafe{Gc::<Self>::from_ref(self)}.into();

            match vm.call_meta_unary(&func) {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }

            let val = vm.regs[0];
            vm.regs.write_arg_regs(args);
            Some(Ok(val))
        } else {
            None
        }
    }

    pub fn meta_call(&mut self,rhs:Value,name:&str,vm:&mut Vm) -> Option<Result<Value>> {
        let meta = self.get_meta_table()?;
        let val = meta.get_str(name);
        if let Value::Function(func) = val {

            let args = vm.regs.arg_regs();
            vm.regs[0] = unsafe{Gc::<Self>::from_ref(self)}.into();
            vm.regs[1] = rhs;

            match vm.call_meta(&func) {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }

            let val = vm.regs[0];
            vm.regs.write_arg_regs(args);
            Some(Ok(val))
        } else {
            None
        }
    }

    pub fn meta_call_set(&mut self,k:Value,v:Value,vm:&mut Vm) -> Option<Result<()>> {
        let meta = self.get_meta_table()?;
        let val = meta.get_str("__set");
        if let Value::Function(func) = val {

            let args = vm.regs.arg_regs();
            vm.regs[0] = unsafe{Gc::<Self>::from_ref(self)}.into();
            vm.regs[1] = k;
            vm.regs[2] = v;

            match vm.call_meta_new_idx(&func) {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }

            let val = vm.regs[0];
            vm.regs.write_arg_regs(args);
            Some(Ok(()))
        } else {
            None
        }
    }

    pub fn set(&mut self,k:Value,v:Value,vm:&mut Vm) -> Result<()> {
        todo!()
    }
}

