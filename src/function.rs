use std::{hash::Hash, ops::Range};

use crate::{bytecode::ByteCode, gc::{FunctionAlloc, Gc, MarkDown, Marked}, upval::UpVal, value::{Type, Value}, vm::{Regs, Vm}, Error, Result};

#[derive(Debug)]
pub struct Function {
    is_call_back:bool,
    ptr: *const (),
    arg_count: u8,
    ret_count: u8,
    marked:bool,
    up_vals:Vec<UpVal>
}

#[derive(Debug)]
pub enum CallErr {
    WrongType(Type),
    MetaWrongArgCount{expected:u8,got:u8},
    MetaWrongRetCount{expected:u8,got:u8},
    UdTypeMismatch{expected:Type,got:Type},
}


fn stack_reserved(x:usize) -> usize {
    if x > Regs::ARG_RESERVED {
        x - Regs::ARG_RESERVED
    } else {
        0
    }
}


impl Function {
    pub fn new_call_back(ptr:*const (),arg_count:usize,ret_count:usize) -> Self {
        Self { 
            is_call_back: true,
            ptr: ptr as *mut (),
            arg_count:arg_count as u8,
            ret_count:ret_count as u8,
            marked: false, 
            up_vals: vec![] 
        }
    }

    pub fn new(vm:&Vm,offset:usize,arg_count:u8,ret_count:u8) -> Self {
        Self { 
            is_call_back: false,
            ptr: vm.program.ptr_offset(offset) as *const (),
            arg_count:arg_count,
            ret_count:ret_count,
            marked: false, 
            up_vals: vec![] 
        }
    }

    pub fn alloc(self) -> Gc<Self> {
        FunctionAlloc::alloc(self)
    }

    pub fn arg_count(&self) -> usize {self.arg_count as usize}
    pub fn ret_count(&self) -> usize {self.ret_count as usize}

    fn stack_arg_count(&self) -> usize {
        stack_reserved(self.arg_count as usize)
    }

    fn stack_ret_count(&self) -> usize {
        stack_reserved(self.ret_count as usize)
    }

    pub fn addres(&self) -> usize {
        self.ptr as usize
    }
}

impl Marked for Function {
    fn is_marked(&self) -> bool {self.marked}
    fn mark(&mut self) {self.marked=true}
    fn unmark(&mut self) {self.marked=false}
}

impl MarkDown for Function {
    fn mark_down(&mut self,vm:&mut Vm) {
        self.up_vals.iter_mut().for_each(move |x| x.mark_down(vm));
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

impl Eq for Function {}

impl Hash for Function {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.ptr as usize);
    }
}

#[derive(Debug)]
pub enum CallStackEntry {
    Normal{
        ret_addres:*const ByteCode,
        base_ptr:*const Value,
        ret_count:u8,
        ret_count_expected:u8
    },

    Meta,
}

pub type CallStack = Vec<CallStackEntry>;

impl Vm {

    fn fill_args_with_nil(&mut self,range:Range<usize>) {
        for i in range {
            if i < Regs::ARG_RESERVED {
                self.regs[i] = Value::Nil;
            } else {
                self.stack.push(Value::Nil);
            }
        }
    }

    fn push_call_stack(&mut self,f:&Function,ret_count_expected:usize) {
        self.call_stack.push(CallStackEntry::Normal { 
            ret_addres: self.program.ptr,
            base_ptr: self.stack.base_ptr,
            ret_count: f.ret_count,
            ret_count_expected: ret_count_expected as u8
        });
    }

    pub fn call(&mut self,args_provided:usize,ret_count_expected:usize,f:&Function) -> Result<()> {

        self.fill_args_with_nil(args_provided..f.arg_count());
        self.push_call_stack(f, ret_count_expected);

        self.stack.base_ptr = unsafe {
            self.stack.top_ptr.sub(f.stack_arg_count()).add(1)
        };

        if !f.is_call_back {
            self.program.ptr = f.ptr as *const ByteCode;
            Ok(())
        } else {
            let f:fn(&mut Vm) -> Result<()> = unsafe{std::mem::transmute(f.ptr)};
            f(self)
        }
    }

