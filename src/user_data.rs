use std::{any::Any, fmt::Debug, mem};

use crate::{gc::{Gc, MarkDown, Marked}, table::Table, value::Value, vm::Vm, Result};

pub struct UserData {
    val:*mut dyn Any,
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
    pub fn meta_call(&mut self,rhs:Value,name:&str,vm:&mut Vm) -> Option<Result<Value>> {
        let val = self.get_meta_table()?.get_str(name);
        if let Value::Function(func) = val {
            let r0 = vm.regs[0];
            let r1 = vm.regs[1]; 
            match vm.call(2, 1, &func) {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }
            let val = vm.regs[0];
            vm.regs[0] = r0;
            vm.regs[1] = r1;
            Some(Ok(val))
        } else {
            None
        }
    }

    pub fn meta_call_unary(&mut self,name:&str,vm:&mut Vm) -> Option<Result<Value>> {
        let val = self.get_meta_table()?.get_str(name);
        if let Value::Function(func) = val {
            let r0 = vm.regs[0];
            match vm.call(1, 1, &func) {
                Ok(_) => {},
                Err(e) => return Some(Err(e))
            }
            let val = vm.regs[0];
            vm.regs[0] = r0;
            Some(Ok(val))
        } else {
            None
        }
    }  
}

