use std::{fmt::Debug, mem};

use crate::{gc::{MarkDown, Marked, UpValAlloc}, value::Value, vm::Vm};

pub struct UpVal {
    bits:u128
}

impl UpVal {
    const MASK:u128 = 1<<127;
    pub fn open(val:&mut Value) {
        let up_val = UpVal{bits:unsafe{mem::transmute(*val)}};
        let gc = UpValAlloc::alloc(up_val);
        *val = Value::UpValue(gc);
    }

    pub fn get(&self) -> Value {
        unsafe{mem::transmute(self.bits & !(1<<127))}
    }

    pub fn set(&mut self,val:Value) {
        self.bits &= Self::MASK;
        self.bits |= unsafe{mem::transmute::<Value,u128>(val) & !Self::MASK};
    }
}

impl Marked for UpVal {
    fn is_marked(&self) -> bool {
        self.bits & Self::MASK != 0
    }

    fn mark(&mut self) {
        self.bits |= Self::MASK;
    }

    fn unmark(&mut self) {
        self.bits &= !Self::MASK;
    }
}

impl MarkDown for UpVal {
    fn mark_down(&mut self,vm: &mut Vm) {
        self.mark();
        self.get().mark_down(vm);
    }
}

impl Debug for UpVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get().fmt(f)
    }
}