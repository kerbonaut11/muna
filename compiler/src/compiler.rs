use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::{asm::Assembler, ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement}, expr::{self, Expr, InlineFunction, Op, UnaryOp}, bytecode::ByteCode};


pub struct FuncCtx {
    scope_depth:u32,
    locals:Vec<(Box<str>,u32)>
}

impl FuncCtx {
    pub fn new() -> Self {
        Self { 
            scope_depth:0,
            locals: vec![] 
        }
    }

    pub fn add_local(&mut self,name:&str) {
        let id = self.locals.len();
        self.locals.push((name.into(),self.scope_depth));
    }

    pub fn get_local_id(&self,name:&str) -> Option<u16> {
        self.locals.iter().enumerate().rev()
            .find(|(i,x)| *x.0 == *name)
            .map(|(i,_)| i as u16)
    }

    pub fn up_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn down_scope(&mut self) {
        loop {
            if self.locals.last().unwrap().1 != self.scope_depth {
                break;
            }
            _ = self.locals.pop();
        }
        self.scope_depth -= 1;
    }

    pub fn compile(&mut self,asm:&mut Assembler,block:&[AstNode]) {
        for node in block { match node {
            AstNode::Declaration(Declaration { lhs, rhs }) => {
                lhs.iter().for_each(|x| self.add_local(x));
                rhs.iter().for_each(|x| x.compile(self, asm));
            }

            AstNode::Assing(Assing { lhs, rhs }) => {
                for expr in rhs {
                    expr.compile(self, asm);
                }

                for lhs in lhs.iter().rev() {
                    match lhs {
                        Expr::Ident(name) => {
                            match self.get_local_id(&name) {
                                Some(id) => asm.encode_instr(ByteCode::Write(id)),
                                None => todo!(),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => todo!(),
        }};
    }
}

impl Expr {
    pub fn compile(&self,ctx:&mut FuncCtx,asm:&mut Assembler) {
        match self {
            Expr::Ident(name) => {
                match ctx.get_local_id(&name) {
                    Some(id) => asm.encode_instr(ByteCode::Load(id)),
                    None => todo!(),
                }
            },

            Expr::NilLiteral => asm.encode_instr(ByteCode::LoadNil),
            Expr::BoolLiteral(x) => asm.encode_instr(if *x {ByteCode::LoadTrue} else {ByteCode::LoadFalse}),
            Expr::IntLiteral(x) => {
                asm.encode_instr(ByteCode::LoadInt);
                asm.encode_int(*x);
            }
            Expr::FloatLiteral(x) => {
                asm.encode_instr(ByteCode::LoadFloat);
                asm.encode_float(*x);
            }
            Expr::StrLiteral(x) => {
                let idx = asm.get_idx_of_name(&x);
                asm.encode_instr(ByteCode::LoadStr(idx));
            } 

            Expr::Binary { op, lhs, rhs } => {
                lhs.compile(ctx, asm);
                rhs.compile(ctx, asm);
                asm.encode_instr(match op {
                    Op::Add => ByteCode::Add,
                    Op::Sub => ByteCode::Sub,
                    Op::Mul => ByteCode::Mul,
                    Op::Div => ByteCode::Div,
                    Op::IDiv => ByteCode::IDiv,
                    Op::Pow => ByteCode::Pow,
                    Op::Mod => ByteCode::Mod,
                    Op::Concat => ByteCode::Concat,
                    _ => todo!()
                });
            }


            _ => todo!()
        }
    }
}


#[derive(PartialEq, Eq, Hash)]
pub struct LableId(NonZeroU32);

static LABEL_NEXT_ID:Mutex<u32> = Mutex::new(0);

impl LableId {
    pub fn new() -> Self {
        let mut lock = LABEL_NEXT_ID.lock().unwrap();
        *lock += 1;
        return Self(unsafe{NonZeroU32::new_unchecked(*lock)}); 
    }
}