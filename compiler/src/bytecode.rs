use crate::asm::LabelId;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ByteCode {

    LoadNil        = 0,
    LoadTrue       = 1,
    LoadFalse      = 2,
    LoadInt(i32)   = 3,
    LoadFloat(f32) = 4,
    LoadStr(u16)   = 6,

    Load(u16)  = 7,
    Write(u16) = 8,

    Add    =  9,
    Sub    = 10,
    Mul    = 11,
    Div    = 12,
    IDiv   = 13,
    Pow    = 14,
    Mod    = 15,
    Concat = 16,

    Less(bool)   = 26,
    LessEq(bool) = 27,
    Eq(bool)     = 28,

    Closure(ClosureArgs) = 17,
    Call    = 18,
    Ret     = 19,

    BindUpval(u16) = 20,
    GetUpval(u16)  = 21,
    SetUpval(u16)  = 22,

    Jump(LabelId)      = 23,
    JumpTrue(LabelId)  = 24,
    JumpFalse(LabelId) = 25,

    Halt = 30,
}

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClosureArgs{
    pub label:LabelId,
    pub upval_cap:u8,
    pub arg_count:u8,
}