    pub fn call_meta_unary(&mut self,f:&Function) -> Result<()> {

        if f.arg_count() != 1 {return Err(CallErr::MetaWrongArgCount{ expected:1, got: f.arg_count() as u8 }.into());}
        if f.ret_count() != 1 {return Err(CallErr::MetaWrongRetCount{ expected:1, got: f.ret_count }.into());}

        self.call_stack.push(CallStackEntry::Meta);
        self.call_and_exec(f)
    }

    pub fn call_meta(&mut self,f:&Function) -> Result<()> {

        if f.arg_count() != 2 {return Err(CallErr::MetaWrongArgCount{ expected:2, got: f.arg_count() as u8 }.into());}
        if f.ret_count() != 1 {return Err(CallErr::MetaWrongRetCount{ expected:1, got: f.ret_count }.into());}

        self.call_stack.push(CallStackEntry::Meta);
        self.call_and_exec(f)
    }

    pub fn call_meta_new_idx(&mut self,f:&Function) -> Result<()> {

        if f.arg_count() != 3 {return Err(CallErr::MetaWrongArgCount{ expected:3, got: f.arg_count() as u8 }.into());}
        if f.ret_count() != 0 {return Err(CallErr::MetaWrongRetCount{ expected:0, got: f.ret_count }.into());}

        self.call_stack.push(CallStackEntry::Meta);
        self.call_and_exec(f)
    }

    fn call_and_exec(&mut self,f:&Function) -> Result<()> {
        if !f.is_call_back {
            self.program.ptr = f.ptr as *const ByteCode;
            loop {
                match self.exec() {
                    Ok(_) => {},
                    Err(e) => match e {
                        Error::Halt => break,
                        _ => return Err(e),
                    }
                }
            }
            Ok(())
        } else {
            let f:fn(&mut Vm) -> Result<()> = unsafe{std::mem::transmute(f.ptr)};
            f(self)
        }
    }

    pub fn ret(&mut self) -> Result<()>{

        let(ret_addres,
            base_ptr,
            ret_count,
            ret_count_expected) 
        = match self.call_stack.pop().unwrap() {
            CallStackEntry::Normal { ret_addres, base_ptr, ret_count, ret_count_expected } => (ret_addres,base_ptr,ret_count,ret_count_expected),
            CallStackEntry::Meta => return Err(crate::Error::Halt)
        };
        self.program.ptr = ret_addres;
        self.stack.base_ptr = base_ptr as *mut Value;

        if ret_count_expected > ret_count {
            for i in ret_count as usize..ret_count_expected as usize {
                if i < Regs::ARG_RESERVED {
                    self.regs[i] = Value::Nil;
                } else {
                    self.stack.push(Value::Nil);
                }
            }
        } else {
            let stack_size_expected = stack_reserved(ret_count_expected as usize);
            let stack_size = stack_reserved(ret_count as usize);
            self.stack.top_ptr = unsafe {
                self.stack.top_ptr.sub(stack_size-stack_size_expected)
            };
        }
        Ok(())
    }
}

#[test]
fn test() {
    let mut vm = Vm::new(vec![], Box::new([]));
    for i in 0..Regs::COUNT {
        vm.regs[i] = (i as i64 + 10).into()
    }
    vm.stack.push(1.0.into());
    vm.stack.push(2.0.into());

    let f = Function {
        is_call_back:false,
        ptr:0xFFAA as *const (),
        arg_count:7,
        ret_count:8,
        marked:false,
        up_vals:vec![]
    };

    vm.call(5, 1, &f).unwrap();
    assert_eq!(vm.program.ptr as *const (),f.ptr);
    assert!(vm.eq(vm.regs[4],Value::Int(14)).unwrap());
    assert!(vm.eq(vm.regs[5],Value::Nil).unwrap());
    assert!(vm.eq(*vm.stack.top(),Value::Nil).unwrap());
    assert!(vm.eq(vm.stack[0],Value::Nil).unwrap());
    
    vm.stack.push(Value::Bool(false));

    vm.ret();

    fn call_back(vm:&mut Vm) -> Result<()> {
        vm.regs[12] = Value::Int(32);
        vm.ret();
        Ok(())
    }

    let f = Function {
        is_call_back:true,
        ptr:call_back as *const (),
        arg_count:0,
        ret_count:0,
        marked:false,
        up_vals:vec![]
    };

    vm.call(0, 0, &f).unwrap();
    assert!(vm.eq(vm.regs[12],Value::Int(32)).unwrap());
}