#[derive(Debug)]
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

    Halt = 17,
}


impl Into<u32> for ByteCode {
    fn into(self) -> u32 {
        unsafe{std::mem::transmute(self)}
    }
}