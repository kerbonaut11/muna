use std::{collections::HashMap, io::{Read, Write}};

use crate::bytecode::ByteCode;

pub struct Assembler{
    bytecode:Vec<u32>,
    name_map:HashMap<Box<str>,u16>
}

impl Assembler {
    pub fn new() -> Self {
        Self{
            bytecode:vec![],
            name_map:HashMap::new()
        }
    }

    pub fn encode_instr(&mut self,x:ByteCode) {
        let array_in:[u8;4] = unsafe{std::mem::transmute(x)};
        let array_out = [array_in[2],array_in[3],array_in[0],array_in[1]]; 
        self.bytecode.push(unsafe{std::mem::transmute(array_out)});
    }

    pub fn encode_int(&mut self,x:i32) {
        self.bytecode.push(unsafe{std::mem::transmute(x)});
    }

    pub fn encode_float(&mut self,x:f32) {
        self.bytecode.push(unsafe{std::mem::transmute(x)});
    }

    pub fn get_idx_of_name(&mut self,name:&str) -> u16 {
        if let Some(idx) = self.name_map.get(name) {
            return *idx;
        }

        let idx = self.name_map.len();
        self.name_map.insert(name.into(), idx as u16);
        idx as u16
    }

    pub fn encode_name_table(&self,f:&mut std::fs::File) {
        let low = (self.name_map.len() & 0xFF) as u8;
        let high = ((self.name_map.len() >> 8) & 0xFF) as u8;
        f.write(&[low,high]).unwrap();

        for (name,_) in &self.name_map {
            f.write_all(name.as_bytes()).unwrap();
            f.write_all(&[0]).unwrap();
        }
    }

    pub fn write_to_file(&self,path:&str) {
        use std::fs;

        _ = fs::remove_file(path);
        let mut f = fs::File::create_new(path).unwrap();
        self.encode_name_table(&mut f);

        _ = f.write_all(unsafe {
           let x = std::slice::from_raw_parts(std::mem::transmute(self.bytecode.as_ptr()), self.bytecode.len()*4);
           x 
        });
    }

    pub fn print(&self) {
        let mut i = 0;
        loop {
            if i == self.bytecode.len() {
                return;
            }

            let instr:ByteCode = decode_instr_to_u32(self.bytecode[i]);
            print!("{:?}",instr);

            match instr {
                ByteCode::LoadInt => {
                    i += 1;
                    let x:i32 = unsafe{std::mem::transmute(self.bytecode[i])};
                    print!("({})",x);
                }

                ByteCode::LoadFloat => {
                    i += 1;
                    let x:f32 = unsafe{std::mem::transmute(self.bytecode[i])};
                    print!("({})",x);
                }

                _ => {}
            }

            println!("");

            i += 1;
        }
    }
}


fn encode_instr_to_u32(x:ByteCode) -> u32 {
   unsafe{std::mem::transmute::<ByteCode,u32>(x)}.rotate_right(16)
}

fn decode_instr_to_u32(x:u32) -> ByteCode {
    unsafe{std::mem::transmute::<u32,ByteCode>(x.rotate_right(16))}
 }