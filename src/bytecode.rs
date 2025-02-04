use crate::value::Type;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ByteCode {

    MovRR(RegReg),
    MovMR(MemReg),
    MovRM(RegMem),
    Push(Reg),
    Pop(Reg),

    LoadNil(Reg),
    LoabBool(Reg,bool),
    LoadFloat(Reg),
    LoadInt(Reg), 
    LoadStr(RegMem),
    LoadFunc(Reg),

    AddRR(RegReg),
    SubRR(RegReg),
    MulRR(RegReg), 
    DivRR(RegReg), 
    IDivRR(RegReg), 
    ModRR(RegReg),
    PowRR(RegReg),

    AndRR(RegReg),
    OrRR(RegReg),
    ShrRR(RegReg),
    ShlRR(RegReg),

    EqRR(RegReg),
    LessRR(RegReg),
    LessEqRR(RegReg),
    IsRR(RegReg),

    NotR(Reg),
    NegR(Reg),
    LenR(Reg),

    AddRM(RegMem),
    SubRM(RegMem),
    MulRM(RegMem), 
    DivRM(RegMem), 
    IDivRM(RegMem), 
    ModRM(RegMem),
    PowRM(RegMem),

    AndRM(RegMem),
    OrRM(RegMem),
    ShrRM(RegMem),
    ShlRM(RegMem),

    EqRM(RegMem),
    LessRM(RegMem),
    LessEqRM(RegMem),
    IsRM(RegMem),

    //const stored in next instruction
    //const ints
    AddRCI(Reg),
    SubRCI(Reg),
    MulRCI(Reg),
    DivRCI(Reg),
    IDivRCI(Reg),  
    ModRCI(Reg),
    PowRCI(Reg),

    EqRCI(Reg),
    LessRCI(Reg),
    LessEqRCI(Reg),

    AndRC(Reg),
    OrRC(Reg),
    ShrRC{val:u8,dest:Reg},
    ShlRC{val:u8,dest:Reg},  

    //const floats
    AddRCF(Reg),
    SubRCF(Reg),
    MulRCF(Reg),
    DivRCF(Reg),
    ModRCF(Reg),
    PowRCF(Reg),

    EqRCF(Reg),
    LessRCF(Reg),
    LessEqRCF(Reg),


    AddMR(MemReg),
    SubMR(MemReg),
    MulMR(MemReg), 
    DivMR(MemReg), 
    IDivMR(MemReg), 
    ModMR(MemReg),
    PowMR(MemReg),

    AndMR(MemReg),
    OrMR(MemReg),
    ShrMR(MemReg),
    ShlMR(MemReg),

    EqMR(MemReg),
    LessMR(MemReg),
    LessEqMR(MemReg),
    IsMR(MemReg),

    NotM(Mem),
    NegM(Mem),
    LenM(Mem),


    AddMCI(Mem),
    SubMCI(Mem),
    MulMCI(Mem),
    DivMCI(Mem),
    IDivMCI(Mem),  
    ModMCI(Mem),
    PowMCI(Mem),

    AndMC(Mem),
    OrMC(Mem),
    ShrMC(MemVal),
    ShlMC(MemVal),

    EqMCI(Mem),
    LessMCI(Mem),
    LessEqMCI(Mem),


    AddMCF(Mem),
    SubMCF(Mem),
    MulMCF(Mem),
    DivMCF(Mem),
    ModMCF(Mem),
    PowMCF(Mem),

    EqMCF(Mem),
    LessMCF(Mem),
    LessEqMCF(Mem),


    ToBoolR(Reg),
    ToIntR(Reg),
    ToFloatR(Reg),
    ToStingR(Reg),

    ToBoolM(Mem),
    ToIntM(Mem),
    ToFloatM(Mem),
    ToStingM(Mem),

    IsTypeR{reg:Reg,t:Type},
    IsTypeM(MemType),

    MakeTable(TableDesc),

    TableGet(TableGet),
    TableGetConstKI{t:Reg,dest:Reg},   
    TableGetConstKS{t:Reg,dest:Reg},   

    TableSet(TableSet),
    TableSetConstVI{t:Reg,key:Reg},
    TableSetConstVF{t:Reg,key:Reg},
    TableSetConstKI{t:Reg},   
    TableSetConstKS(TableMem),
    TableSetConstVIConstKI(Reg),
    TableSetConstVFConstKI(Reg),
    TableSetConstVIConstKS(TableMem),
    TableSetConstVFConstKS(TableMem),

    SetMetaTable(TableReg),
    GetMetaTable(TableReg),
    GetMethod(TableMem),

    OpenUpValR(Reg),
    OpenUpValM(Mem),

    GetUpValRR(RegReg),
    GetUpValMR(MemReg),
    GetUpValRM(RegMem),

    SetUpValRR(RegReg),
    SetUpValMR(MemReg),
    SetUpValRM(RegMem),


    SkipTrueR(Reg),
    SkipTrueM(Mem),
    SkipFalseR(Reg),
    SkipFalseM(Mem),
    SkipNilR(Reg),
    SkipNilM(Mem),
    SkipNonNilR(Reg),
    SkipNonNilM(Mem),

    Jump(Offset),

    CallPrep{f:Reg,arg_count:u8},
    Call,
    Return,

    //skips next instruction if the for returns Some()
    KVForStart(Reg),
    KvFor{kdest:Reg,vdest:Reg},
    NumForStart{low:Reg,high:Reg},
    NumFor{dest:Reg},

    Halt,
}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Reg(pub u8);

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Mem(pub u16);

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct RegReg{pub dest:Reg,pub src:Reg}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct RegMem{pub dest:Reg,pub src:Mem}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemReg{pub dest:Mem,pub src:Reg}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemVal{pub mem:Mem,pub val:u8}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct MemType{pub mem:Mem,pub t:Type}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct TableDesc{pub dest:Reg,pub array_cap:u8,pub map_cap:u8}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct TableSet{pub t:Reg,pub k:Reg,pub v:Reg}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct TableGet{pub t:Reg,pub k:Reg,pub dest:Reg}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct TableMem{pub t:Reg,pub mem:Mem}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct TableReg{pub t:Reg,pub reg:Reg}

#[repr(packed)]
#[derive(Clone, Copy, Debug)]
pub struct Offset{high:u8,low:u16}



#[test]
fn size() {
    assert_eq!(std::mem::size_of::<ByteCode>(),4);
    let hlt = &ByteCode::Halt;
    println!("Instruction count {}",unsafe{*std::mem::transmute::<&ByteCode,*mut u8>(hlt)});
}