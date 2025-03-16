use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::{asm::{ByteCodeVec, CompileCtx}, ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement}, bytecode::ByteCode, expr::{self, Expr, InlineFunction, Op, UnaryOp}};


pub struct FuncCtx<'a> {
    pub prev:Option<*mut Self>,
    pub sub_funcs:Vec<Self>,

    scope_depth:u32,
    args:Vec<Box<str>>,
    locals:Vec<(Box<str>,u32)>,
    upvals:Vec<Box<str>>,
}

enum VarKind {
    Local(u16),
    Global(Box<str>),
    Upval(u16),
}

impl<'a> FuncCtx<'a> {
    pub fn new(args:&[Box<str>]) -> Self {
        let mut ctx = Self {
            prev:None,
            sub_funcs:vec![],

            args:vec![],
            scope_depth:0,
            locals: vec![],
            upvals:vec![], 
        };

        for arg in args {
            ctx.args.push(arg.clone());
        }

        ctx
    }

    fn get_prev(&self) -> Option<&Self> {
        unsafe {
            self.prev.map(|x| x.as_ref().unwrap())
        }
    }

    fn get_prev_mut(&mut self) -> Option<&mut Self> {
        unsafe {
            self.prev.map(|x| x.as_mut().unwrap())
        }
    }

    fn add_local(&mut self,name:&str) {
        let id = self.locals.len() + self.args.len();
        self.locals.push((name.into(),self.scope_depth));
    }

    fn kind_of_ident(&mut self,name:&str) -> VarKind {
        if let Some(id) = self.locals.iter().enumerate().rev()
            .find(|(i,x)| *x.0 == *name)
            .map(|(i,_)| i as u16) {
            return VarKind::Local(id);
        }

        if let Some(id) = self.args.iter().enumerate().rev()
            .find(|(i,x)| ***x == *name)
            .map(|(i,_)| i as u16) {
            return VarKind::Local(id);
        }

        if let Some(id) = self.upvals.iter().enumerate()
            .find(|(i,x)| ***x == *name)
            .map(|(i,_)| i as u16) {
            return VarKind::Upval(id);
        }

        if let Some(prev) = self.get_prev() {
            if let Some(_) = prev.locals.iter().enumerate()
                .find(|(i,x)| *x.0 == *name)
                .map(|(i,_)| i as u16) {
                let id = self.upvals.len();
                self.upvals.push(name.into());
                return VarKind::Upval(id as u16);
            }
        }

        return VarKind::Global(name.into());
    }

    fn up_scope(&mut self) {
        self.scope_depth += 1;
    }

    fn down_scope(&mut self) {
        loop {
            if self.locals.last().unwrap().1 != self.scope_depth {
                break;
            }
            _ = self.locals.pop();
        }
        self.scope_depth -= 1;
    }

