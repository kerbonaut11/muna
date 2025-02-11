use crate::bytecode::*;
use crate::function::CallErr;
use crate::table::Table;
use crate::value::{Type, Value};
use crate::vm::Vm;
use crate::Result;

type B = ByteCode;

pub fn exec(vm:&mut Vm,instr:ByteCode) -> Result<()> { match instr {

    B::MovRR(RegReg{dest,src}) => vm.regs[dest] = vm.regs[src],
    B::MovRM(RegMem{dest,src}) => vm.regs[dest] = vm.stack[src],
    B::MovMR(MemReg{dest,src}) => vm.stack[dest] = vm.regs[src],
    B::Push(src) => vm.stack.push(vm.regs[src]),
    B::Pop(dest) => vm.regs[dest] = vm.stack.pop(),

    B::LoadRNil(dest)                  => vm.regs[dest] = Value::Nil,
    B::LoadRBool(dest,x)         => vm.regs[dest] = x.into(),
    B::LoadRInt(dest)                  => vm.regs[dest] = vm.program.load_int().into(),
    B::LoadRFloat(dest)                => vm.regs[dest] = vm.program.load_float().into(),
    B::LoadRStr(RegMem{dest,src}) => vm.regs[dest] = vm.get_name(src).into(),
    B::LoadRFunc(dest) => todo!(),

    B::LoadMNil(dest)                  => vm.stack[dest] = Value::Nil,
    B::LoadMBool(dest,x)         => vm.stack[dest] = x.into(),
    B::LoadMInt(dest)                  => vm.stack[dest] = vm.program.load_int().into(),
    B::LoadMFloat(dest)                => vm.stack[dest] = vm.program.load_float().into(),
    B::LoadMStr(dest)                  => vm.stack[dest] = {let i = vm.program.load_mem(); vm.get_name(i).into()},
    B::LoadMFunc(dest) => todo!(),


    B::AddRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::add),
    B::SubRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::sub),
    B::MulRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::mul),
    B::DivRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::div),
    B::IDivRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::idiv),
    B::ModRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::rem),
    B::PowRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::pow),

    B::AndRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::bitand),
    B::OrRR(RegReg{dest,src})  => return vm.reg_reg_op(dest, src, Vm::bitor),
    B::XorRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::bitxor),
    B::ShrRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::shr),
    B::ShlRR(RegReg{dest,src}) => return vm.reg_reg_op(dest, src, Vm::shl),

    B::EqRR(RegReg{dest,src})     => vm.regs[dest] = vm.eq(vm.regs[dest],vm.regs[src])?.into(),
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
    B::XorRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::bitor),
    B::ShrRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::shr),
    B::ShlRM(RegMem{dest,src}) => return vm.reg_mem_op(dest, src, Vm::shl),

    B::EqRM(RegMem{dest,src})     => vm.regs[dest] = vm.eq(vm.regs[dest],vm.stack[src])?.into(),
    B::LessRM(RegMem{dest,src})   => vm.regs[dest] = vm.less(vm.regs[dest],vm.stack[src])?.into(),
    B::LessEqRM(RegMem{dest,src}) => vm.regs[dest] = vm.less_eq(vm.regs[dest],vm.stack[src])?.into(),
    B::IsRM(RegMem{dest,src})     => vm.regs[dest] = (vm.regs[dest].is(&vm.stack[src])).into(),

    B::AddRCI(dest)  => return  vm.reg_int_op(dest, Vm::addi),
    B::SubRCI(dest)  => return  vm.reg_int_op(dest, Vm::subi),
    B::MulRCI(dest)  => return  vm.reg_int_op(dest, Vm::muli),
    B::DivRCI(dest)  => return  vm.reg_int_op(dest, Vm::divi),
    B::IDivRCI(dest) => return  vm.reg_int_op(dest, Vm::idivi),  
    B::ModRCI(dest)  => return  vm.reg_int_op(dest, Vm::remi),
    B::PowRCI(dest)  => return  vm.reg_int_op(dest, Vm::powi),

    B::EqRCI(dest)     => vm.regs[dest] = {let x = vm.program.load_int(); vm.eqi(vm.regs[dest],x)?.into()},
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

    B::EqRCF(dest)=> vm.regs[dest]     = {let x = vm.program.load_float(); vm.eqf(vm.regs[dest],x)?.into()},
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

    B::EqMR(MemReg{dest,src})     => vm.stack[dest] = vm.eq(vm.stack[dest],vm.regs[src])?.into(),
    B::LessMR(MemReg{dest,src})   => vm.stack[dest] = vm.less(vm.stack[dest],vm.regs[src])?.into(),
    B::LessEqMR(MemReg{dest,src}) => vm.stack[dest] = vm.less_eq(vm.stack[dest],vm.regs[src])?.into(),
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

    B::EqMCI(dest)     => vm.stack[dest] = {let x = vm.program.load_int(); vm.eqi(vm.stack[dest],x)?.into()},
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

    B::EqMCF(dest)     => vm.stack[dest] = {let x = vm.program.load_float(); vm.eqf(vm.stack[dest],x)?.into()},
    B::LessMCF(dest)   => vm.stack[dest] = {let x = vm.program.load_float(); vm.lessf(vm.stack[dest],x)?.into()},
    B::LessEqMCF(dest) => vm.stack[dest] = {let x = vm.program.load_float(); vm.less_eqf(vm.stack[dest],x)?.into()},

    B::BoolAndRR(RegReg{dest, src}) => vm.regs[dest] = vm.bool_and(vm.regs[dest], vm.regs[src])?.into(),
    B::BoolOrRR(RegReg{dest, src})  => vm.regs[dest] = vm.bool_or(vm.regs[dest], vm.regs[src])?.into(),
    B::BoolNotR(dest) => vm.regs[dest] = vm.bool_not(vm.regs[dest])?.into(),

    B::BoolAndRM(RegMem{dest, src}) => vm.regs[dest] = vm.bool_and(vm.regs[dest], vm.stack[src])?.into(),
    B::BoolOrRM(RegMem{dest, src})  => vm.regs[dest] = vm.bool_or(vm.regs[dest], vm.stack[src])?.into(),

    B::BoolAndMR(MemReg{dest, src}) => vm.stack[dest] = vm.bool_and(vm.stack[dest], vm.regs[src])?.into(),
    B::BoolOrMR(MemReg{dest, src})  => vm.stack[dest] = vm.bool_or(vm.stack[dest], vm.regs[src])?.into(),
    B::BoolNotM(dest) => vm.stack[dest] = vm.bool_not(vm.stack[dest])?.into(),

    B::ToBoolR(dest) => vm.regs[dest] = vm.to_bool(vm.regs[dest])?.into(),
    B::ToIntR(dest) => vm.regs[dest] = vm.to_int(vm.regs[dest])?.into(),
    B::ToFloatR(dest) => vm.regs[dest] = vm.to_float(vm.regs[dest])?.into(),
    B::ToStrR(dest) => vm.regs[dest] = vm.to_str(vm.regs[dest])?.into(),

    B::ToBoolM(dest) => vm.stack[dest] = vm.to_bool(vm.stack[dest])?.into(),
    B::ToIntM(dest) => vm.stack[dest] = vm.to_int(vm.stack[dest])?.into(),
    B::ToFloatM(dest) => vm.stack[dest] = vm.to_float(vm.stack[dest])?.into(),
    B::ToStrM(dest) => vm.stack[dest] = vm.to_str(vm.stack[dest])?.into(),

    B::IsTypeR{reg,t} => vm.regs[reg] = (Type::of_val(&vm.regs[reg]) == t).into(),
    B::IsTypeM(MemType { mem, t }) => vm.stack[mem] = (Type::of_val(&vm.stack[mem]) == t).into(),


    B::MakeTable(TableDesc{dest,array_cap,map_cap}) => vm.regs[dest] = Table::with_capacity(array_cap as usize, map_cap as usize).alloc().into(),

    B::Get(TableGet{t,k,dest}) => vm.regs[dest] =  vm.get(vm.regs[t], vm.regs[k])?,
    B::GetConstKI{t,dest} => {let i = vm.program.load_int(); vm.regs[dest] = vm.geti(vm.regs[t], i)?;},
    B::GetConstKS{t,dest} => {let i = vm.program.load_mem(); vm.regs[dest] = vm.gets(vm.regs[t], vm.get_name(i))?;},  

    B::Set(TableSet{t,k,v}) => return vm.set(vm.regs[t], vm.regs[k], vm.regs[v]),
    B::SetConstKI{t,v} => {let i = vm.program.load_int(); return vm.seti(vm.regs[t], i, vm.regs[v]);},   
    B::SetConstKS{t,v} => {let i = vm.program.load_mem(); return vm.sets(vm.regs[t], vm.get_name(i), vm.regs[v]);},

    B::SetMetaTable(TableReg{t,reg}) => {let meta = vm.regs[reg]; vm.regs[t].set_meta_table(meta)?;},
    B::GetMetaTable(TableReg{t,reg}) => vm.regs[reg] = vm.regs[t].get_meta_table()?,

    B::OpenUpValR(dest) => vm.regs[dest].open_upval(),
    B::OpenUpValM(dest) => vm.stack[dest].open_upval(),

    B::GetUpValRR(RegReg{dest,src}) => vm.regs[dest] = vm.regs[src].get_upval(),
    B::GetUpValRM(RegMem{dest,src}) => vm.regs[dest] = vm.stack[src].get_upval(),
    B::GetUpValMR(MemReg{dest,src}) => vm.stack[dest] = vm.regs[src].get_upval(),

    B::SetUpValRR(RegReg{dest,src}) => {let val = vm.regs[src]; vm.regs[dest].set_upval(val);},
    B::SetUpValRM(RegMem{dest,src}) => {let val = vm.stack[src]; vm.regs[dest].set_upval(val);},
    B::SetUpValMR(MemReg{dest,src}) => {let val = vm.regs[src]; vm.stack[dest].set_upval(val);},

    B::SkipTrueR(src,offset)   => {let cond =  vm.truthy(vm.regs[src])?;  vm.skip_if( cond, offset);}
    B::SkipTrueM(src,offset)   => {let cond =  vm.truthy(vm.stack[src])?; vm.skip_if( cond, offset);}
    B::SkipFalseR(src,offset)  => {let cond = !vm.truthy(vm.regs[src])?;  vm.skip_if( cond, offset);}
    B::SkipFalseM(src,offset)  => {let cond = !vm.truthy(vm.stack[src])?; vm.skip_if( cond, offset);}
    B::SkipNilR(src,offset)    => {let cond = vm.regs[src].is_nil();  vm.skip_if( cond, offset);}
    B::SkipNilM(src,offset)    => {let cond = vm.stack[src].is_nil(); vm.skip_if( cond, offset);}
    B::SkipNonNilR(src,offset) => {let cond = vm.regs[src].is_nil();  vm.skip_if( cond, offset);}
    B::SkipNonNilM(src,offset) => {let cond = vm.stack[src].is_nil(); vm.skip_if( cond, offset);}


    B::Jump => unsafe{let offset= std::mem::transmute::<ByteCode,u32>(instr) >> 8; vm.program.ptr = vm.program.ptr.add(offset as usize);},
    B::JumpBack => unsafe{let offset = std::mem::transmute::<ByteCode,u32>(instr) >> 8; vm.program.ptr = vm.program.ptr.sub(offset as usize);},
    B::Call{f,args_provided, ret_count} => match vm.regs[f] {
        Value::Function(f) => vm.call(args_provided as usize, ret_count as usize, &f)?,
        _ => return Err(CallErr::WrongType(Type::of_val(&vm.regs[f])).into()),
    }
    B::Return => vm.ret(),

    B::Halt => return Err(crate::Error::Halt),

    _ => panic!("Unexpected Instruction")
};Ok(())}