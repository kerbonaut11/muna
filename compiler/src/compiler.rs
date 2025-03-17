use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::{asm::{ByteCodeVec, CompileCtx}, ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement}, bytecode::ByteCode, expr::{self, Expr, InlineFunction, Op, UnaryOp}};


pub struct FuncCtx<'a> {
    pub prev:Option<*mut Self>,
    pub sub_funcs:Vec<Self>,

    scope_depth:u32,
    args:Vec<Box<str>>,
    locals:Vec<(Box<str>,u32)>,
    upvals:Vec<Box<str>>,

    func_load_indices:Vec<usize>,
    sub_func_bytecode:Vec<ByteCodeVec>,
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

            func_load_indices:vec![],
            sub_func_bytecode:vec![],
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
        //println!("searching for:{}",name);
        //println!("locals:{:?}",self.locals);
        //println!("args:{:?}",self.args);
        //println!("upvals:{:?}",self.upvals);
        //println!("");

        if let Some((id,_)) = self.locals.iter().enumerate().rev()
        .find(|(i,x)| *x.0 == *name) {
            return VarKind::Local(id as u16 + self.args.len() as u16);
        }

        if let Some((id,_)) = self.args.iter().enumerate().rev()
        .find(|(i,x)| ***x == *name) {
            return VarKind::Local(id as u16);
        }

        if let Some(id) = self.upvals.iter().enumerate()
        .find(|(i,x)| ***x == *name)
        .map(|(i,_)| i as u16) {
            return VarKind::Upval(id);
        }

        if let Some(prev) = self.get_prev_mut() {
            match prev.kind_of_ident(name) {
                VarKind::Local(_) => {
                    let id = self.upvals.len();
                    self.upvals.push(name.into());
                    return VarKind::Upval(id as u16);
                },
                _ => {}
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

        for (i,sub_func) in self.sub_funcs.iter_mut().enumerate() {
            let mut func_bytecode = ByteCodeVec::new();
            sub_func.compile(sub_func_blocks[i], comp_ctx, &mut func_bytecode);
            self.sub_func_bytecode.push(func_bytecode);

            self.func_load_indices.push(bytecode.len());
            bytecode.encode_instr(ByteCode::Closure { 
                upval_cap: sub_func.upvals.len() as u8,
                arg_count: sub_func.args.len() as u8 
            });
            bytecode.encode_int(0x0aaaffff);
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
                                VarKind::Global(name) => panic!("globals unimplimented: {}",name),
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

        for (i,offset_idx) in self.func_load_indices.iter().enumerate() {
            let offset = bytecode.len()-offset_idx-2;
            assert_eq!(bytecode.raw[*offset_idx+1],0x0aaaffff,"{:#x} {:#x}",bytecode.raw[*offset_idx+1],0x0aaaffff,);
            bytecode.raw[(*offset_idx)+1] = offset as u32;
            bytecode.append(&mut self.sub_func_bytecode[i]);
        }
    }
}

impl Expr {
    pub fn compile(
        &self,
        ctx:&mut FuncCtx,
        comp_ctx:&mut CompileCtx,
        bytecode:&mut ByteCodeVec,
    ) {
        match self {
            Expr::Ident(name) => comile_ident(&name, ctx, bytecode),

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

            Expr::Function(InlineFunction { args, block }) => {
                let mut func_bytecode = ByteCodeVec::new();
                let mut sub_func_ctx = FuncCtx::new(args);
                sub_func_ctx.prev = Some(ctx);
                sub_func_ctx.compile(block, comp_ctx, &mut func_bytecode);

                ctx.sub_func_bytecode.push(func_bytecode);
                ctx.func_load_indices.push(bytecode.len());

                bytecode.encode_instr(ByteCode::Closure { 
                    upval_cap: sub_func_ctx.upvals.len() as u8,
                    arg_count: sub_func_ctx.args.len() as u8
                });
                bytecode.encode_int(0x0aaaffff);

                for (i,upval) in sub_func_ctx.upvals.iter().enumerate() {
                    comile_ident(&upval, ctx, bytecode);
                    bytecode.encode_instr(ByteCode::BindUpval(i as u16));
                }
            }


            _ => todo!()
        }
    }
}

fn comile_ident(name:&str,ctx:&mut FuncCtx,bytecode:&mut ByteCodeVec) {
    match ctx.kind_of_ident(name) {
        VarKind::Local(id) => bytecode.encode_instr(ByteCode::Load(id+1)),
        VarKind::Global(_) => todo!(),
        VarKind::Upval(id) => bytecode.encode_instr(ByteCode::GetUpval(id)),
    }
}