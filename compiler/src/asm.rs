use std::{collections::HashMap, io::{Read, Write}, num::NonZeroU32, ptr::slice_from_raw_parts};

use crate::bytecode::{self, ByteCode, ClosureArgs};

pub struct ByteCodeVec{
    code:Vec<(ByteCode,Option<LabelId>)>
}

impl ByteCodeVec {
    pub fn new() -> Self {
        Self {
            code:vec![]
        }
    }

    pub fn add_instr(&mut self,x:ByteCode) {
        self.code.push((x,None));
    }

    pub fn add_instr_at(&mut self,x:ByteCode,i:usize) {
        self.code[i].0 = x;
    }

    pub fn add_label(&mut self,l:LabelId) {
        self.code.last_mut().unwrap().1 = Some(l);
    }

    pub fn add_label_at(&mut self,l:LabelId,i:usize) {
        self.code[i].1 = Some(l);
    }

    pub fn append(&mut self,other: &mut Self) {
        self.code.append(&mut other.code);
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn print(&self) {
        let mut i = 0;
        for (instr,label) in &self.code {
            println!("{}: {:?} ",i,instr);
            if let Some(label) = label {
                println!("label {:?}:",label.0);
            }
            i += instr_width(instr);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct LabelId(NonZeroU32);


pub struct CompileCtx {
    name_map:HashMap<Box<str>,u16>,
    next_label_id:NonZeroU32,
}

impl CompileCtx {
    pub fn new() -> Self {
        Self{
            name_map:HashMap::new(),
            next_label_id: NonZeroU32::new(1).unwrap(),
        }
    }

    pub fn new_label(&mut self) -> LabelId {
        let label = LabelId(self.next_label_id);
        self.next_label_id = self.next_label_id.checked_add(1).unwrap();
        label
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

    pub fn write_to_file(&self, bytecode:ByteCodeVec, path:&str) {
        use std::fs;

        _ = fs::remove_file(path);
        let mut f = fs::File::create_new(path).unwrap();
        self.encode_name_table(&mut f);

        let mut encoded_bc:Vec<u32> = Vec::with_capacity(bytecode.len()*2);
        let mut label_map:HashMap<LabelId,usize> = HashMap::with_capacity(bytecode.code.len());

        let mut bytecode_len = 0;
        for (instr,label) in &bytecode.code {
            bytecode_len += instr_width(&instr);
            if let Some(label) = label  {
                label_map.insert(*label, bytecode_len);   
            }
        }

        for (instr,_) in &bytecode.code {
            unsafe {
                use std::mem::transmute;
                let mut head = [0,0,0,0];
                head[2] = *(std::ptr::from_ref(instr) as *const u8);

                match *instr {
                    ByteCode::LoadInt(x) => {
                        encoded_bc.push(transmute(head));
                        encoded_bc.push(transmute(x));
                    }
        
                    ByteCode::LoadFloat(x) => {
                        encoded_bc.push(transmute(head));
                        encoded_bc.push(transmute(x));
                    }
        
                    ByteCode::Load(x) | ByteCode::Write(x) | ByteCode::LoadStr(x) | 
                    ByteCode::BindUpval(x) | ByteCode::GetUpval(x) | ByteCode::SetUpval(x) => {
                        head[0] = (x & 0xFF)as u8;
                        head[1] = (x >> 8)as u8;
                        encoded_bc.push(transmute(head));
                    }
        
                    ByteCode::Jump(x) | ByteCode::JumpFalse(x) | ByteCode::JumpTrue(x) => {
                        let offset = (*label_map.get(&x).unwrap() as i16) - (encoded_bc.len() as i16) - 1;
                        head[0] = (offset & 0xFF)as u8;
                        head[1] = (offset >> 8)as u8;
                        encoded_bc.push(transmute(head));
                    }
        
                    ByteCode::Less(x) | ByteCode::LessEq(x) | ByteCode::Eq(x) => {
                        head[0] = x as u8;
                        encoded_bc.push(transmute(head));
                    }
        
                    ByteCode::Closure( ClosureArgs{ label, upval_cap, arg_count }) => {
                        head[0] = upval_cap;
                        head[1] = arg_count;
                        let offset = (*label_map.get(&label).unwrap() as i32) - (encoded_bc.len() as i32) - 2;
                        encoded_bc.push(transmute(head));
                        encoded_bc.push(transmute(offset as u32));
                    }
        
                    _ => encoded_bc.push(transmute(head)),
                }
            }
        }

        let slice = unsafe {
            &*slice_from_raw_parts(encoded_bc.as_ptr() as *const u8, encoded_bc.len()*4)
        }; 

        f.write_all(slice).unwrap();
    }
}

fn instr_width(instr:&ByteCode) -> usize {
    match instr {
        ByteCode::LoadInt(_) | ByteCode::LoadFloat(_) | ByteCode::Closure(_) => 2,
        _ => 1
    }
}


#[test]
fn layout() {
    unsafe {
        println!("{:#x}",std::mem::transmute::<ByteCode,u64>(ByteCode::LoadInt(0xaaff)));
        assert_eq!(*(std::ptr::from_ref(&ByteCode::LoadInt(0xffaa)) as *const u8),3);
    }
}