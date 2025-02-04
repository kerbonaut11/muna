use std::{alloc::{alloc, dealloc, Layout}, mem, ops::{Index, IndexMut}, slice};
use crate::{bytecode::Mem, value::Value};

pub struct Stack {
    zero_ptr:*mut u8,
    pub top_ptr:*mut Value,
    pub base_ptr:*mut Value,
    size:usize
}

impl Stack {
    pub fn new(size:usize) -> Stack {
        unsafe {
            let mem = alloc(Layout::array::<Value>(size).unwrap());
            Self { 
                zero_ptr: mem, 
                base_ptr: (mem as *mut Value).add(1), 
                top_ptr: mem as *mut Value,
                size 
            }
        }
    }

    pub fn push(&mut self,val:Value) {
        unsafe {
            self.top_ptr = self.top_ptr.add(1);
            *self.top_ptr = val;
        }
    }

    pub fn pop(&mut self) -> Value {
        unsafe {
            let val = *self.top_ptr;
            self.top_ptr = self.top_ptr.sub(1);
            val
        }
    }

    pub fn top(&self) -> &Value {
        unsafe {
            mem::transmute( self.top_ptr)
        }
    }

    pub fn top_mut(&self) -> &mut Value {
        unsafe {
            mem::transmute( self.top_ptr)
        }
    }

    pub fn slice(&self) -> &[Value] {
        unsafe{slice::from_raw_parts(
            self.base_ptr, 
            (self.top_ptr as usize - self.base_ptr as usize)/std::mem::size_of::<Value>()+1
        )}
    }

    pub fn slice_total(&self) -> &[Value] {
        unsafe{slice::from_raw_parts(
            self.zero_ptr as *mut Value, 
            (self.top_ptr as usize - self.zero_ptr as usize)/std::mem::size_of::<Value>()+1
        )}
    }
}


impl Index<Mem> for Stack {
    type Output = Value;
    fn index(&self, index: Mem) -> &Self::Output {
        unsafe {
            mem::transmute(self.base_ptr.add(index.0 as usize))
        }
    }
}

impl IndexMut<Mem> for Stack {
    fn index_mut(&mut self, index: Mem) -> &mut Self::Output {
        unsafe {
            mem::transmute(self.base_ptr.add(index.0 as usize))
        }
    }
}

impl Index<usize> for Stack {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            mem::transmute(self.base_ptr.add(index))
        }
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe {
            mem::transmute(self.base_ptr.add(index))
        }
    }
}



impl Drop for Stack {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.zero_ptr, Layout::array::<Value>(self.size).unwrap());
        }
    }
}
