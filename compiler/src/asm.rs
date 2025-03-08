use std::io::{Read, Write};

use crate::bytecode::ByteCode;

pub struct Assembler(Vec<u32>);

impl Assembler {
    pub fn new() -> Self {
        return Self(vec![]);
    }

    pub fn encode_instr(&mut self,x:ByteCode) {
        let array_in:[u8;4] = unsafe{std::mem::transmute(x)};
        let array_out = [array_in[2],array_in[3],array_in[0],array_in[1]]; 
        self.0.push(unsafe{std::mem::transmute(array_out)});
    }

    pub fn encode_int(&mut self,x:i32) {
        self.0.push(unsafe{std::mem::transmute(x)});
    }

    pub fn encode_float(&mut self,x:f32) {
        self.0.push(unsafe{std::mem::transmute(x)});
    }

    pub fn write_to_file(&self,path:&str) {
        use std::fs;
        _ = fs::remove_file(path);
        let mut f = fs::File::create_new(path).unwrap();
        _ = f.write_all(unsafe {
           let x = std::slice::from_raw_parts(std::mem::transmute(self.0.as_ptr()), self.0.len()*4);
           x 
        });
    }
}

pub fn create_test_file() {
    let mut asm = Assembler::new();
    asm.encode_instr(ByteCode::LoadTrue);
    asm.encode_instr(ByteCode::LoadInt);
    asm.encode_int(20);
    asm.encode_instr(ByteCode::LoadFalse);
    asm.encode_instr(ByteCode::LoadNil);
    asm.encode_instr(ByteCode::LoadFloat);
    asm.encode_float(0.1);
    asm.encode_instr(ByteCode::Load(1));
    asm.encode_instr(ByteCode::Add);
    asm.write_to_file("../test.out");
}
