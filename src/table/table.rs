use fnv::{FnvBuildHasher, FnvHashMap};

use crate::{gc::{Gc, MarkDown, Marked, TableAlloc}, value::Value, vm::Vm, Result};
use super::key::Key;

#[derive(Clone,Debug)]
pub struct Table {
    meta_table_bits:usize,
    pub map:FnvHashMap<Key,Value>,
    pub array:Vec<Value>,
}


impl Table {
    pub fn new() -> Self {
        Self {
            meta_table_bits:0,
            map: FnvHashMap::default(),
            array: vec![] 
        }
    }

    pub fn with_capacity(array_cap:usize,map_cap:usize) -> Self {
        Self {
            meta_table_bits:0,
            map: FnvHashMap::with_capacity_and_hasher(map_cap, FnvBuildHasher::default()),
            array: Vec::with_capacity(array_cap)
        }
    }

    pub fn alloc(self) -> Gc<Self> {
        TableAlloc::alloc(self)
    }

    pub fn set(&mut self,key:Value,val:Value) -> Result<()> {
        if let Value::Int(i) = key {
            if i > 0 && (i  as usize) < self.array.len() {
                self.array[i as usize] = val;
                return Ok(());
            }
        }
        let key = Key::new_clone(key)?;
        self.map.insert(key,val);
        Ok(())
    }

    pub fn get(&self,key:Value) -> Result<Value> {
        if let Value::Int(i) = key {
            if i > 0 && (i  as usize) < self.array.len() {
                return Ok(self.array[i as usize].clone());
            }
        }
        let key = Key::new(key)?;
        Ok(self.map.get(&key).unwrap_or(&Value::Nil).clone())
    }

    pub fn get_str(&self,str:&str) -> Value {
        let key = Key::new_from_str(str);
        self.map.get(&key).unwrap_or(&Value::Nil).clone()
    }

    pub fn push(&mut self,val:Value) {
        self.array.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.array.pop().unwrap()
    }

    pub fn get_meta_table(&self) -> Option<Gc<Table>> {
        if self.meta_table_bits & !1 == 0 {
            return None;
        }
        Some(unsafe {
            std::mem::transmute(self.meta_table_bits & !1)
        })
    }

    pub fn set_meta_table(&mut self,meta:Gc<Table>) {
        let bits:usize = unsafe {
            std::mem::transmute(meta)
        };
        self.meta_table_bits |= bits & !1;
    }

    pub fn meta_call(&mut self,rhs:Value,name:&str,vm:&mut Vm) -> Option<Result<Value>> {
        let meta = self.get_meta_table()?;
        let val = meta.get_str(name);
        if let Value::Function(func) = val {
            let r0 = vm.regs[0];
            let r1 = vm.regs[1]; 
            match vm.call_meta(&func) {
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
        let meta = self.get_meta_table()?;
        let val = meta.get_str(name);
        if let Value::Function(func) = val {
            let r0 = vm.regs[0];
            match vm.call_meta_unary(&func) {
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

impl Marked for Table {
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

impl MarkDown for Table {
    fn mark_down(&mut self,vm:&mut Vm) {
        if self.is_marked() {return;}
        self.mark();
        if let Some(mut meta) = self.get_meta_table() {
            meta.mark_down(vm);
        }
        self.array.iter_mut().for_each(|val| val.mark_down(vm));
        self.map.iter_mut().for_each(|(k,v)| {
            v.mark_down(vm);
        });
    }
}


#[test]
fn test() {
    println!("{}",std::mem::size_of::<Table>());
}

