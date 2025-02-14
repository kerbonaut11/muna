#[cfg(test)]

use std::mem::transmute;

use macros::wrap;

use crate::{bytecode::{ByteCode, Mem, Reg, RegMem, RegReg, TableDesc, TableReg}, string::LuaString, table::Table, value::Value, vm::Vm, Error};

#[test]
pub fn test1() {
    let code = unsafe {vec![
        ByteCode::LoadRInt(Reg(0)),
        transmute(1),
        transmute(0),
        ByteCode::LoadMFloat(Mem(0)),
        transmute(0),
        transmute(0),
        ByteCode::AddRM(RegMem{dest:Reg(0),src:Mem(0)})
    ]};
    let mut vm = Vm::new(code,Box::new([])); 
    vm.exec().unwrap();
    vm.exec().unwrap();
    vm.exec().unwrap();
    println!("{:?}",vm.regs)
}

#[test]
pub fn test2() {
    let code = unsafe {vec![
        ByteCode::MakeTable(TableDesc{dest:Reg(0),array_cap:32,map_cap:1}),

        ByteCode::LoadRBool(Reg(1),true),
        ByteCode::SetConstKI{ t: Reg(0), v: Reg(1) },
        transmute(1),
        transmute(0),

        ByteCode::LoadRBool(Reg(1),false),
        ByteCode::SetConstKS{ t: Reg(0), v: Reg(1) },
        transmute(0),

        ByteCode::GetConstKI { t: Reg(0), dest: Reg(1) },
        transmute(1),
        transmute(0),

        ByteCode::GetConstKS{ t: Reg(0), dest: Reg(2) },
        transmute(0),

        ByteCode::GetConstKI{ t: Reg(0), dest: Reg(0) },
        transmute(6363),
        transmute(6363),

        ByteCode::Halt,
    ]};
    let mut vm = Vm::new(code,Box::new([LuaString::new("abcxyz")])); 
    loop {
        match vm.exec() {
            Ok(_) => {},
            Err(e) => if let Error::Halt = e {
                break;
            } else {
                panic!("{:?}",e)
            }
        }
    }
    println!("{:?}",vm.regs);
    assert!(vm.regs[0].is_nil());
    vm.regs[1].assert_bool(true);
    vm.regs[2].assert_bool(false);
}

#[test]
pub fn test3() {
    let code = unsafe {vec![
        ByteCode::LoadRFunc { dest: Reg(3), arg_count: 2, ret_count: 1 },
        transmute(10),
        transmute(0),
        ByteCode::SetConstKS { t: Reg(0), v: Reg(3) },
        transmute(0),
        ByteCode::SetMetaTable(TableReg { t: Reg(1), reg: Reg(0) }),
        ByteCode::AddRCI(Reg(1)),
        transmute(32),
        transmute(0),
        ByteCode::Halt,
        ByteCode::LoadRBool(Reg(0), false),
        ByteCode::Return,
        ByteCode::Halt,
    ]};

    let mut vm = Vm::new(code,Box::new([LuaString::new("__add")]));
    vm.regs[0] = Table::new().alloc().into();
    vm.regs[1] = Table::new().alloc().into();

    loop {
        match vm.exec() {
            Ok(_) => {},
            Err(e) => if let Error::Halt = e {
                break;
            } else {
                panic!("{:?}",e)
            }
        }
    }
    println!("{:?}",vm.regs);
}
