use crate::bytecode::*;
use crate::vm::Vm;
use crate::Result;

type B = ByteCode;

pub fn exec(vm:&mut Vm,instr:ByteCode) -> Result<()> { match instr {

    B::MovRR(RegReg{dest,src})  => vm.regs[dest] = vm.regs[src],
    B::MovMR(MemReg{dest,src})  => vm.stack[dest] = vm.regs[src],
    B::MovRM(RegMem{dest,src})  => vm.regs[dest] = vm.stack[src],
 
    B::AddRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::add),
    B::SubRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::sub),
    B::MulRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::mul),
    B::DivRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::div),
    B::IDivRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::idiv),
    B::ModRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::rem),
    B::PowRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::pow),

    B::AndRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::bitand),
    B::OrRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::bitor),
    B::ShrRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::shr),
    B::ShlRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::shl),

    B::EqRR(RegReg{dest,src})     => vm.regs[dest] = (vm.regs[dest] == vm.regs[src]).into(),
    B::LessRR(RegReg{dest,src})   => vm.regs[dest] = vm.less(vm.regs[dest],vm.regs[src])?.into(),
    B::LessEqRR(RegReg{dest,src}) => vm.regs[dest] = vm.less_eq(vm.regs[dest],vm.regs[src])?.into(),
    B::IsRR(RegReg{dest,src})     => vm.regs[dest] = (vm.regs[dest].is(&vm.regs[src])).into(),

    B::NotR(reg) => vm.regs[reg] = vm.not( vm.regs[reg])?,
    B::NegR(reg) => vm.regs[reg] = vm.neg( vm.regs[reg])?,
    B::LenR(reg) => vm.regs[reg] = vm.len( vm.regs[reg])?,

    B::AddRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::add),
    B::SubRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::sub),
    B::MulRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::mul),
    B::DivRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::div),
    B::IDivRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::idiv),
    B::ModRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::rem),
    B::PowRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::pow),

    B::AndRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::bitand),
    B::OrRM(RegMem{dest,src})  => return vm.reg_mem_op(dest, src, Vm::bitor),
    B::ShrRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::shr),
    B::ShlRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::shl),

    B::EqRM(RegMem{dest,src})     => vm.regs[dest] = (vm.regs[dest] == vm.stack[src]).into(),
    B::LessRM(RegMem{dest,src})   => vm.regs[dest] =  vm.less(vm.regs[dest],vm.stack[src])?.into(),
    B::LessEqRM(RegMem{dest,src}) => vm.regs[dest] =  vm.less_eq(vm.regs[dest],vm.stack[src])?.into(),
    B::IsRM(RegMem{dest,src})     => vm.regs[dest] = (vm.regs[dest].is(&vm.stack[src])).into(),

    B::AddRCI(dest)  => return  vm.reg_int_op(dest, Vm::addi),
    B::SubRCI(dest)  => return  vm.reg_int_op(dest, Vm::subi),
    B::MulRCI(dest)  => return  vm.reg_int_op(dest, Vm::muli),
    B::DivRCI(dest)  => return  vm.reg_int_op(dest, Vm::divi),
    B::IDivRCI(dest) => return  vm.reg_int_op(dest, Vm::idivi),  
    B::ModRCI(dest)  => return  vm.reg_int_op(dest, Vm::remi),
    B::PowRCI(dest)  => return  vm.reg_int_op(dest, Vm::powi),

    B::EqRCI(dest)     => vm.regs[dest] = {let x = vm.program.load_int(); vm.eqi(vm.regs[dest],x).into()},
    B::LessRCI(dest)   => vm.regs[dest] = {let x = vm.program.load_int(); vm.lessi(vm.regs[dest],x)?.into()},
    B::LessEqRCI(dest) => vm.regs[dest] = {let x = vm.program.load_int(); vm.less_eqi(vm.regs[dest],x)?.into()},

    B::AndRC(dest) => return vm.reg_int_op(dest, Vm::bitandi),
    B::OrRC(dest)  => return vm.reg_int_op(dest, Vm::bitori),
    B::ShrRC{val,dest} => vm.regs[dest] = vm.shri(vm.regs[dest],val)?,
    B::ShlRC{val,dest} => vm.regs[dest] = vm.shli(vm.regs[dest],val)?, 

    B::AddRCF(dest)=> return  vm.reg_float_op(dest, Vm::addf),
    B::SubRCF(dest)=> return  vm.reg_float_op(dest, Vm::subf),
    B::MulRCF(dest)=> return  vm.reg_float_op(dest, Vm::mulf),
    B::DivRCF(dest)=> return  vm.reg_float_op(dest, Vm::divf),
    B::ModRCF(dest)=> return  vm.reg_float_op(dest, Vm::remf),
    B::PowRCF(dest)=> return  vm.reg_float_op(dest, Vm::powf),

    B::EqRCF(dest)=> vm.regs[dest]     = {let x = vm.program.load_float(); vm.eqf(vm.regs[dest],x).into()},
    B::LessRCF(dest) => vm.regs[dest]  = {let x = vm.program.load_float(); vm.lessf(vm.regs[dest],x)?.into()},
    B::LessEqRCF(dest) =>vm.regs[dest] = {let x = vm.program.load_float(); vm.less_eqf(vm.regs[dest],x)?.into()},


    B::AddMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::add),
    B::SubMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::sub),
    B::MulMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::mul),
    B::DivMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::div),
    B::IDivMR(MemReg{dest,src}) => return vm.mem_reg_op(dest, src, Vm::idiv),
    B::ModMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::rem),
    B::PowMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::pow),

    B::AndMR(MemReg{dest,src}) => return vm.mem_reg_op(dest, src, Vm::bitand),
    B::OrMR(MemReg{dest,src})  => return vm.mem_reg_op(dest, src, Vm::bitor),
    B::ShrMR(MemReg{dest,src}) => return vm.mem_reg_op(dest, src, Vm::shr),
    B::ShlMR(MemReg{dest,src}) => return vm.mem_reg_op(dest, src, Vm::shl),

    B::EqMR(MemReg{dest,src})     => vm.stack[dest] = (vm.stack[dest] == vm.regs[src]).into(),
    B::LessMR(MemReg{dest,src})   => vm.stack[dest] =  vm.less(vm.stack[dest],vm.regs[src])?.into(),
    B::LessEqMR(MemReg{dest,src}) => vm.stack[dest] =  vm.less_eq(vm.stack[dest],vm.regs[src])?.into(),
    B::IsMR(MemReg{dest,src})     => vm.stack[dest] = (vm.stack[dest].is(&vm.regs[src])).into(),


    B::NotM(dest) => vm.stack[dest] = vm.not( vm.stack[dest])?,
    B::NegM(dest) => vm.stack[dest] = vm.neg( vm.stack[dest])?,
    B::LenM(dest) => vm.stack[dest] = vm.len( vm.stack[dest])?,

    B::AddMCI(dest)  => return  vm.mem_int_op(dest, Vm::addi),
    B::SubMCI(dest)  => return  vm.mem_int_op(dest, Vm::subi),
    B::MulMCI(dest)  => return  vm.mem_int_op(dest, Vm::muli),
    B::DivMCI(dest)  => return  vm.mem_int_op(dest, Vm::divi),
    B::IDivMCI(dest) => return  vm.mem_int_op(dest, Vm::idivi),  
    B::ModMCI(dest)  => return  vm.mem_int_op(dest, Vm::remi),
    B::PowMCI(dest)  => return  vm.mem_int_op(dest, Vm::powi),

    B::EqMCI(dest)     => vm.stack[dest] = {let x = vm.program.load_int(); vm.eqi(vm.stack[dest],x).into()},
    B::LessMCI(dest)   => vm.stack[dest] = {let x = vm.program.load_int(); vm.lessi(vm.stack[dest],x)?.into()},
    B::LessEqMCI(dest) => vm.stack[dest] = {let x = vm.program.load_int(); vm.less_eqi(vm.stack[dest],x)?.into()},

    B::AndMC(dest) => return vm.mem_int_op(dest, Vm::bitandi),
    B::OrMC(dest)  => return vm.mem_int_op(dest, Vm::bitori),
    B::ShrMC(MemVal{mem,val}) => vm.stack[mem] = vm.shri(vm.stack[mem],val)?,
    B::ShlMC(MemVal{mem,val}) => vm.stack[mem] = vm.shli(vm.stack[mem],val)?, 

    B::AddMCF(dest) => return  vm.mem_float_op(dest, Vm::addf),
    B::SubMCF(dest) => return  vm.mem_float_op(dest, Vm::subf),
    B::MulMCF(dest) => return  vm.mem_float_op(dest, Vm::mulf),
    B::DivMCF(dest) => return  vm.mem_float_op(dest, Vm::divf),
    B::ModMCF(dest) => return  vm.mem_float_op(dest, Vm::remf),
    B::PowMCF(dest) => return  vm.mem_float_op(dest, Vm::powf),

    B::EqMCF(dest)     => vm.stack[dest] = {let x = vm.program.load_float(); vm.eqf(vm.stack[dest],x).into()},
    B::LessMCF(dest)   => vm.stack[dest] = {let x = vm.program.load_float(); vm.lessf(vm.stack[dest],x)?.into()},
    B::LessEqMCF(dest) => vm.stack[dest] = {let x = vm.program.load_float(); vm.less_eqf(vm.stack[dest],x)?.into()},

    _ => panic!("Unexpected Instruction")
};Ok(())}