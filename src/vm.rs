use std::ops::{Index, IndexMut};

use crate::{bytecode::{ByteCode, Mem, Reg}, exec::exec, function::CallStack, gc::Gc, program::Program, stack::Stack, string::LuaString, value::Value, Result};

pub struct Vm {
    pub program:Program,
    pub stack:Stack,
    pub regs:Regs,
    pub name_table:Box<[LuaString]>,

    pub call_stack:CallStack,
}

impl Vm {

    pub fn reg_reg_op(&mut self,dest:Reg,src:Reg,op:fn(&mut Self,Value,Value) -> Result<Value>) -> Result<()> {
        self.regs[dest] = op(self,self.regs[dest],self.regs[src])?;
        Ok(())
    }

    pub fn reg_mem_op(&mut self,dest:Reg,src:Mem,op:fn(&mut Self,Value,Value) -> Result<Value>) -> Result<()> {
        self.regs[dest] = op(self,self.regs[dest],self.stack[src])?;
        Ok(())
    }

    pub fn reg_int_op(&mut self,dest:Reg,op:fn(&mut Self,Value,i64) -> Result<Value>) -> Result<()> {
        let x = self.program.load_int();
        self.regs[dest] = op(self,self.regs[dest],x)?;
        Ok(())
    }

    pub fn reg_float_op(&mut self,dest:Reg,op:fn(&mut Self,Value,f64) -> Result<Value>) -> Result<()> {
        let x = self.program.load_float();
        self.regs[dest] = op(self,self.regs[dest],x)?;
        Ok(())
    }

    pub fn mem_reg_op(&mut self,dest:Mem,src:Reg,op:fn(&mut Self,Value,Value) -> Result<Value>) -> Result<()> {
        self.stack[dest] = op(self,self.stack[dest],self.regs[src])?;
        Ok(())
    }

    pub fn mem_int_op(&mut self,dest:Mem,op:fn(&mut Self,Value,i64) -> Result<Value>) -> Result<()> {
        let x = self.program.load_int();
        self.stack[dest] = op(self,self.stack[dest],x)?;
        Ok(())
    }

    pub fn mem_float_op(&mut self,dest:Mem,op:fn(&mut Self,Value,f64) -> Result<Value>) -> Result<()> {
        let x = self.program.load_float();
        self.stack[dest] = op(self,self.stack[dest],x)?;
        Ok(())
    }
}

impl Vm {
    pub fn new(bytecode:Vec<ByteCode>,name_table:Box<[LuaString]>) -> Self {
        Self { 
            program: Program::new(bytecode),
            stack: Stack::new(100_000),
            regs: Regs::new(),
            name_table,

            call_stack: vec![],
        }
    }

    pub fn exec(&mut self) -> Result<()> {
        let instr = self.program.next();
        exec(self, instr)
    }

    pub fn get_name(&self,i:Mem) -> Gc<LuaString> {
        let _ref = &self.name_table[i.0 as usize];
        unsafe{std::mem::transmute(_ref)}
    }

    pub fn skip_if(&mut self, cond: bool,offset: i8) {
        if cond {unsafe{
            self.program.ptr = self.program.ptr.offset(offset as isize);
        }}
    }
}


#[derive(Debug)]
pub struct Regs([Value;Self::COUNT]);

impl Regs {
    pub const COUNT:usize = 14;
    pub const ARG_RESERVED:usize = 6;
    pub const CALLER_RESERVED:usize = Self::COUNT-Self::ARG_RESERVED;

    fn new() -> Self {
        Self([Value::Nil;Self::COUNT])
    }

    pub fn arg_regs(&self) -> [Value;Self::ARG_RESERVED] {
        let mut out = [Value::Nil;Self::ARG_RESERVED];
        for i in 0..Self::ARG_RESERVED {
            out[i] = self[i];
        }
        out
    }

    pub fn write_arg_regs(&mut self,args:[Value;Self::ARG_RESERVED]) {
        for i in 0..Self::ARG_RESERVED {
            self[i] = args[i]
        }
    }
}

impl Index<Reg> for Regs {
    type Output = Value;
    fn index(&self, index: Reg) -> &Self::Output {
        &self.0[index.0 as usize]
    }
}

impl IndexMut<Reg> for Regs {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        &mut self.0[index.0 as usize]
    }
}

impl Index<usize> for Regs {
    type Output = Value;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Regs {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}