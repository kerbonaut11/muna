use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::bytecode::{Mem, Reg};

use super::{ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement}, expr::{self, Expr, InlineFunction}, LocalId};

pub struct FunctionCtx<'a> {
    is_inline:bool,
    pub prev:Option<&'a Self>,
    pub sub_funcs:Vec<Self>,

    args:Vec<Local>,
    scope_depth:u32,
    local_next_id:u32,
    locals:Vec<Local>,
}

struct Local {
    scope_depth:LocalId,
    id:u32,
    name:Box<str>
}

impl<'a> FunctionCtx<'a> {
    fn new(args:&[Box<str>],is_inline:bool) -> Self {
        let mut ctx = Self{
            is_inline,
            prev:None,
            sub_funcs:vec![],

            args:vec![],
            scope_depth:0,
            local_next_id:args.len() as u32,
            locals:vec![],
        };

        for arg in args {
            ctx.args.push(Local{
                scope_depth: 0,
                id: ctx.args.len() as u32,
                name: arg.clone().into(),
            });
        }

        ctx
    }
}

impl<'a> From<&InlineFunction> for FunctionCtx<'a> {
    fn from(value: &InlineFunction) -> Self {
        Self::new(&value.args,true)
    }
}

impl<'a> From<&Function> for FunctionCtx<'a> {
    fn from(value: &Function) -> Self {
        Self::new(&value.args,false)
    }
}



impl<'a> FunctionCtx<'a> {
    fn add_local(&mut self,name:&str) {
        self.locals.push(Local{
            scope_depth:self.scope_depth,
            id:self.local_next_id,
            name:name.into(),
        });
        self.local_next_id += 1;
    }

    fn get_local_id(&self,name:&str) -> Option<LocalId> {
        if let Some(local) = self.args.iter().find(|x| x.name == name.into() ) {
            return Some(local.id);
        } 
        if let Some(local) = self.locals.iter().rev().find(|x| x.name == name.into() ) {
            return Some(local.id);
        };

        return None;
    }

    fn up_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn down_scope(&mut self) {
        while let Some(local) = self.locals.last() {
            if local.scope_depth != self.scope_depth {
                break;
            }
            self.locals.pop();
        }

        self.scope_depth -= 1;
    }

    fn get_expr_of_ident(&self,name:&str) -> Expr {
        if let Some(id) = self.get_local_id(name) {
            return Expr::Local(id);
        }

        return Expr::Global(name.into());
    }


    pub fn compile(&mut self,block:Block,ir:&mut Vec<((),Option<LableId>)>) {
        if block.is_empty() {
            return;
        }
    }
}


#[derive(PartialEq, Eq, Hash)]
struct LableId(NonZeroU32);

static LABEL_NEXT_ID:Mutex<u32> = Mutex::new(0);

impl LableId {
    pub fn new() -> Self {
        let mut lock = LABEL_NEXT_ID.lock().unwrap();
        *lock += 1;
        return Self(unsafe{NonZeroU32::new_unchecked(*lock)}); 
    }
}

enum IrValue {
    Local(LableId),
    Reg(Reg),
    Mem(Mem),
    IntLiteral(i64),
    FloatLiteral(f64),
    StrLiteral(Box<str>),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum ByteCode {

    MovRR(RegReg),
    MovMR(MemReg),
    MovRM(RegMem),
    Push(Reg),
    Pop(Reg),

    LoadRNil(Reg),
    LoadRBool(Reg,bool),
    LoadRInt(Reg), 
    LoadRFloat(Reg),
    LoadRStr(RegMem),
    LoadRFunc{dest:Reg,arg_count:u8,ret_count:u8},

    LoadMNil(Mem),
    LoadMBool(Mem,bool),
    LoadMInt(Mem), 
    LoadMFloat(Mem),

    AddRR(RegReg),
    SubRR(RegReg),
    MulRR(RegReg), 
    DivRR(RegReg), 
    IDivRR(RegReg), 
    ModRR(RegReg),
    PowRR(RegReg),

    AndRR(RegReg),
    OrRR(RegReg),
    XorRR(RegReg),
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
    XorRM(RegMem),
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
    XorRC(Reg),
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
    XorMR(MemReg),
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


    BoolAndRR(RegReg),
    BoolOrRR(RegReg),
    BoolNotR(Reg),

    BoolAndRM(RegMem),
    BoolOrRM(RegMem),

    BoolAndMR(MemReg),
    BoolOrMR(MemReg),
    BoolNotM(Mem),


    ToBool(Reg),
    ToInt(Reg),
    ToFloat(Reg),
    ToStr(Reg),

    IsType{reg:Reg,t:Type},
    MakeTable(TableDesc),
    Get(TableGet),
    Set(TableSet),
    SetMetaTable(TableReg),
    GetMetaTable(TableReg),


    SkipTrue(Reg,i8),
    SkipFalse(Reg,i8),
    SkipNil(Reg,i8),
    SkipNonNil(Reg,i8),

    Jump,
    JumpBack,
    Call{f:Reg,args_provided:u8,ret_count:u8},
    Return,

    //skips next instruction if the for returns Some()
    KVForStart(Reg),
    KvFor{kdest:Reg,vdest:Reg},
    NumForStart{low:Reg,high:Reg},
    NumFor{dest:Reg},

    Halt,
}