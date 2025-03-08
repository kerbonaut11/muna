pub enum ByteCode {

    LoadNil,
    LoadTrue,
    LoadFalse,
    LoadInt,
    LoadFloat,

    Load(u16),
    Write(u16),

    Add,
    Sub,
    Mul,
    Div,
    IDiv,
    Pow,
    Mod,
    Concat,
}


impl Into<u32> for ByteCode {
    fn into(self) -> u32 {
        unsafe{std::mem::transmute(self)}
    }
}