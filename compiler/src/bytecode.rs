#[derive(Debug,Clone, Copy,PartialEq, Eq)]
#[repr(u8)]
pub enum ByteCode {

    LoadNil              = 0,
    LoadTrue             = 1,
    LoadFalse            = 2,
    LoadInt              = 3,
    LoadFloat            = 4,
    LoadStr(u16)         = 6,

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

    Closure{
        upval_cap:u8,
        arg_count:u8,
    } = 17,
    Call    = 18,
    Ret     = 19,

    BindUpval(u16) = 20,
    GetUpval(u16)  = 21,
    SetUpval(u16)  = 22,

    Jump(i16)      = 23,
    JumpTrue(i16)  = 24,
    JumpFalse(i16) = 25,

    Halt = 30,
}


impl Into<u32> for ByteCode {
    fn into(self) -> u32 {
        unsafe{std::mem::transmute(self)}
    }
}