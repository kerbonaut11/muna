use crate::bytecode::{ByteCode, Mem};

pub struct Program {
    slice:Box<[ByteCode]>,
    pub ptr:*const ByteCode,
}


impl Program {
    pub fn new(program:Vec<ByteCode>) -> Self {
        let ptr = program.as_ptr();
        Self { 
            slice: program.into_boxed_slice(),
            ptr,
        }
    }

    pub fn next(&mut self) -> ByteCode {
        unsafe {
            let bc = *self.ptr;
            self.ptr = self.ptr.add(1);
            bc
        }
    }

    pub fn load_u64(&mut self) -> u64 {
        unsafe {
            let low = *self.ptr;
            let high = *self.ptr.add(1);
            self.ptr = self.ptr.add(2);
            std::mem::transmute((low,high))
        }
    }

    pub fn load_int(&mut self) -> i64 {
        unsafe {
            std::mem::transmute(self.load_u64())
        }
    }

    pub fn load_mem(&mut self) -> Mem {
        unsafe {
            let x = *(self.ptr as *const u32);
            self.ptr = self.ptr.add(1);
            Mem(x as u16)
        }
    }

    pub fn load_float(&mut self) -> f64 {
        unsafe {
            std::mem::transmute(self.load_u64())
        }
    }

    pub fn ptr_offset(&self,offset:usize) -> *const ByteCode {
        unsafe {
            std::mem::transmute(&self.slice[offset])
        }
    }
}