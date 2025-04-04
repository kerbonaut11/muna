use std::{collections::HashMap, num::NonZeroU32, sync::Mutex};

use crate::{asm::{ByteCodeVec, CompileCtx, LabelId}, ast_gen::{Assing, AstNode, Block, Declaration, ForStatement, Function, IfElseStatement, WhileStatement}, bytecode::{ByteCode, ClosureArgs}, expr::{self, Expr, InlineFunction, Op, TableLiteral, TableLiteralIdx, UnaryOp}};


pub struct FuncCtx<'a> {
    pub prev:Option<*mut Self>,
    pub sub_funcs:Vec<Self>,

    scope_depth:u32,
    args:Vec<Box<str>>,
    locals:Vec<(Box<str>,u32)>,
    upvals:Vec<Box<str>>,

    sub_func_labels:Vec<LabelId>,
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

            sub_func_labels:vec![],
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

    fn local_count(&mut self) -> usize {
        self.locals.len() + self.args.len()
    }

    fn add_local(&mut self,name:&str) {
        let id = self.local_count();
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

    pub fn compile(
        &mut self,
        block:&[AstNode],
        comp_ctx:&mut CompileCtx,
        bytecode:&mut ByteCodeVec,
        encode_at_end:Option<ByteCode>,
        break_label:Option<LabelId>
    ) {

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
            sub_func.compile(sub_func_blocks[i], comp_ctx, &mut func_bytecode, Some(ByteCode::Ret), None);
            self.sub_func_bytecode.push(func_bytecode);

            let label = comp_ctx.new_label();
            self.sub_func_labels.push(label);
            bytecode.add_instr(ByteCode::Closure(ClosureArgs{
                label,
                upval_cap: sub_func.upvals.len() as u8,
                arg_count: sub_func.args.len() as u8 
            }));
        }

        for (sub_func_idx,sub_func) in self.sub_funcs.iter_mut().enumerate() {
            if !sub_func.upvals.is_empty() {
                bytecode.add_instr(ByteCode::Load((sub_func_idx+1) as u16));

                for (upval_idx,upval) in sub_func.upvals.iter().enumerate() {
                    let id = self.locals.iter().enumerate().rev()
                        .find(|(i,x)| *x.0 == **upval)
                        .map(|(i,_)| i as u16)
                        .unwrap();

                        bytecode.add_instr(ByteCode::Load(id+1));
                        bytecode.add_instr(ByteCode::BindUpval(upval_idx as u16));
                }

                bytecode.add_instr(ByteCode::Write((sub_func_idx+1) as u16));
            }
        }


        for node in block { match node {
            AstNode::Declaration(Declaration { lhs, rhs }) => {
                lhs.iter().for_each(|x| self.add_local(x));
                rhs.iter().for_each(|x| x.compile(self, comp_ctx,bytecode));
            }

            AstNode::Assing(Assing { lhs, rhs }) => {
                for (expr,lhs) in rhs.iter().zip(lhs) {
                    match lhs {
                        Expr::Index { table, idx } => {
                            table.compile(self, comp_ctx, bytecode);
                            idx.compile(self, comp_ctx, bytecode);
                        },
                        _ => {},
                    }
                    expr.compile(self,comp_ctx,bytecode);
                }

                for lhs in lhs.iter().rev() {
                    match lhs {
                        Expr::Ident(name) => {
                            match self.kind_of_ident(name) {
                                VarKind::Local(id) => bytecode.add_instr(ByteCode::Write(id+1)),
                                VarKind::Global(name) => panic!("globals unimplimented: {}",name),
                                VarKind::Upval(id) => bytecode.add_instr(ByteCode::SetUpval(id)),
                            }
                        }
                        
                        Expr::Index {..} => {
                            bytecode.add_instr(ByteCode::SetPop);
                        },

                        _ => unreachable!(),
                    }
                }
            }

            AstNode::Return(expr) => {
                if let Some(expr) = expr {
                    expr.compile(self, comp_ctx, bytecode);
                    bytecode.add_instr(ByteCode::Write(0));
                }
                bytecode.add_instr(ByteCode::Ret);
            }

            AstNode::If(x) => {
                let mut current = x;
                let end_label = comp_ctx.new_label();

                loop {
                    if let Some(cond) = current.cond.clone() {
                        cond.compile(self, comp_ctx, bytecode);
                        let else_label = comp_ctx.new_label();
                        bytecode.add_instr(ByteCode::JumpFalse(else_label));

                        self.up_scope();
                        self.compile(&current.block, comp_ctx, bytecode, None, break_label);
                        self.down_scope();
                        bytecode.add_instr(ByteCode::Jump(end_label));
                        bytecode.add_label(else_label);
                    } else {
                        self.up_scope();
                        self.compile(&current.block, comp_ctx, bytecode, None, break_label);
                        self.down_scope();
                    }

                    if current.next.is_none() {
                        break;
                    }

                    current = &current.next.as_ref().unwrap();
                }

                bytecode.add_label(end_label);
            }

            AstNode::While(WhileStatement { cond, block }) => {
                let end_label = comp_ctx.new_label();
                let start_label = comp_ctx.new_label();
                bytecode.add_label(start_label);

                cond.compile(self, comp_ctx, bytecode);
                bytecode.add_instr(ByteCode::JumpFalse(end_label));

                self.up_scope();
                self.compile(block, comp_ctx, bytecode, None, Some(end_label));
                self.down_scope();

                bytecode.add_instr(ByteCode::Jump(start_label));
                bytecode.add_label(end_label);
            }

            AstNode::Break => bytecode.add_instr(ByteCode::Jump(break_label.unwrap())),

            AstNode::Function(_) => {}

            _ => todo!(),
        }}

        if let Some(instr) = encode_at_end {
            bytecode.add_instr(instr);
        }

        for (i,sub_func_bytecode) in self.sub_func_bytecode.iter_mut().enumerate() {
            bytecode.add_label(self.sub_func_labels[i]);
            bytecode.append(sub_func_bytecode);
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

            Expr::NilLiteral => bytecode.add_instr(ByteCode::LoadNil),
            Expr::BoolLiteral(x) => bytecode.add_instr(if *x {ByteCode::LoadTrue} else {ByteCode::LoadFalse}),
            Expr::IntLiteral(x) => {
                bytecode.add_instr(ByteCode::LoadInt(*x));
            }
            Expr::FloatLiteral(x) => {
                bytecode.add_instr(ByteCode::LoadFloat(*x));
            }
            Expr::StrLiteral(x) => {
                let idx = comp_ctx.get_idx_of_name(&x);
                bytecode.add_instr(ByteCode::LoadStr(idx));
            } 

            Expr::Binary { op, lhs, rhs } => {
                lhs.compile(ctx,comp_ctx,bytecode);
                rhs.compile(ctx,comp_ctx,bytecode);
                bytecode.add_instr(match op {
                    Op::Add    => ByteCode::Add,
                    Op::Sub    => ByteCode::Sub,
                    Op::Mul    => ByteCode::Mul,
                    Op::Div    => ByteCode::Div,
                    Op::IDiv   => ByteCode::IDiv,
                    Op::Pow    => ByteCode::Pow,
                    Op::Mod    => ByteCode::Mod,
                    Op::Concat => ByteCode::Concat,

                    Op::Eq        => ByteCode::Eq(true),
                    Op::NotEq     => ByteCode::Eq(false),
                    Op::Less      => ByteCode::Less(true),
                    Op::Greater   => ByteCode::LessEq(false),
                    Op::LessEq    => ByteCode::LessEq(true),
                    Op::GreaterEq => ByteCode::Less(false),
                    _ => todo!()
                });
            }

            Expr::Unary { op, val } => {
                val.compile(ctx,comp_ctx,bytecode);
                bytecode.add_instr(match op {
                    UnaryOp::Neg => ByteCode::Neg,
                    UnaryOp::Not => ByteCode::Not,
                    UnaryOp::BoolNot => ByteCode::BoolNot,
                    UnaryOp::Len => ByteCode::Len,
                });
            }

            Expr::Call { function, args } => {
                bytecode.add_instr(ByteCode::LoadNil);
                for arg in args {
                    arg.compile(ctx, comp_ctx, bytecode);
                }

                function.compile(ctx, comp_ctx, bytecode);
                
                bytecode.add_instr(ByteCode::Call(args.len() as u16));
            }

            Expr::MethodCall { table, name, args } => {

                bytecode.add_instr(ByteCode::LoadNil);
                table.compile(ctx, comp_ctx, bytecode);
                for arg in args {
                    arg.compile(ctx, comp_ctx, bytecode);
                }

                bytecode.add_instr(ByteCode::Load(ctx.local_count() as u16 + 2));
                bytecode.add_instr(ByteCode::GetMethod(comp_ctx.get_idx_of_name(&name)));
                bytecode.add_instr(ByteCode::Call(args.len() as u16));
            }

            Expr::Index { table, idx } => {
                table.compile(ctx, comp_ctx, bytecode);
                idx.compile(ctx, comp_ctx, bytecode);
                bytecode.add_instr(ByteCode::Get);
            }

            Expr::TableLiteral(table) => {
                bytecode.add_instr(ByteCode::NewTable(table.arr.len() as u16));

                for expr in &table.arr {
                    expr.compile(ctx, comp_ctx, bytecode);
                    bytecode.add_instr(ByteCode::Push);
                }

                for (k,v) in &table.map {
                    bytecode.add_instr(match k {
                        TableLiteralIdx::BoolLiteral(x) => if *x {ByteCode::LoadTrue} else {ByteCode::LoadFalse},
                        TableLiteralIdx::IntLiteral(x) => ByteCode::LoadInt(*x),
                        TableLiteralIdx::FloatLiteral(x) => ByteCode::LoadFloat(*x),
                        TableLiteralIdx::StrLiteral(x) => ByteCode::LoadStr(comp_ctx.get_idx_of_name(x)),
                    });
                    v.compile(ctx, comp_ctx, bytecode);
                    bytecode.add_instr(ByteCode::Set);
                }
            }

            Expr::Function(InlineFunction { args, block }) => {
                let mut func_bytecode = ByteCodeVec::new();
                let mut sub_func_ctx = FuncCtx::new(args);
                sub_func_ctx.prev = Some(ctx);
                sub_func_ctx.compile(block, comp_ctx, &mut func_bytecode, Some(ByteCode::Ret), None);

                ctx.sub_func_bytecode.push(func_bytecode);
                let label = comp_ctx.new_label();
                ctx.sub_func_labels.push(label);

                bytecode.add_instr(ByteCode::Closure(ClosureArgs{
                    label, 
                    upval_cap: sub_func_ctx.upvals.len() as u8,
                    arg_count: sub_func_ctx.args.len() as u8
                }));

                for (i,upval) in sub_func_ctx.upvals.iter().enumerate() {
                    comile_ident(&upval, ctx, bytecode);
                    bytecode.add_instr(ByteCode::BindUpval(i as u16));
                }
            }
        }
    }
}


fn comile_ident(name:&str,ctx:&mut FuncCtx,bytecode:&mut ByteCodeVec) {
    match ctx.kind_of_ident(name) {
        VarKind::Local(id) => bytecode.add_instr(ByteCode::Load(id+1)),
        VarKind::Global(_) => todo!(),
        VarKind::Upval(id) => bytecode.add_instr(ByteCode::GetUpval(id)),
    }
}