    pub fn compile(&mut self, block:&[AstNode], comp_ctx:&mut CompileCtx,  bytecode:&mut ByteCodeVec) {

        let mut sub_func_blocks = vec![];
        for node in block { if let AstNode::Function(func) = node {
            self.add_local(&func.name.clone());
            sub_func_blocks.push(&func.block);

            let mut func = Self::new(&func.args);
            func.prev = Some(self);
            self.sub_funcs.push(func);
        }}

        let mut func_load_bytecode = vec![];
        let mut sub_func_bytecode:Vec<ByteCodeVec> = vec![];

        for (i,sub_func) in self.sub_funcs.iter_mut().enumerate() {
            let mut func_bytecode = ByteCodeVec::new();
            sub_func.compile(sub_func_blocks[i], comp_ctx, &mut func_bytecode);
            sub_func_bytecode.push(func_bytecode);

            func_load_bytecode.push(ByteCode::Closure { upval_cap: sub_func.upvals.len() as u8, arg_count: sub_func.args.len() as u8 });
            func_load_bytecode.push(ByteCode::Load(i as u16));
        }

        for _ in 0..func_load_bytecode.len() {
            bytecode.encode_int(0);
        }

        for (sub_func_idx,sub_func) in self.sub_funcs.iter_mut().enumerate() {
            if !sub_func.upvals.is_empty() {
                bytecode.encode_instr(ByteCode::Load((sub_func_idx+1) as u16));

                for (upval_idx,upval) in sub_func.upvals.iter().enumerate() {
                    let id = self.locals.iter().enumerate().rev()
                        .find(|(i,x)| *x.0 == **upval)
                        .map(|(i,_)| i as u16)
                        .unwrap();

                        bytecode.encode_instr(ByteCode::Load(id+1));
                        bytecode.encode_instr(ByteCode::BindUpval(upval_idx as u16));
                }

                bytecode.encode_instr(ByteCode::Write((sub_func_idx+1) as u16));
            }
        }


        for node in block { match node {
            AstNode::Declaration(Declaration { lhs, rhs }) => {
                lhs.iter().for_each(|x| self.add_local(x));
                rhs.iter().for_each(|x| x.compile(self, comp_ctx,bytecode));
            }

            AstNode::Assing(Assing { lhs, rhs }) => {
                for expr in rhs {
                    expr.compile(self,comp_ctx,bytecode);
                }

                for lhs in lhs.iter().rev() {
                    match lhs {
                        Expr::Ident(name) => {
                            match self.kind_of_ident(name) {
                                VarKind::Local(id) => bytecode.encode_instr(ByteCode::Write(id+1)),
                                VarKind::Global(_) => todo!(),
                                VarKind::Upval(id) => bytecode.encode_instr(ByteCode::SetUpval(id)),
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }

            AstNode::Return(expr) => {
                if let Some(expr) = expr {
                    expr.compile(self, comp_ctx, bytecode);
                    bytecode.encode_instr(ByteCode::Write(0));
                }
                bytecode.encode_instr(ByteCode::Ret);
            }

            AstNode::Function(_) => {}
            _ => todo!(),
        }}

        bytecode.encode_instr(ByteCode::Ret);

        for i in (0..func_load_bytecode.len()).step_by(2) {
            bytecode.encode_instr_at(func_load_bytecode[i], i);
            let offset = bytecode.len()-i-2;
            bytecode.raw[i+1] = offset as u32;
            bytecode.append(&mut sub_func_bytecode[i/2]);
        }
    }
}

impl Expr {
    pub fn compile(&self, ctx:&mut FuncCtx, comp_ctx:&mut CompileCtx, bytecode:&mut ByteCodeVec) {
        match self {
            Expr::Ident(name) => {
                match ctx.kind_of_ident(name) {
                    VarKind::Local(id) => bytecode.encode_instr(ByteCode::Load(id+1)),
                    VarKind::Global(_) => todo!(),
                    VarKind::Upval(id) => bytecode.encode_instr(ByteCode::GetUpval(id)),
                }
            },

            Expr::NilLiteral => bytecode.encode_instr(ByteCode::LoadNil),
            Expr::BoolLiteral(x) => bytecode.encode_instr(if *x {ByteCode::LoadTrue} else {ByteCode::LoadFalse}),
            Expr::IntLiteral(x) => {
                bytecode.encode_instr(ByteCode::LoadInt);
                bytecode.encode_int(*x);
            }
            Expr::FloatLiteral(x) => {
                bytecode.encode_instr(ByteCode::LoadFloat);
                bytecode.encode_float(*x);
            }
            Expr::StrLiteral(x) => {
                let idx = comp_ctx.get_idx_of_name(&x);
                bytecode.encode_instr(ByteCode::LoadStr(idx));
            } 

            Expr::Binary { op, lhs, rhs } => {
                lhs.compile(ctx,comp_ctx,bytecode);
                rhs.compile(ctx,comp_ctx,bytecode);
                bytecode.encode_instr(match op {
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

            Expr::Call { function, args } => {
                bytecode.encode_instr(ByteCode::LoadNil);
                for arg in args {
                    arg.compile(ctx, comp_ctx, bytecode);
                }

                function.compile(ctx, comp_ctx, bytecode);
                
                bytecode.encode_instr(ByteCode::Call);
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