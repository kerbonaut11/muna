use crate::bytecode::ByteCode;

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

    pub fn load_int(&mut self) -> i64 {
        unsafe {
            let x = *(self.ptr as *const i64);
            self.ptr = self.ptr.add(2);
            x
        }
    }

    pub fn load_float(&mut self) -> f64 {
        unsafe {
            let x = *(self.ptr as *const f64);
            self.ptr = self.ptr.add(2);
            x
        }
    }
}