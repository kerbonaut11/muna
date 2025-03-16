use std::{collections::HashMap, io::{Read, Write}};

use crate::bytecode::ByteCode;

pub struct ByteCodeVec{
    pub raw:Vec<u32>,
}

impl ByteCodeVec {
    pub fn new() -> Self {
        return Self{raw:vec![]};
    }

    pub fn encode_instr(&mut self,x:ByteCode) {
        println!("{}:{:?}",self.raw.len(),x);
        self.raw.push(encode_instr_to_u32(x));
    }

    pub fn encode_int(&mut self,x:i32) {
        self.raw.push(unsafe{std::mem::transmute(x)});
    }

    pub fn encode_float(&mut self,x:f32) {
        self.raw.push(unsafe{std::mem::transmute(x)});
    }

    pub fn encode_instr_at(&mut self,x:ByteCode,i:usize) {
        println!("{}:{:?}",i,x);
        self.raw[i] = encode_instr_to_u32(x);
    }

    pub fn encode_int_at(&mut self,x:i32,i:usize) {
        self.raw[i] = unsafe{std::mem::transmute(x)};
    }

    pub fn encode_float_at(&mut self,x:f32,i:usize) {
        self.raw[i] = unsafe{std::mem::transmute(x)};
    }

    pub fn append(&mut self,other: &mut Self) {
        self.raw.append(&mut other.raw);
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }
}


pub struct CompileCtx {
    name_map:HashMap<Box<str>,u16>
}

impl CompileCtx {
    pub fn new() -> Self {
        Self{
            name_map:HashMap::new()
        }
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
        f.write_all(&[low,high]).unwrap();

        let mut names = self.name_map.iter().collect::<Vec<(&Box<str>,&u16)>>();
        names.sort_by_key(|x| x.1);
        for (name,_) in names {
            f.write_all(name.as_bytes()).unwrap();
            f.write_all(&[0]).unwrap();
        }
    }

    pub fn write_to_file(&self,bytecode:&ByteCodeVec,path:&str) {
        use std::fs;

        _ = fs::remove_file(path);
        let mut f = fs::File::create_new(path).unwrap();
        self.encode_name_table(&mut f);

        _ = f.write_all(unsafe {
           let x = std::slice::from_raw_parts(std::mem::transmute(bytecode.raw.as_ptr()), bytecode.len()*4);
           x 
        });
    }
}


fn encode_instr_to_u32(x:ByteCode) -> u32 {
    use std::ptr;
    unsafe {
        let mut out = [0,0,0,0];
        out[2] = *(ptr::from_ref(&x) as *const u8);
        match x {
            ByteCode::Load(x) | ByteCode::Write(x) | ByteCode::LoadStr(x) | 
            ByteCode::BindUpval(x) | ByteCode::GetUpval(x) | ByteCode::SetUpval(x) => {
                out[0] = (x & 0xFF)as u8;
                out[1] = (x >> 8)as u8;
            }

            ByteCode::Closure { upval_cap, arg_count } => {
                out[0] = upval_cap;
                out[1] = arg_count;
            }

            _ => {},
        }
        std::mem::transmute(out)
    }
}

#[test]
fn layout_test() {
    //110102
    //7ffaa
    let x:u32 = unsafe{std::mem::transmute(ByteCode::Load(0xffaa))};
    println!("{:x}",x);
    println!("{:x}",encode_instr_to_u32(ByteCode::Load(0xffaa)));
    let x:u32 = unsafe{std::mem::transmute(ByteCode::Closure { arg_count: 1, upval_cap: 2 })};
    println!("{:x}",x);
    println!("{:x}",encode_instr_to_u32(ByteCode::Closure { arg_count: 1, upval_cap: 2 }));
}